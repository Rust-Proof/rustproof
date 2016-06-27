/// Prints the full type of argument provided
/// # Arguments
/// * *_* - any variable
#[allow(unused_assignments)]
pub fn print_type_of<T>(_: &T) -> () {
    let mut type_name = String::with_capacity(50);
    unsafe {
        type_name = (*super::std::intrinsics::type_name::<T>()).to_string();
    };
    println!("{:?}", type_name);
}
