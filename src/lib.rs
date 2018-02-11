extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::TokenNode;
use syn::Data;

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
            //println!("token: {}", token.span);

        }
        //println!("attr: {}", attr.tts);
    }
    if is_reprc {
        match ast.data {
            Data::Struct(ds) => {
                //i'm here (https://dtolnay.github.io/syn/syn/struct.DataStruct.html)
                //figuring out how to iterate through struct fields
                //so far everything is named?
                match ds.fields {
                    syn::Fields::Named(_) => {println!("named")},
                    syn::Fields::Unnamed(_) => {println!("unnamed")},
                    syn::Fields::Unit => {println!("unit")}
                }
                println!("it's a struct!");
            },
        _ => {}
        }
    }

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
