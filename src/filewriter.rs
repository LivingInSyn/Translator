use std::fs::File;
use syn::{Ident};
use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref CSHARPMAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("bool", "byte");
        m.insert("c_char", "sbyte");
        m.insert("i8", "sbyte");
        m.insert("c_schar", "sbyte");
        m.insert("c_char_pntr", "string");
        m.insert("u8", "byte");
        m.insert("c_uchar", "byte");
        m.insert("u16", "ushort");
        m.insert("c_ushort", "ushort");
        m.insert("i16", "short");
        m.insert("c_short", "short");
        m.insert("c_void", "IntPtr");
        m.insert("u32", "uint");
        m.insert("c_uint", "uint");
        m.insert("i32", "int");
        m.insert("c_int", "int");
        m.insert("f32", "float");
        m.insert("c_float", "float");
        m.insert("i64", "long");
        m.insert("c_long", "long");
        m.insert("c_longlong", "long");
        m.insert("c_double", "double");
        m.insert("f64", "double");
        m
    };
    static ref CPPMAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("bool", "bool");
        m.insert("c_char", "signed char");
        m.insert("i8", "signed char");
        m.insert("c_schar", "signed char");
        m.insert("c_char_pntr", "char*");
        m.insert("u8", "unsigned char");
        m.insert("c_uchar", "unsigned char");
        m.insert("u16", "unsigned short");
        m.insert("c_ushort", "unsigned short");
        m.insert("i16", "short");
        m.insert("c_short", "short");
        m.insert("c_void", "void*");
        m.insert("u32", "unsigned int");
        m.insert("c_uint", "unsigned int");
        m.insert("i32", "int");
        m.insert("c_int", "int");
        m.insert("f32", "float");
        m.insert("c_float", "float");
        m.insert("i64", "long long int");
        m.insert("c_long", "long long int");
        m.insert("c_longlong", "long long int");
        m.insert("c_ulong", "unsigned long long int");
        m.insert("c_ulonglong", "unsigned long long int");
        m.insert("c_double", "double");
        m.insert("f64", "double");
        m
    };
    static ref PYMAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("bool", "c_bool");
        m.insert("c_char", "c_byte");
        m.insert("i8", "c_byte");
        m.insert("c_schar", "c_byte");
        m.insert("c_char_pntr", "c_char_p");
        m.insert("u8", "c_ubyte");
        m.insert("c_uchar", "c_ubyte");
        m.insert("u16", "c_ushort");
        m.insert("c_ushort", "c_ushort");
        m.insert("i16", "c_short");
        m.insert("c_short", "c_short");
        m.insert("c_void", "c_void_p");
        m.insert("u32", "c_uint");
        m.insert("c_uint", "c_uint");
        m.insert("i32", "c_int");
        m.insert("c_int", "c_int");
        m.insert("f32", "c_float");
        m.insert("c_float", "c_float");
        m.insert("i64", "c_longlong");
        m.insert("c_long", "c_longlong");
        m.insert("c_longlong", "c_longlong");
        m.insert("c_ulong", "c_ulonglong");
        m.insert("c_ulonglong", "c_ulonglong");
        m.insert("c_double", "c_double");
        m.insert("f64", "c_double");
        m
    };
}

pub enum LanguageType {
    CPP,
    Python,
    CSharp,
}

pub fn create_directory() -> io::Result<()>  {
    fs::create_dir_all("./target/TranslateOutput")?;
    Ok(())
}

///Creates a file, writes the struct opening
pub fn create_file(filename: String, ltype: LanguageType, structname: Ident) -> io::Result<File> {
    let mut file = File::create(filename)?;
    //write the struct opening
    match ltype {
        LanguageType::CPP => {
            write!(file, "typedef struct {}Tag {{\n", structname)?;
        },
        LanguageType::Python => {
            write!(file, "class {}(Structure):\n", structname)?;
        }
        LanguageType::CSharp => {
            write!(file, "\t[StructLayout(LayoutKind.Sequential)]\n\tpublic struct {}\n\t{{\n", structname)?;
        }
    }
    //return the file ref
    return Ok(file);
}

///adds a simple type like i32 or other struct to the file
pub fn add_simple_type(file: &mut File, ltype: LanguageType, name: Ident, dtype: Ident) {
    match ltype {
        LanguageType::CPP => {
            match CPPMAP.get(&dtype.as_ref()) {
                Some(t) => write!(file, "\t{} {};\n", t, name).unwrap(),
                None => write!(file, "\t{} {};\n", dtype, name).unwrap()
            } 
        },
        LanguageType::Python => {
            match PYMAP.get(&dtype.as_ref()) {
                Some(t) => write!(file, "        (\"{}\", {}),\n", t, name).unwrap(),
                None => write!(file, "        (\"{}\", {}),\n", dtype, name).unwrap()
            } 
        }
        LanguageType::CSharp => {
            match CSHARPMAP.get(&dtype.as_ref()) {
                Some(t) => write!(file, "\t\tpublic {} {};\n", t, name).unwrap(),
                None => write!(file, "\t\tpublic {} {};\n", dtype, name).unwrap()
            }            
        }
    }
}

///adds an array type to the file
pub fn add_array(file: &mut File, ltype: LanguageType, name: Ident, length: u64, dtype: Ident) {
    match ltype {
        LanguageType::CPP => {
            match CPPMAP.get(&dtype.as_ref()) {
                Some(t) => write!(file, "\t{} {}[{}];\n", t, name, length).unwrap(),
                None => write!(file, "\t{} {}[{}];\n", dtype, name, length).unwrap()
            } 
        },
        LanguageType::Python => {
            match PYMAP.get(&dtype.as_ref()) {
                Some(t) => write!(file, "        (\"{}\", {} * {}),\n", t, name, length).unwrap(),
                None => write!(file, "        (\"{}\", {} * {}),\n", dtype, name, length).unwrap()
            } 
        }
        LanguageType::CSharp => {
            match CSHARPMAP.get(&dtype.as_ref()) {
                Some(t) => {
                    if t == &"string" {
                        write!(file, "\t\t[MarshalAs(UnmanagedType.LPArray, ArraySubType=UnmanagedType.LPStr, SizeConst={})]\n", length);
                    } else {
                        write!(file, "\t\t[MarshalAs(UnmanagedType.ByValArray, SizeConst = {})]\n", length);
                    }
                    write!(file, "\t\tpublic {}[] {};\n", t, name).unwrap();
                },
                None => {
                    write!(file, "\t\t[MarshalAs(UnmanagedType.ByValArray, SizeConst = {})]\n", length);
                    write!(file, "\t\tpublic {}[] {};\n", dtype, name).unwrap();
                }                    
            }            
        }
    }
}

// ///adds a pointer type to the file
// pub fn add_pointer(file: File, ltype: LanguageType, name: Ident) {
//     //todo: add more args here
// }

///nicely closes the struct
pub fn close_struct(file: &mut File, ltype: LanguageType, structname: Ident) {
    match ltype {
        LanguageType::CPP => {
            write!(file, "}} {};\n\n", structname).unwrap();
        },
        LanguageType::CSharp => {
            write!(file, "\t}}\n\n").unwrap();
        },
        LanguageType::Python => {
            write!(file, "        ]\n\n").unwrap();
        }
    }
}