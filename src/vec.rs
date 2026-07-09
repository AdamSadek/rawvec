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
        for i in 0..self.len() {
            if i != 0 {
                print!(", ");
            }
            let element_ptr = unsafe { &*self.start.add(i) };
            print!("{:?}",  element_ptr);
        }
        println!("]");
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_u32_exact_vec_size() {
        const VEC_SIZE: usize = 10;
        let mut vec: RawVec<u32> = RawVec::new(VEC_SIZE);

        for i in 1..=VEC_SIZE as u32 {
            vec.push(i);
        }
        vec.print();
        assert_eq!(vec.len(), VEC_SIZE, "expected vector size to be {VEC_SIZE}");
    }

    #[test]
    fn test_push_usize_one_less_than_size() {
        const VEC_SIZE: usize = 10;
        const VEC_SIZE_MINUS_ONE: usize = VEC_SIZE - 1;
        let mut vec: RawVec<usize> = RawVec::new(VEC_SIZE);

        for i in 1..=VEC_SIZE_MINUS_ONE {
            vec.push(i);
        }
        assert_eq!(vec.len(), VEC_SIZE_MINUS_ONE);
        assert!(
            vec.len() < VEC_SIZE,
            "expected vector length {:?} to be less than {VEC_SIZE}",
            vec.len()
        );
    }

    #[test]
    fn test_push_string_exact_vec_size() {
        const VEC_SIZE: usize = 10;
        let mut vec: RawVec<String> = RawVec::new(VEC_SIZE);

        for _ in 1..=VEC_SIZE {
            vec.push("test".to_string());
        }
        vec.print();
        assert_eq!(vec.len(), VEC_SIZE, "expected vector size to be {VEC_SIZE}");
    }

    #[test]
    fn test_push_varied_strings() {
        let inputs = ["", "a", "hello world", "🦀", "line1\nline2", "  spaces  "];
        let mut vec: RawVec<String> = RawVec::new(inputs.len());

        for s in inputs {
            vec.push(s.to_string());
        }
        vec.print();
        assert_eq!(vec.len(), inputs.len());
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

    // revisit
    //
    // #[test]
    // fn test_push_with_large_intgers() {
    //     const VEC_SIZE: usize = 10;
    //     let mut vec: RawVec<usize> = RawVec::new(VEC_SIZE);

    //     for i in usize::MAX..=usize::MAX {
    //         vec.push(i);
    //     }
    //     vec.print();
    //     assert_eq!(
    //         vec.len(),
    //         VEC_SIZE,
    //         "expected vector length {:?}, to be vector total size {VEC_SIZE}",
    //         vec.len()
    //     )
    // }
}
