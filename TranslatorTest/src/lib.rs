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
    pub foobar: [u8;5],
    pub test_c_char: *mut c_char
}
impl SomeStruct {
    pub fn new() -> SomeStruct {
        SomeStruct {
            foo: 1,
            bar: Baz {
                bob: 2f32
            },
            foobar: [0;5],
            test_c_char: ptr::null_mut()
        }
    }
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
    use std::path::Path;
    use std::fs::File;
    //use std::io;
    use std::io::prelude::*;
    // use std::io::Write;

    static CPPFILE: &'static str = ".\\target\\TranslateOutput\\TranslateOutput.h";
    static CSHARPFILE: &'static str = ".\\target\\TranslateOutput\\TranslateOutput.cs";
    static PYFILE: &'static str = ".\\target\\TranslateOutput\\TranslateOutput.py";

    #[test]
    fn files_exist() {
        // let mut f = File::create("foo.txt").unwrap();
        // write!(f, "test");
        assert_eq!(Path::new(CSHARPFILE).exists(), true);
        assert_eq!(Path::new(PYFILE).exists(), true);
        assert_eq!(Path::new(CPPFILE).exists(), true);
    }

    #[test]
    fn check_cpp_headers() {
        //read cpp file into string
        let mut f = File::open(CPPFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();
        //line 0 should be #include <stdbool.h>
        assert_eq!(lines[0], "#include <stdbool.h>");
        //line 1 should be #include <cstdint>
        assert_eq!(lines[1], "#include <cstdint>");
        //line 2 should be blank
        assert_eq!(lines[2], "");
        //line 3 - #ifndef TranslateOutput_H, line 4 - #define TranslateOutput_H
        assert_eq!(lines[3], "#ifndef TranslateOutput_H");
        assert_eq!(lines[4], "#define TranslateOutput_H");
        //line 5 should be blank
        assert_eq!(lines[5], "");
    }

    #[test]
    fn check_csharp_headers() {
        /*
        Expected output:

        0 using System;
        1 using System.Linq;
        2 using System.Runtime.InteropServices;
        3 
        4 namespace TranslateOutput
        5 {
        */
        //read cpp file into string
        let mut f = File::open(CSHARPFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();
        assert_eq!(lines[0], "using System;");
        assert_eq!(lines[1], "using System.Linq;");
        assert_eq!(lines[2], "using System.Runtime.InteropServices;");
        assert_eq!(lines[3], "");
        assert_eq!(lines[4], "namespace TranslateOutput");
        assert_eq!(lines[5], "{");

    }

    #[test]
    fn check_python_headers() {
        //There are currently no python headers
    }

}
