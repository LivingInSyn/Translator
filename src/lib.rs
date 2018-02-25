extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate lazy_static;

use proc_macro::TokenStream;
use proc_macro2::TokenNode;
use syn::{Data, Type, Fields};//, Ident};

mod filewriter;
use filewriter::*;


#[proc_macro_derive(Translate)]
pub fn translate(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    println!("inside of translate");
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

fn impl_translate(ast: syn::DeriveInput)  {
    // let name = ast.ident;
    
    println!("name is: {}", ast.ident);
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
    if is_reprc {
        //make sure we're matching a struct
        match ast.data {
            Data::Struct(ds) => {
                //TODO: create the file
                let mut cppfile = create_file("TestOut.h", LanguageType::CPP, ast.ident).unwrap();
                let mut pyfile = create_file("TestOut.py", LanguageType::Python, ast.ident).unwrap();
                let mut csfile = create_file("TestOut.cs", LanguageType::CSharp, ast.ident).unwrap();
                //make sure we're matching named fields 
                match ds.fields {
                    Fields::Named(fieldsnamed) => {
                        //foreach field  (https://dtolnay.github.io/syn/syn/struct.Field.html)
                        for field in fieldsnamed.named {
                            //fieldsnamed.named is of type Puncatuated<Field, Comma>
                            //field is type syn::Field. field.ident is Option<ident>, so we can print it,
                            //or save it for future use (like translating a struct :))                            
                            println!("field: {}", field.ident.unwrap());
                            //now we can find the type of the field
                            //field.ty is Enum syn::Type (https://dtolnay.github.io/syn/syn/enum.Type.html)
                            match field.ty {
                                //array
                                Type::Array(_array) => {
                                    println!("{} is an array", field.ident.unwrap())
                                },
                                //pointer
                                Type::Ptr(ptr) => {
                                    println!("{} is a ptr", field.ident.unwrap());
                                    //get the type of the pointer
                                    let fp_path = match *ptr.elem {
                                        Type::Array(array) => {
                                            // //TODO: fix this
                                            //write an array pointer to the file
                                        },
                                        Type::Ptr(ptr) => {
                                            //TODO: add a pointer to a file
                                        },
                                        Type::Path(path) => {
                                            //recursive?
                                        },
                                        _ => {println!("not supported")}
                                    };
                                },
                                //aparently fields like u16, i32, etc. are paths to their type
                                Type::Path(typepath)=> {
                                    //get the last Punctuated<PathSegment, Colon2> from 
                                    //typepath.path.segments
                                    let segment = typepath.path.segments.iter().last().unwrap();
                                    println!("type is: {}", segment.ident);
                                    println!("name is: {}", field.ident.unwrap());
                                    add_simple_type(&mut cppfile, LanguageType::CPP, field.ident.unwrap(), segment.ident);
                                    add_simple_type(&mut csfile, LanguageType::Python, field.ident.unwrap(), segment.ident);
                                    add_simple_type(&mut pyfile, LanguageType::CSharp, field.ident.unwrap(), segment.ident);
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
                                _ => {println!("{} is an unsupported type?", field.ident.unwrap())}
                            }
                        }
                    },
                    syn::Fields::Unnamed(_) => {println!("unnamed")},
                    syn::Fields::Unit => {println!("unit")}
                }
            },
        _ => {}
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
