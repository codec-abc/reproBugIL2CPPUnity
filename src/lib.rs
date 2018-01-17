#[no_mangle]
pub extern fn print_and_change_value(x : *mut isize) {
    unsafe {
        println!("argument current value is {}", *x);
        let desired_value : isize = 42;
        println!("setting argument to {}", desired_value);
        *x = desired_value;
    }
}
