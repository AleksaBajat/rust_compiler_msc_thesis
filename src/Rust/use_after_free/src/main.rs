fn main() {
    let mut a: Box<i32> = Box::new(5);
    drop(a);
    *a = 6;
}
