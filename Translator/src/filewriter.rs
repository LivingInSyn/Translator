use std::fs::File;
use syn::{Ident};
use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

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
        m.insert("c_char_pntr", "char");
        m.insert("i8", "signed char");
        m.insert("c_schar", "signed char");
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
    static ref PY_USED_TYPES: Mutex<Vec<&'static str>> = {
        let mut v = Vec::new();
        v.push("Structure");
        Mutex::new(v)
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
                Some(t) => {
                    write!(file, "        (\"{}\", {}),\n", name, t).unwrap();
                    let mut pyused = PY_USED_TYPES.lock().unwrap();
                    if !pyused.contains(t){
                        pyused.push(t);
                    }                    
                },
                None => write!(file, "        (\"{}\", {}),\n", name, dtype).unwrap()
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
                Some(t) => {
                    write!(file, "        (\"{}\", {} * {}),\n", name, t, length).unwrap();
                    let mut pyused = PY_USED_TYPES.lock().unwrap();
                    if !pyused.contains(t){
                        pyused.push(t);
                    }  
                },
                None => write!(file, "        (\"{}\", {} * {}),\n", name, dtype, length).unwrap()
            } 
        }
        LanguageType::CSharp => {
            match CSHARPMAP.get(&dtype.as_ref()) {
                Some(t) => {
                    if t == &"string" {
                        write!(file, "\t\t[MarshalAs(UnmanagedType.LPArray, ArraySubType=UnmanagedType.LPStr, SizeConst={})]\n", length).unwrap();
                    } else {
                        write!(file, "\t\t[MarshalAs(UnmanagedType.ByValArray, SizeConst = {})]\n", length).unwrap();
                    }
                    write!(file, "\t\tpublic {}[] {};\n", t, name).unwrap();
                },
                None => {
                    write!(file, "\t\t[MarshalAs(UnmanagedType.ByValArray, SizeConst = {})]\n", length).unwrap();
                    write!(file, "\t\tpublic {}[] {};\n", dtype, name).unwrap();
                }                    
            }            
        }
    }
}

///adds a pointer type to the file
pub fn add_pointer(file: &mut File, ltype: LanguageType, name: Ident, dtype: Ident) {
    //change dtype to _pntr, to see if we have that
    let mut dtype_lookup_val = dtype.to_string();    
    if dtype.to_string() == "c_char" {        
        dtype_lookup_val.push_str("_pntr");
    }
    match ltype {
        LanguageType::CPP => {
            match CPPMAP.get(&dtype_lookup_val.as_ref()) {
                Some(t) => write!(file, "\t{}* {};\n", t, name).unwrap(),
                None => write!(file, "\t{}* {};\n", dtype, name).unwrap()
            }
        },
        LanguageType::Python => {
            let mut usedpntr = false;
            let mut pyused = PY_USED_TYPES.lock().unwrap();
            match PYMAP.get(&dtype_lookup_val.as_ref()) {
                Some(t) => {
                    if dtype_lookup_val != "c_char_pntr" {
                        write!(file, "        (\"{}\", POINTER({})),\n", name, t).unwrap();
                        usedpntr = true;
                    } else {
                        write!(file, "        (\"{}\", {}),\n", name, t).unwrap();
                    }
                    if !pyused.contains(t){
                        pyused.push(t);
                    }                    
                },
                None => {
                    write!(file, "        (\"{}\", POINTER({})),\n", name, dtype).unwrap();
                    usedpntr = true;
                }
            }
            if usedpntr && !pyused.contains(&"POINTER") {
                pyused.push("POINTER");
            } 
        },
        LanguageType::CSharp =>{
            match CSHARPMAP.get(&dtype_lookup_val.as_ref()) {
                //if it's a c_char or another known type
                Some(t) => {
                    //if it's a c_char pointer
                    if dtype_lookup_val == "c_char_pntr" {
                        write!(file, "\t\t[MarshalAs(UnmanagedType.LPStr)]\n").unwrap();
                        write!(file, "\t\tpublic {} {};\n", t, name).unwrap();
                    }
                    //if it's any other mapped type just do an IntPtr for now
                    //further, more complex, mapping will need to be done later
                    //todo: better mapping (https://msdn.microsoft.com/en-us/library/system.runtime.interopservices.unmanagedtype(v=vs.110).aspx)
                    //for right now, there'll be a comment describing what type it is
                    else {
                        write!(file, "\t\t//This is a '{}' type\n", name).unwrap();
                        write!(file, "\t\tpublic IntPtr {};\n", name).unwrap();
                    }
                }
                //if it's some other struct
                None => {
                    write!(file, "\t\t[MarshalAs(UnmanagedType.LPStruct)]\n").unwrap();
                    write!(file, "\t\tpublic {} {};\n", dtype, name).unwrap();
                }
            }
        }
    }
    //todo: add more args here
}

pub fn start_struct(file: &mut File, ltype: LanguageType, structname: Ident) {
    match ltype {
        LanguageType::CPP => {
            write!(file, "typedef struct {}Tag {{\n", structname).unwrap();
        },
        LanguageType::CSharp => {
            write!(file, "\t[StructLayout(LayoutKind.Sequential)]\n\tpublic struct {}\n\t{{\n", structname).unwrap();
        },
        LanguageType::Python => {
            write!(file, "class {}(Structure):\n", structname).unwrap();
            write!(file, "        _fields_ = [\n").unwrap();
        }
    }
}

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

pub fn close_file(file: &mut File, ltype: LanguageType) {
    match ltype {
        LanguageType::CPP => {
            write!(file, "\n#endif").unwrap();
        },
        LanguageType::CSharp => {
            write!(file, "\n}}").unwrap();
        },
        LanguageType::Python => {
            write!(file, "\n").unwrap();
            let used_py = PY_USED_TYPES.lock().unwrap();
            let mut ctypestring: String = String::from("from ctypes import ");
            for ctype in used_py.iter() {
                ctypestring.push_str(&format!("{},", ctype));
            }
            //remove the last comma
            ctypestring.pop();
            write!(file, "{}\n", ctypestring).unwrap();
        }
    }
}