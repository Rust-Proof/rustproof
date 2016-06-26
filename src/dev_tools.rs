pub fn print_type_of<T>(_: &T) -> () {
    let mut s = String::with_capacity(25);
        unsafe {
                s = (*super::std::intrinsics::type_name::<T>()).to_string();
        };
    println!("{:?}", s);
}
