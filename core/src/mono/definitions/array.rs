use std::{ffi::c_int, ptr};

use il2cppinterop_macros::Mono;

use crate::mono::runtime::*;

#[derive(Debug, Mono)]
#[repr(C)]
pub struct Il2cppArray<T: Sized> {
    #[base]
    object: Il2cppObject,           // 0x00
    bounds: *mut Il2cppArrayBounds, // 0x08
    capacity: usize,                // 0x0C
    items: [T; 65355],              // 0x10
}

impl<T> Il2cppArray<T> {
    /// Gets first subsequent pointer that points to the item in this array
    pub fn get_data_head_mut(&mut self) -> *mut T {
        &mut self.items as *mut T
    }

    /// Checks if the array is multi-dimensional.
    pub fn is_multi_dimensional(&self) -> bool {
        self.get_num_dimensions() > 1
    }

    /// Retrieves the number of dimensions of the array.
    pub fn get_num_dimensions(&self) -> usize {
            // If bounds is null, it's a single-dimensional array.
            if self.bounds.is_null() {
                1
            } else {
                // Retrieve the rank from the associated Il2CppClass.
                self.get_instance_info().rank as usize
            }
    }

    /// Returns the total number of elements in the array.
    pub fn total_elements(&self) -> usize {
        if self.is_multi_dimensional() {
            let dimensions = self.get_dimensions();
            dimensions.iter().map(|dim| dim.length).product()
        } else {
            self.capacity
        }
    }

    /// Retrieves the dimension information as a vector of `Il2cppArrayBounds`.
    pub fn get_dimensions(&self) -> Vec<Il2cppArrayBounds> {
        let num_dimensions = self.get_num_dimensions();
        let mut dimensions = Vec::with_capacity(num_dimensions);
        if num_dimensions == 1 {
            // Single-dimensional array.
            dimensions.push(Il2cppArrayBounds {
                length: self.capacity,
                lower_bound: 0,
            });
        } else {
            // Multi-dimensional array.
            unsafe {
                let bounds_slice = std::slice::from_raw_parts(self.bounds, num_dimensions);
                dimensions.extend_from_slice(bounds_slice);
            }
        }
        dimensions
    }

    /// Gets a reference to the element at the given index.
    pub fn get(&self, index: usize) -> Option<&T> {
        match index < self.total_elements() {
            true => self.items.get(index),
            false => None,
        }
    }

    /// Gets a mutable reference to the element at the given index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match index < self.total_elements() {
            true => self.items.get_mut(index),
            false => None,
        }
    }

    /// Checks if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.total_elements() == 0
    }

    /// Fills the array with the given value.
    pub fn fill(&mut self, value: T)
    where
        T: Copy,
    {
        let total_elements = self.total_elements();
        unsafe {
            let data_ptr = self.get_data_head_mut();
            for i in 0..total_elements {
                *data_ptr.add(i) = value;
            }
        }
    }

    /// Inserts elements into the array starting at the given linear index by overwriting existing elements.
    pub fn insert(&mut self, index: usize, elements: &[T])
    where
        T: Copy,
    { 
        let total_elements = self.total_elements();
        if index >= total_elements {
            return;
        }
        let max_insert_size = total_elements - index;
        let insert_size = elements.len().min(max_insert_size);

        unsafe {
            let dest_ptr = self.get_data_head_mut().add(index);
            ptr::copy_nonoverlapping(elements.as_ptr(), dest_ptr, insert_size);
        }
    }

    /// Removes (resets) the element at the specified linear index.
    pub fn remove_at(&mut self, index: usize)
    where
        T: Default,
    {
        if index >= self.total_elements() {
            return;
        }
        unsafe {
            *self.get_data_head_mut().add(index) = T::default()
        }
    }

    /// Removes (resets) a range of elements starting from the specified linear index.
    /// The count specifies how many elements to reset.
    pub fn remove_range(&mut self, index: usize, count: usize)
    where
        T: Default,
    {
        let total_elements = self.total_elements();
        if index >= total_elements {
            return;
        }
        let max_remove_size = total_elements - index;
        let remove_size = count.min(max_remove_size);

        unsafe {
            let data_ptr = self.get_data_head_mut().add(index);
            for i in 0..remove_size {
                *data_ptr.add(i) = T::default();
            }
        }
    }

    /// Removes (resets) all elements from the array.
    pub fn remove_all(&mut self)
    where
        T: Default,
    {
        let total_elements = self.total_elements();
        unsafe {
            let data_ptr = self.get_data_head_mut();
            for i in 0..total_elements {
                *data_ptr.add(i) = T::default();
            }
        }
    }
}

/// Iterator over the elements of an `Il2cppArray`.
pub struct Il2cppArrayIterator<'a, T> {
    array: &'a Il2cppArray<T>,
    total_elements: usize,
    current_index: usize,
}

impl<'a, T> Iterator for Il2cppArrayIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_index < self.array.total_elements() {
            true => {
                self.current_index += 1;
                self.array.get(self.current_index - 1) 
                    .or_else(|| self.next())
            }
            false => None,
        }        
    }
}

impl<'a, T> IntoIterator for &'a Il2cppArray<T> {
    type Item = &'a T;
    type IntoIter = Il2cppArrayIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Il2cppArrayIterator {
            array: self,
            total_elements: self.total_elements(),
            current_index: 0,
        }
    }
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct Il2cppArrayBounds {
    pub length: usize,
    pub lower_bound: c_int,
}