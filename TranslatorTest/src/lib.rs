#[macro_use]
extern crate translator;
extern crate libc;

use std::os::raw::c_char;
use libc::c_void;
use std::ptr;

#[repr(C)]
#[derive(Translate)]
pub struct Baz {
    pub bob: f32,
    pub void_pntr: *mut c_void,
    pub int_pntr: *const i32,
}

#[repr(C)]
#[derive(Translate)]
pub struct SomeStruct {
    //pub raw_message: [i16;5],
    pub foo: i32,
    pub bar: Baz,
    pub baz_pntr: *const Baz,
    pub foobar: [u8;5],
    pub test_c_char: *mut c_char,
}
impl SomeStruct {
    pub fn new() -> SomeStruct {
        SomeStruct {
            foo: 1,
            bar: Baz {
                bob: 2f32,
                void_pntr: ptr::null_mut(),
                int_pntr: ptr::null(),
            },
            baz_pntr: ptr::null(),
            foobar: [0;5],
            test_c_char: ptr::null_mut()
        }
    }
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
    //****HEADER TESTS****
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
    //****PYTHON TESTS****
    #[test]
    fn check_python_void_pntr() {
        let mut f = File::open(PYFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();
        let validation = lines.contains(&"        (\"void_pntr\", c_void_p),");
        assert_eq!(validation, true);
    }

    #[test]
    fn check_python_struct_pointer() {
        let mut f = File::open(PYFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();

        assert_eq!(lines.contains(&"        (\"baz_pntr\", POINTER(Baz)),"), true);

    }

    #[test]
    fn check_python_int_pointer() {
        let mut f = File::open(PYFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();

        assert_eq!(lines.contains(&"        (\"int_pntr\", POINTER(c_int)),"), true);

    }

    #[test]
    fn check_python_imports() {
        let mut f = File::open(PYFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();
        let validation = lines.contains(&"from ctypes import Structure,c_float,c_void_p,c_int,POINTER,c_ubyte,c_char_p");
        assert_eq!(validation, true);
    }

    #[test]
    fn check_python_basic_types() {
        let mut f = File::open(PYFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();

        assert_eq!(lines.contains(&"        (\"bob\", c_float),"), true);
        assert_eq!(lines.contains(&"        (\"foo\", c_int),"), true);

    }

    #[test]
    fn check_python_struct_member() {
        let mut f = File::open(PYFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();

        assert_eq!(lines.contains(&"        (\"bar\", Baz),"), true);

    }

    #[test]
    fn check_python_array() {
        let mut f = File::open(PYFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();

        assert_eq!(lines.contains(&"        (\"foobar\", c_ubyte * 5),"), true);
    }
    //****CSHARP TESTS****
    #[test]
    fn check_cs_void_pntrs() {
        let mut f = File::open(CSHARPFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();
        //check both the comments and the code, comments are important until I get unmarshaling done 
        //in a more advanced way
        /*
        //This is a 'void_pntr' type
		public IntPtr void_pntr;
		//This is a 'int_pntr' type
		public IntPtr int_pntr;
        */
        assert_eq!(lines.contains(&"\t\t//This is a 'void_pntr' type"), true);
        assert_eq!(lines.contains(&"\t\tpublic IntPtr void_pntr;"), true);
        assert_eq!(lines.contains(&"\t\t//This is a 'int_pntr' type"), true);
        assert_eq!(lines.contains(&"\t\tpublic IntPtr int_pntr;"), true);
    }

    #[test]
    fn check_cs_structs() {
        let mut f = File::open(CSHARPFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();
        /*
        public Baz bar;
		[MarshalAs(UnmanagedType.LPStruct)]
		public Baz baz_pntr;
        */
        assert_eq!(lines.contains(&"\t\tpublic Baz bar;"), true);
        assert_eq!(lines.contains(&"\t\t[MarshalAs(UnmanagedType.LPStruct)]"), true);
        assert_eq!(lines.contains(&"\t\tpublic Baz baz_pntr;"), true);
    }

    #[test]
    fn check_cs_basic_dts() {
        let mut f = File::open(CSHARPFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();
        
        assert_eq!(lines.contains(&"\t\tpublic float bob;"), true);
        assert_eq!(lines.contains(&"\t\tpublic int foo;"), true);
    }

    #[test]
    fn check_cs_arrays() {
        let mut f = File::open(CSHARPFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();
        /*
        */
        assert_eq!(lines.contains(&"\t\t[MarshalAs(UnmanagedType.ByValArray, SizeConst = 5)]"), true);
        assert_eq!(lines.contains(&"\t\tpublic byte[] foobar;"), true);
    }
    #[test]
    fn check_cs_strings() {
        let mut f = File::open(CSHARPFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();
        /*
        */
        assert_eq!(lines.contains(&"\t\t[MarshalAs(UnmanagedType.LPStr)]"), true);
        assert_eq!(lines.contains(&"\t\tpublic string test_c_char;"), true);
    }
    #[test]
    fn check_cs_struct_decls() {
        let mut f = File::open(CSHARPFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();
        /*
        */
        //make sure we have 2 of the struct layout decls (1 for each struct)
        let num_struct_layout = lines.iter().filter(|&n| *n == "\t[StructLayout(LayoutKind.Sequential)]").count();
        assert_eq!(num_struct_layout, 2);
        //check the struct decls
        assert_eq!(lines.contains(&"\tpublic struct Baz"), true);
        assert_eq!(lines.contains(&"\tpublic struct SomeStruct"), true);
    }
    //****C++ TESTS****
    #[test]
    fn check_cpp_voids(){
        let mut f = File::open(CPPFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();
        /*
        */
        assert_eq!(lines.contains(&"\tvoid* void_pntr;"), true);
    }
    #[test]
    fn check_cpp_structs(){
        let mut f = File::open(CPPFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();
        /*
        */
        assert_eq!(lines.contains(&"\tBaz bar;"), true);
        assert_eq!(lines.contains(&"\tBaz* baz_pntr;"), true);
    }
    #[test]
    fn check_cpp_basic_dts(){
        let mut f = File::open(CPPFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();
        /*
        */
        assert_eq!(lines.contains(&"\tfloat bob;"), true);
        assert_eq!(lines.contains(&"\tint foo;"), true);
    }
    #[test]
    fn check_cpp_arrays(){
        let mut f = File::open(CPPFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();
        /*
        */
        assert_eq!(lines.contains(&"\tunsigned char foobar[5];"), true);
    }
    #[test]
    fn check_cpp_basic_strings(){
        let mut f = File::open(CPPFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();
        /*
        */
        assert_eq!(lines.contains(&"\tchar* test_c_char;"), true);
    }
    #[test]
    fn check_cpp_struct_decls(){
        let mut f = File::open(CPPFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();
        /*
        */
        assert_eq!(lines.contains(&"typedef struct BazTag {"), true);
        assert_eq!(lines.contains(&"typedef struct SomeStructTag {"), true);
        assert_eq!(lines.contains(&"} SomeStruct;"), true);
        assert_eq!(lines.contains(&"} Baz;"), true);
    }
    #[test]
    fn check_cpp_footer() {
        let mut f = File::open(CPPFILE).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        //split string into newlines
        let lines: Vec<&str> = buffer.split("\n").collect();
        /*
        */
        assert_eq!(lines.contains(&"#endif"), true);
    }
}
