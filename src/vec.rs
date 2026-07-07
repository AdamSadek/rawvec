use std::alloc::{GlobalAlloc, Layout, LayoutError, System, alloc};

#[derive(Debug)]
pub struct RawVec<T> {
    ptr: *mut T,
    start: *mut T,
    capacity: *mut T, // pre-alloc memory to begin with
    len: usize,
}

impl<T> RawVec<T> {
    pub fn new(size: usize) -> Self {
        let initial_layout = Layout::array::<T>(size).unwrap();
        let starting_address = unsafe { alloc(initial_layout) as *mut T };
        let end = unsafe { starting_address.add(size) };
        RawVec {
            ptr: starting_address,
            start: starting_address,
            capacity: end,
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        println!("ptr: {:?}", self.ptr as usize);
        println!("cap: {:?}", self.capacity);

        if self.ptr as usize >= self.capacity as usize {
            eprintln!("max size reached");
        } else {
            unsafe {
                self.ptr.write(item);
                self.ptr = self.ptr.add(1);
                self.len += 1;
            };
        }
    }

    pub fn print(&self)
    where
        T: std::fmt::Debug,
    {
        print!("[");
        for i in 0..self.len {
            if i != 0 {
                print!(", ");
            }
            unsafe { print!("{:?}", self.start.add(i).read()) };
        }
        println!("]");
    }

    pub fn len(&self) {
        println!("{:?}", self.len);
    }
}
