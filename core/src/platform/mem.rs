
use std::{ops::{Deref, DerefMut, Range}, sync::Arc};

use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use udbg::{error::UDbgError, memory::MemoryPage, pe::{MEM_COMMIT, MEM_IMAGE, MEM_MAPPED, MEM_PRIVATE, PAGE_EXECUTE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE, PAGE_EXECUTE_WRITECOPY, PAGE_GUARD, PAGE_NOACCESS, PAGE_READONLY, PAGE_READWRITE, PAGE_WRITECOPY}};

pub type IEngine = dyn udbg::target::UDbgEngine;
pub type ITarget = dyn udbg::target::UDbgTarget;

pub type Result<T> = core::result::Result<T, Error>;

static ENGINE: Lazy<Mutex<Option<Engine>>> = Lazy::new(|| Mutex::new(None));
static TARGET: Mutex<Option<Target>> = Mutex::new(None);

pub fn initialize() -> Result<()> {
    let mut engine = ENGINE.lock();
    let mut target = TARGET.lock();

    if engine.is_some() {
        return Err(Error::AlreadyInitialized);
    }

    *engine = Some(Engine::default());
    *target = Some(Target::new(engine.as_mut().unwrap().open_self()?));

    Ok(())
}

pub fn uninitialize() -> Result<()> {
    let mut engine = ENGINE.lock();
    let mut target = TARGET.lock();

    if engine.is_none() {
        return Err(Error::NotInitialized);
    }

    *engine = None;
    *target = None;
    

    Ok(())
}

pub fn aob_query(pattern_str: &str, mapped: bool, readable: bool, writable: bool, executable: bool, address_range: Option<Range<usize>>,) -> Result<Vec<usize>> {
    let target = get_target()?;
    let pages = filter_pages(&target.collect_memory_info(), mapped, readable, writable, executable, address_range,);
    let pattern = parse_pattern(pattern_str);
    let skip_table = build_skip_table_with_wildcards(&pattern);

    if pages.is_empty() {
        return Ok(vec![]);
    }

    let addresses: Vec<usize> = pages
        .par_iter()
        .filter_map(|page| {
            match read_bytes(page.base, page.size) {
                Ok(buffer) => Some(
                    find_all_occurrences_with_wildcards(&buffer, &pattern, &skip_table)
                        .into_iter()
                        .map(|offset| page.base + offset)
                        .collect::<Vec<_>>()
                ),
                Err(_) => None,
            }
        })
        .flatten()
        .collect();

    Ok(addresses)
}

pub fn write_memory(address: usize, buffer: &[u8]) -> Result<usize> {
    get_target()?
        .write_memory(address, buffer)
        .filter(|&bytes| bytes != 0)
        .ok_or(Error::FailedToWriteProcessMemory)
}

pub fn read_bytes(address: usize, size: usize) -> Result<Vec<u8>> {
    let mut buffer: Vec<u8> = vec![0u8; size];
    let len = get_target()?
        .read_memory(address, &mut buffer)
        .map(|b|b.len()).filter(|&len| len != 0)
        .ok_or(Error::FailedtoReadProcessMemory)?;
    buffer.resize(len, 0);
    Ok(buffer)
}

pub fn bytes_to_pattern(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{:02X}", byte))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn filter_pages(pages: &[MemoryPage], mapped: bool, readable: bool, writable: bool, executable: bool, address_range: Option<Range<usize>>) -> Vec<MemoryPage> {
    pages
        .par_iter()
        .filter_map(|page| {
            #[cfg(target_pointer_width = "64")]
            let max_space: usize = 0x00007FFFFFFFFFFF;
            #[cfg(target_pointer_width = "32")]
            let max_space: usize = 0xFFFFFFFF;

            let is_valid = page.state == MEM_COMMIT
                && page.base < max_space
                && (page.protect & PAGE_GUARD) == 0
                && (page.protect & PAGE_NOACCESS) == 0
                && (page.type_ == MEM_PRIVATE || page.type_ == MEM_IMAGE)
                && (!mapped || page.type_ == MEM_MAPPED)
                && address_range.as_ref().map_or(true, |range| range.contains(&page.base));

            if !is_valid {
                return None;
            }

            let is_readable = (page.protect & PAGE_READONLY) > 0;
            let is_writable = (page.protect & PAGE_READWRITE) > 0
                || (page.protect & PAGE_WRITECOPY) > 0
                || (page.protect & PAGE_EXECUTE_READWRITE) > 0
                || (page.protect & PAGE_EXECUTE_WRITECOPY) > 0;
            let is_executable = (page.protect & PAGE_EXECUTE) > 0
                || (page.protect & PAGE_EXECUTE_READ) > 0
                || (page.protect & PAGE_EXECUTE_READWRITE) > 0
                || (page.protect & PAGE_EXECUTE_WRITECOPY) > 0;

            if (is_readable && readable) || (is_writable && writable) || (is_executable && executable) {
                Some(page.clone())
            } else {
                None
            }
        })
        .collect()
}

pub fn get_target() -> Result<Target> {
    let current_target = TARGET.lock();
    match current_target.as_ref() {
        None => Err(Error::NotInitialized),
        Some(target) => Ok(target.clone()),
    }
}

fn parse_pattern(pattern_str: &str) -> Vec<Option<u8>> {
    pattern_str
        .split_whitespace()
        .map(|s| {
            match s == "??" {
                true => None,
                false => Some(u8::from_str_radix(s, 16).expect("Invalid byte in pattern"))
            }
        })
        .collect()
}

fn build_skip_table_with_wildcards(pattern: &[Option<u8>]) -> [usize; 256] {
    let m = pattern.len();
    let mut skip_table = [m; 256];

    for i in 0..m - 1 {
        if let Some(byte) = pattern[i] {
            skip_table[byte as usize] = m - i - 1;
            continue;
        }

        (0..256).for_each(|b| { skip_table[b] = skip_table[b].min(m - i - 1); });
    }

    skip_table
}

fn find_all_occurrences_with_wildcards(
    haystack: &[u8],
    pattern: &[Option<u8>],
    skip_table: &[usize; 256],
) -> Vec<usize> {
    let m = pattern.len();
    let n = haystack.len();
    
    if m == 0 || n < m {
        return Vec::new();
    }
    
    let chunk_size = 1024 * 1024;
    let overlap = m - 1;

    let chunk_indices: Vec<(usize, usize)> = (0..n)
        .step_by(chunk_size)
        .map(|start| {
            let end = (start + chunk_size + overlap).min(n);
            (start, end)
        })
        .collect();

    let occurrences: Vec<usize> = chunk_indices
        .par_iter()
        .flat_map(|&(start, end)| {
            let chunk = &haystack[start..end];
            let mut local_occurrences = Vec::new();
            let mut i = 0;
            let chunk_len = chunk.len();

            while i <= chunk_len - m {
                let mut j = (m - 1) as isize;
                
                unsafe {
                    while j >= 0 {
                        let hay_byte = *chunk.get_unchecked(i + j as usize);
                        match pattern.get_unchecked(j as usize) {
                            Some(pat_byte) if *pat_byte != hay_byte => break,
                            _ => {}
                        }
                        j -= 1;
                    }
                }
                
                if j < 0 {
                    local_occurrences.push(start + i);
                    i += 1;
                } else {
                    let next_byte = unsafe { *chunk.get_unchecked(i + m - 1) };
                    let shift = skip_table[next_byte as usize].max(1);
                    i += shift;
                }
            }

            local_occurrences
        })
        .collect();

    occurrences
}

pub struct Engine {
    ptr: *mut IEngine
}

unsafe impl Send for Engine {}
unsafe impl Sync for Engine {}

impl Deref for Engine {
    type Target = IEngine;

    fn deref(&self) -> &Self::Target {
        unsafe { &mut *(self.ptr as *mut IEngine) }
    }
}

impl DerefMut for Engine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.ptr as *mut IEngine) }
    }
}

impl Default for Engine {
    fn default() -> Self {
        let engine = udbg::os::DefaultEngine::default();
        let leaked = Box::into_raw(Box::new(engine));

        Engine {
            ptr: leaked,
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {

        if self.ptr.is_null() == false {
            let _ = unsafe {
                Box::from_raw(self.ptr)
            };
        }
    }
}

#[derive(Clone)]
pub struct Target {
    inner: Arc<ITarget>
}

unsafe impl Send for Target {}
unsafe impl Sync for Target {}

impl Target {
    pub fn new(inner: Arc<ITarget>) -> Self {
        Target { inner }
    }
}

impl Deref for Target {
    type Target = ITarget;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("already-initialized")]
    AlreadyInitialized,
    #[error("not-initialized")]
    NotInitialized,
    #[error("failed-to-write-process-memory")]
    FailedToWriteProcessMemory,
    #[error("failed-to-read-process-memory")]
    FailedtoReadProcessMemory,
    #[error("invalid-target-handle")]
    InvalidTargetHandle,
    #[error("udbg-engine-error")]
    EngineError(#[from] UDbgError)
}

/// Extension trait for creating aob patterns out of structs
pub trait AsArrayOfBytePattern {
    fn as_array_of_byte_pattern(&self) -> String;
}

impl<T: Sized> AsArrayOfBytePattern for T {
    fn as_array_of_byte_pattern(&self) -> String {
        bytes_to_pattern(unsafe { std::slice::from_raw_parts(self as *const T as *const u8, size_of::<T>()) })
    }
}

/// Extension trait for checked references on raw pointers
pub trait CheckedRefPointer<'a, T: ?Sized + 'a> {
    /// Attempts to read single byte on this location before dereferencing, uses os apis.
    fn checked_ref(self) -> Option<&'a T>;
}

/// Extension trait for checked references on raw pointers
pub trait CheckedMutPointer<'a, T: ?Sized + 'a> {
    /// Attempts to read single byte on this location before dereferencing, uses os apis.
    fn checked_mut(self) -> Option<&'a mut T>;
}

impl<'a, T: ?Sized + 'a> CheckedRefPointer<'a, T> for *const T {
    fn checked_ref(self) -> Option<&'a T> {
        match read_bytes(&self as *const _ as usize, size_of::<u8>()).is_ok() {
            true => unsafe { self.as_ref() },
            false => None,
        }
    }
}

impl<'a, T: ?Sized + 'a> CheckedMutPointer<'a, T> for *mut T {
    fn checked_mut(self) -> Option<&'a mut T> {
        match read_bytes(&self as *const _ as usize, size_of::<u8>()).is_ok() {
            true => unsafe { self.as_mut() },
            false => None,
        }
    }
}

impl<'a, T: ?Sized + 'a> CheckedRefPointer<'a, T> for *mut T {
    fn checked_ref(self) -> Option<&'a T> {
        match read_bytes(&self as *const _ as usize, size_of::<u8>()).is_ok() {
            true => unsafe { self.as_ref() },
            false => None,
        }
    }
}