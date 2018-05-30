extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate lazy_static;

use proc_macro::TokenStream;
use proc_macro2::TokenNode;
use syn::{Data, Type, Fields, Expr};//, Ident};
use std::sync::{Mutex, RwLock};
use std::io::Write;
use std::fs::File;

mod filewriter;
use filewriter::*;

lazy_static! {
    static ref EXPORT_NAME: RwLock<String> = {
        RwLock::new("TranslateOutput".to_string())
    };
    static ref CLOSED_FLAG: Mutex<bool> = {
        Mutex::new(false)
    };
    static ref CPPFILE: Mutex<File> = {
        let en = EXPORT_NAME.read().unwrap();
        let mut file = File::create(format!("./target/TranslateOutput/{}.h", *en)).unwrap();
        //cpp file headers
        write!(file, "#include <stdbool.h>\n#include <cstdint>\n\n").unwrap();
        write!(file, "#ifndef {}_H\n", *en).unwrap();
        write!(file, "#define {}_H\n\n", *en).unwrap();

        Mutex::new(file)
    };
    static ref CSFILE: Mutex<File> = {
        let en = EXPORT_NAME.read().unwrap();
        let mut file = File::create(format!("./target/TranslateOutput/{}.cs", *en)).unwrap();
        //c# file header
        write!(file, "using System;\n").unwrap();
        write!(file, "using System.Linq;\n").unwrap();
        write!(file, "using System.Runtime.InteropServices;\n\n").unwrap();
        //c# namespace def
        write!(file, "namespace {}\n{{\n", *en).unwrap();

        Mutex::new(file)
    };
    static ref PYFILE: Mutex<File> = {
        let en = EXPORT_NAME.read().unwrap();
        let file = File::create(format!("./target/TranslateOutput/{}.py", *en)).unwrap();
        Mutex::new(file)
    };
}

#[proc_macro_derive(Translate)]
pub fn translate(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    //println!("inside of translate");
    //let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    impl_translate(ast);
    
    //return empty tokenstream
    let empty_tokens = quote!{
    };
    empty_tokens.parse().unwrap()
}

fn get_array_len(arr: syn::TypeArray) -> Option<u64> {
    match arr.len {
        Expr::Lit(l) => {
            match l.lit {
                syn::Lit::Int(int) => {
                    Some(int.value())
                }
                _ => {
                    None
                }
            }
        },
        _ => None
    }
}

fn check_magic_struct(id: &syn::Ident) -> bool {
    if id.to_string() == "__FinalizeTranslatorStruct__" {
        let mut cppfile = CPPFILE.lock().unwrap();
        let mut pyfile = PYFILE.lock().unwrap();
        let mut csfile = CSFILE.lock().unwrap();
        close_file(&mut *cppfile, LanguageType::CPP);
        close_file(&mut *pyfile, LanguageType::Python);
        close_file(&mut *csfile, LanguageType::CSharp);
        let mut cf = CLOSED_FLAG.lock().unwrap();
        *cf = true;
        return true;
    }
    return false;
}

fn check_finalized() {
    let cf = CLOSED_FLAG.lock().unwrap();
    if *cf {
        panic!("translator files are closed, please make sure that __FinalizeTranslatorStruct__ is the last struct translated");
    }
} 

fn impl_translate(ast: syn::DeriveInput)  {
    //check if the files are closed
    check_finalized();
    
    //println!("name is: {}", ast.ident);
    let mut is_reprc: bool = false;
    //determine if this struct is repr c
    for attr in ast.attrs {
        for token in attr.tts.into_iter() {
            match token.kind {
                TokenNode::Group(_, ts) => {
                    for t in ts.into_iter() {
                        match t.kind {
                            TokenNode::Term(t) => {
                                is_reprc =  t.as_str() == "C";
                            },
                            _ => {}
                        }
                    }
                }
                _ => {}
            };
        }
    }
    //if it's not repr c, all we're going to do is check if it's the magic struct
    if !is_reprc {
        check_magic_struct(&ast.ident);
    }
    else {
        if let Data::Struct(ds) = ast.data {
            //check if it's the magic struct (close files)
            if check_magic_struct(&ast.ident) {
                return;
            }
            //TODO: create the directory
            let _ = create_directory();
            //create the files/unlock them if already made (lazy static initilization)
            let mut cppfile = CPPFILE.lock().unwrap();
            let mut pyfile = PYFILE.lock().unwrap();
            let mut csfile = CSFILE.lock().unwrap();
            //start the struct
            start_struct(&mut *cppfile, LanguageType::CPP, ast.ident);
            start_struct(&mut *pyfile, LanguageType::Python, ast.ident);
            start_struct(&mut *csfile, LanguageType::CSharp, ast.ident);

            //make sure we're matching named fields
            if let Fields::Named(fieldsnamed) = ds.fields {
                //foreach field  (https://dtolnay.github.io/syn/syn/struct.Field.html)
                for field in fieldsnamed.named {
                    //fieldsnamed.named is of type Puncatuated<Field, Comma>
                    //field is type syn::Field. field.ident is Option<ident>, so we can print it,
                    //or save it for future use (like translating a struct :))                            
                    //println!("field: {}", field.ident.unwrap());
                    //now we can find the type of the field
                    //field.ty is Enum syn::Type (https://dtolnay.github.io/syn/syn/enum.Type.html)
                    match field.ty {
                        //array
                        Type::Array(array) => {
                            //println!("{} is an array", field.ident.unwrap());
                            //check the len type
                            if let Some(len) = get_array_len(array.clone()) {
                                //println!("length val is: {}", len);
                                match *array.elem {
                                    //scaffold handling 2d arrays
                                    Type::Array(_a) => {
                                        //todo: support 2d arrays
                                        panic!("2+D Arrays not currently implemented")
                                    },
                                    //an array of pointers
                                    //TODO: update to n-pointers later, right now only handle single pointers
                                    Type::Ptr(_p) => {
                                        //todo: support array of pointers
                                        panic!("Pointer Arrays not currently implemented")
                                    },
                                    Type::Path(p) => {
                                        //println!("array name is {}", field.ident.unwrap());
                                        //println!("array type is: {}", p.path.segments.iter().last().unwrap().ident);
                                        add_array(&mut cppfile, LanguageType::CPP, 
                                                    field.ident.unwrap(), len,
                                                    p.path.segments.iter().last().unwrap().ident);
                                        add_array(&mut csfile, LanguageType::CSharp, 
                                                    field.ident.unwrap(), len,
                                                    p.path.segments.iter().last().unwrap().ident);
                                        add_array(&mut pyfile, LanguageType::Python, 
                                                    field.ident.unwrap(), len,
                                                    p.path.segments.iter().last().unwrap().ident);
                                    },
                                    _ => {}
                                }
                            }
                        },
                        //pointer
                        Type::Ptr(ptr) => {
                            //println!("{} is a ptr", field.ident.unwrap());
                            //get the type of the pointer
                            match *ptr.elem {
                                Type::Array(_array) => {
                                    //todo: support pointer to an array
                                    panic!("pointer to an array not currently implemented");
                                },
                                Type::Ptr(_ptr) => {
                                    //TODO: add a pointer to a file
                                    panic!("double+ pointers not currently implemented");
                                },
                                Type::Path(p) => {
                                    //println!("ptr name is {}", field.ident.unwrap());
                                    //println!("ptr type is: {}", p.path.segments.iter().last().unwrap().ident);
                                    add_pointer(&mut cppfile, LanguageType::CPP, field.ident.unwrap(), p.path.segments.iter().last().unwrap().ident);
                                    add_pointer(&mut pyfile, LanguageType::Python, field.ident.unwrap(), p.path.segments.iter().last().unwrap().ident);
                                    add_pointer(&mut csfile, LanguageType::CSharp, field.ident.unwrap(), p.path.segments.iter().last().unwrap().ident);
                                },
                                _ => {
                                    panic!("pointer type not implemented");
                                }
                            };
                        },
                        //aparently fields like u16, i32, etc. are paths to their type
                        Type::Path(typepath)=> {
                            //get the last Punctuated<PathSegment, Colon2> from 
                            //typepath.path.segments
                            let segment = typepath.path.segments.iter().last().unwrap();
                            //println!("type is: {}", segment.ident);
                            //println!("name is: {}", field.ident.unwrap());
                            add_simple_type(&mut cppfile, LanguageType::CPP, field.ident.unwrap(), segment.ident);
                            add_simple_type(&mut csfile, LanguageType::CSharp, field.ident.unwrap(), segment.ident);
                            add_simple_type(&mut pyfile, LanguageType::Python, field.ident.unwrap(), segment.ident);
                        },
                                                        
                        /* I'm not going to support these types yet */
                        // =====
                        // Type::Slice(_) => {println!("slice")},
                        // Type::Reference(_) => {println!("reference")},
                        // Type::BareFn(_) => {println!("barefn")},
                        // Type::Never(_) => {println!("never")},
                        // Type::Tuple(_TypeTuple)=> {println!("tuple")},                                
                        // Type::TraitObject(_TypeTraitObject)=> {println!("trait obj")},
                        // Type::ImplTrait(_TypeImplTrait)=> {println!("type impl trait")},
                        // Type::Paren(_TypeParen)=> {println!("paren")},
                        // Type::Group(_TypeGroup)=> {println!("group")},
                        // Type::Infer(_TypeInfer)=> {println!("infer")},
                        // Type::Macro(_TypeMacro)=> {println!("macro")},
                        //Type::Verbatim(typeverbatim) => {println!("{} is a verbatim", field.ident.unwrap())},
                        // =====
                        
                        //TODO: for unsupported types, throw a warning and don't translate the struct 
                        _ => {println!("{} is an unsupported type", field.ident.unwrap())}
                    }
                }
            }
            //close the struct
            close_struct(&mut *cppfile, LanguageType::CPP, ast.ident);
            close_struct(&mut *pyfile, LanguageType::Python, ast.ident);
            close_struct(&mut *csfile, LanguageType::CSharp, ast.ident);
        }
    }
    
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
