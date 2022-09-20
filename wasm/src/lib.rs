extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[no_mangle]
pub extern fn test() -> i32 {
    5
}

#[no_mangle]
pub extern fn change_something() {
    let document = web_sys::window().unwrap().document().unwrap();
    let mut button = document.get_element_by_id("button").unwrap();
    button.set_inner_html("You clicked me!");
}

#[no_mangle]
pub extern fn hello_world() {
    console::log_1(&"Hello from Rust!".into());
    panic!("Just some problem, pls ignore!");
}

#[no_mangle]
pub extern fn init() {
    console_error_panic_hook::set_once();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
