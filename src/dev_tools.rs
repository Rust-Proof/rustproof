//pub fn print_type_of<T>(x: &T) -> () {
pub fn print_type_of<T> () -> () {
    let type_name;
    unsafe {
        type_name = (*super::std::intrinsics::type_name::<T>()).to_string();
    };
    println!("{:?}", type_name);
}
