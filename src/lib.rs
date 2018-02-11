extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::TokenNode;
use syn::{Data, Type, Fields};

#[proc_macro_derive(Translate)]
pub fn translate(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    println!("inside of translate");
    //let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let gen = impl_translate(ast);
    
    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_translate(ast: syn::DeriveInput) -> quote::Tokens {
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
    //if it is repc, lets try and translate it
    if is_reprc {
        //make sure we're matching a struct
        match ast.data {
            Data::Struct(ds) => {
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
                                Type::Array(_array) => {println!("{} is an array", field.ident.unwrap())},
                                Type::Ptr(_ptr) => {println!("{} is a ptr", field.ident.unwrap())},
                                //aparently fields like u16, i32, etc. are paths to their type
                                Type::Path(typepath)=> {
                                    println!("{} is a path type", field.ident.unwrap());
                                    //get the last Punctuated<PathSegment, Colon2> from 
                                    //typepath.path.segments
                                    let segment = typepath.path.segments.iter().last().unwrap();
                                    println!("type is: {}", segment.ident);
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
    //empty quote! macro generates an empty quote::Tokens to return
    quote! {
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
