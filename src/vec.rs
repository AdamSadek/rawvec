use std::alloc::{Layout, alloc};

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
        if self.ptr as usize >= self.capacity as usize {
            panic!("RawVec capacity exceeded")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        const VEC_SIZE: usize = 10;
        let mut vec: RawVec<u32> = RawVec::new(VEC_SIZE);

        for i in 1..=VEC_SIZE as u32 {
            vec.push(i);
        }
        assert_eq!(vec.len, VEC_SIZE, "expected vector size to be {VEC_SIZE}");
    }

    #[test]
    #[should_panic(expected = "RawVec capacity exceeded")]
    fn test_push_fails_on_out_of_bounds() {
        const VEC_SIZE: usize = 10;
        const LARGER_VEC_SIZE: usize = 15;
        let mut vec: RawVec<u32> = RawVec::new(VEC_SIZE);

        for i in 1..=LARGER_VEC_SIZE as u32 {
            vec.push(i);
        }
    }
}
