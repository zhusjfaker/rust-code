use wasm_bindgen::prelude::*;
mod clib;

#[wasm_bindgen]
pub extern "C" fn test_add(x: i32) -> i32 {
    x + 2
}

#[wasm_bindgen]
pub fn greet(num: i32) -> i32 {
    unsafe {
        return test_add(num);
    }
}

#[cfg(test)]
mod tests {
    use crate::clib::root::test_add;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn clib_test() {
        unsafe {
            let a = test_add(2);
            println!("{} is res", a);
            assert_eq!(a, 4);
        }
    }
}



