use std::ffi::c_void;

use array::Il2cppArray;
use derive_more::derive::Debug;
use getset::Getters;
use il2cppinterop_macros::Mono;

use super::*;

use crate::{mono::runtime::*, platform::mem::{CheckedMutPointer, CheckedRefPointer}};

#[derive(Debug, Mono, Getters)]
#[repr(C)]
pub struct Il2cppDictionary<K: Sized + PartialEq, V: Sized> {
    #[base]
    base: Il2cppObject,
    buckets: *mut Il2cppArray<i32>,
    entries: *mut Il2cppArray<Il2cppDictionaryEntry<K, V>>,
    #[getset(get = "pub with_prefix")]
    count: usize,
    free_list: i32,
    free_count: i32,
    #[getset(get = "pub with_prefix")]
    version: i32,
    comparer: *mut c_void,
    keys: *mut c_void,
    values: *mut c_void,
    sync_root: *mut Il2cppObject,
}

impl<K: PartialEq + Sized, V: Sized> Il2cppDictionary<K, V> {
    /// Returns a reference to the underlaying key-value pair array
    pub fn get_entries(&self) -> &Il2cppArray<Il2cppDictionaryEntry<K, V>> {
        unsafe { &*self.entries }
    }

    /// Returns a mutable reference to the underlaying key-value pair array
    pub fn get_entries_mut(&mut self) -> &mut Il2cppArray<Il2cppDictionaryEntry<K, V>> {
        unsafe { &mut *self.entries }
    }

    pub fn get_entry(&self, index: usize) -> Option<&Il2cppDictionaryEntry<K, V>> {
        self.get_entries().get(index)
            .filter(|candidate| candidate.get_key().is_some())
    }

    pub fn get_entry_mut(&mut self, index: usize) -> Option<&mut Il2cppDictionaryEntry<K, V>> {
        self.get_entries_mut().get_mut(index).and_then(|candidate| {
            match candidate.get_key_mut().is_some() {
                true => Some(candidate),
                false => None,
            }
        })
    }    

    pub fn get_key_by_index_mut(&mut self, index: usize) -> Option<&mut K>
    where
        K: Default + Copy,
    {
        self.get_entry_mut(index)
            .map(|entry| entry.get_key_mut())
            .flatten()
    }

    pub fn get_value_by_index_mut(&mut self, index: usize) -> Option<&mut V>
    where
        V: Default + Copy,
    {
        self.get_entry_mut(index)
            .map(|entry| entry.get_value_mut())
            .flatten()
    }

    pub fn get_value_by_key_mut(&mut self, compare: K) -> Option<&mut V>
    where
        K: PartialEq + Copy,
        V: Default + Copy,
    {
        let entry_index = (0..self.count)
            .find(|&i| {
                self.get_entry_mut(i)
                    .map_or(false, |entry| entry.get_key_mut().is_some_and(|key| *key == compare))
            });

        entry_index
            .and_then(|i| self.get_entry_mut(i))
            .map(|entry| entry.get_value_mut())
            .flatten()
    }
    
}

pub struct Il2cppDictionaryIterator<'a, K, V>
where K: PartialEq + Sized + 'a, V: Sized +'a
{
    dictionary: &'a Il2cppDictionary<K, V>,
    current_index: usize
}

impl<'a, K: PartialEq + Sized + 'a, V: Sized + 'a> Iterator for Il2cppDictionaryIterator<'a, K, V> {
    type Item = &'a Il2cppDictionaryEntry<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_index < *self.dictionary.get_count() {
            true => {
                self.current_index += 1;
                self.dictionary.get_entry(self.current_index - 1) 
                    .or_else(|| self.next())
            }
            false => None,
        }  
    }
}   

impl<'a, K: PartialEq + Sized + 'a, V: Sized + 'a> IntoIterator for &'a Il2cppDictionary<K, V> {
    type Item = &'a Il2cppDictionaryEntry<K, V>;
    type IntoIter = Il2cppDictionaryIterator<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Il2cppDictionaryIterator {
            dictionary: self,
            current_index: 0
        }
    }
}

#[derive(Debug, Getters)]
#[repr(C)]
pub struct Il2cppDictionaryEntry<K: PartialEq + Sized, V: Sized> {
    #[getset(get = "pub with_prefix")]
    #[debug(skip)]
    hash_code: i32,
    #[getset(get = "pub with_prefix")]
    #[debug(skip)]
    next: i32,
    key: *mut K,
    value: *mut V,
}

impl<K: PartialEq + Sized, V: Sized> Il2cppDictionaryEntry<K, V> {
    pub fn get_key(&self) -> Option<&K> {
        //TODO: surely there is some proper way to do this but for now its what its
        // validates the pointer, at least makes sure its not dangling
        self.key.checked_ref()
    }

    pub fn get_value(&self) -> Option<&V> {         
        self.value.checked_ref()
    }

    pub fn get_key_mut(&mut self) -> Option<&mut K> {
        self.key.checked_mut()
    }

    pub fn get_value_mut(&mut self) -> Option<&mut V> {
        self.value.checked_mut()
    }
}