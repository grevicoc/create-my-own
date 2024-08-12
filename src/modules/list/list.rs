use std::ptr;

#[derive(Debug)]
pub struct List<T> {
    capacity: usize,
    size: usize,
    array: *mut T
}

impl<T> List<T> {    
    pub fn new() -> Self {
        let init_capacity = 100;
        let array = unsafe {
            let layout = std::alloc::Layout::array::<T>(init_capacity).unwrap();
            std::alloc::alloc(layout) as *mut T
        };

        return List {
            capacity: init_capacity,
            size: 0,
            array
        };
    }

    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn get(&self, index: usize) -> Option<&T>  {
        if index < self.size {
            unsafe {
                let pointer = self.array.add(index);
                Some(&*pointer)
            }
        
        //TODO: I think we need to revise this into an error state so we let user know they are trying to access problematic index.
        } else {
            None
        }
    }

    pub fn add(&mut self, object: T) {
        if self.size == self.capacity {
            self.resize();
        }

        unsafe {
            let pointer = self.array.add(self.size);
            ptr::drop_in_place(pointer);
            ptr::write(pointer, object);
        }

        self.size += 1;
    }

    pub fn delete_at(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        self.size -= 1;
        unsafe {
            let pointer = self.array.add(self.size);
            let retval = ptr::read(pointer);
            return Some(retval)
        }
    }

    fn resize(&mut self) {
        let new_capacity = self.capacity * 2;
        let new_layout = std::alloc::Layout::array::<T>(new_capacity).unwrap();
        let old_layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();

        unsafe {
            let new_array = std::alloc::alloc(new_layout) as *mut T;
            ptr::copy_nonoverlapping(self.array, new_array, self.size);

            std::alloc::dealloc(self.array as *mut u8, old_layout);
            
            self.array = new_array;
            self.capacity = new_capacity;
        };
    }
}

impl<T> Drop for List<T> {
    // TODO: I think the implementation of current drop somehow is not safe.. Because when an object being hold by >1 list and we drop one of the list it will frop the object and make other list contain an invalid object..?
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.size {
                ptr::drop_in_place(self.array.add(i));
            }

            let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
            std::alloc::dealloc(self.array as *mut u8, layout);
        }
    }
}