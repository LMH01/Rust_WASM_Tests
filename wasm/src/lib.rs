use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
