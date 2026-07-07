use rawvec::vec::RawVec;

fn main() {
    let mut array: RawVec<i32> = RawVec::new(10);
    array.print();
    for i in 1..=10 {
        array.push(i);
    }
    array.print();
}
