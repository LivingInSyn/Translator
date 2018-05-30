#[macro_use]
extern crate translator;

use std::os::raw::c_char;
use std::ptr;


#[repr(C)]
#[derive(Clone, Copy, Translate)]
pub struct SomeStruct {
    //pub raw_message: [i16;5],
    pub foo: i32,
    pub bar: Baz,
    pub foobar: [u8;5]
    pub test_c_char: *mut c_char
}

#[repr(C)]
#[derive(Clone, Copy, Translate)]
pub struct Baz {
    pub bob: f32
}


#[derive(Translate)]
struct __FinalizeTranslatorStruct__{}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
