use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, spanned::Spanned, Error, Expr, Lit, Type, Ident};

// call as such: panic_span!(something.span(), "Error message"); in a function that returns
// a TokenStream
macro_rules! panic_span {
    ($span:expr, $message:literal) => {
        return Error::new($span, $message).to_compile_error().into()
    };
}

#[proc_macro_attribute]
pub fn char_enum(_input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(annotated_item as DeriveInput);
    if item.generics.lifetimes().count() != 0
        || item.generics.type_params().count() != 0
        || item.generics.const_params().count() != 0 {
            panic_span!(item.generics.span(), "Generics are not supported");
    }
    match item.data {
        Data::Enum(enum_data) => {
            let vis = item.vis;
            let ident = item.ident;
            let variants = enum_data.variants;

            let mut data = vec![];

            for variant in variants {
                if !variant.fields.is_empty() {
                    panic_span!(variant.fields.span(), "Fields are not supported");
                }
                match variant.discriminant {
                    Some((_, expr)) => {
                        if let Expr::Lit(literal) = expr {
                            if let Lit::Char(chr) = literal.lit {
                                data.push((variant.attrs, variant.ident, chr.token()));
                            } else {
                                panic_span!(literal.span(), "Expected character literal");
                            }
                        } else {
                            panic_span!(expr.span(), "Expected character literal");
                        }
                    },
                    None => panic_span!(variant.span(), "Must include = '<char>'")
                }
            }

            let top_level_attrs = item.attrs;

            let identifiers = data.iter().map(|(attrs, id, _)| quote!{
                #(
                    #attrs
                )*
                #id,
            });
            let char_to_ident = data.iter().map(|(_, id, literal)| quote!{#literal => #ident::#id});
            let ident_to_char = data.iter().map(|(_, id, literal)| quote!{#ident::#id => #literal});
            let has_encode_decode = Ident::new(&(ident.to_string() + "__HasEncodeDecode__"), ident.span());

            TokenStream::from(quote!{
                #(
                    #top_level_attrs
                )*
                #vis enum #ident {
                    #( #identifiers )*
                }

                #[automatically_derived]
                #[allow(non_camel_case_types)]
                #vis trait #has_encode_decode {
                    fn decode(chr: char) -> #ident;
                    fn encode(&self) -> char;
                }

                #[automatically_derived]
                impl #has_encode_decode for #ident {
                    fn decode(chr: char) -> #ident {
                        match chr {
                            #( #char_to_ident, )*
                            _ => panic!("Unknown character `{}`", chr)
                        }
                    }

                    fn encode(&self) -> char {
                        match self {
                            #( #ident_to_char, )*
                        }
                    }
                }
            })
        },
        _ => panic!("char_enum can only be applied to enums")
    }
}

#[proc_macro_attribute]
pub fn data_enum(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    //let input: Vec<TokenTree> = input.into_iter().collect();
    let input = parse_macro_input!(input as Type);
    let item = parse_macro_input!(annotated_item as DeriveInput);

    /*if input.len() > 1 {
        panic_span!(input.last().unwrap().span().into(), "Too many parameters");
    } else if input.len() == 0 {
        panic_span!(item.ident.span(), "Expected exactly one type parameter");
    }*/

    if item.generics.lifetimes().count() != 0
        || item.generics.type_params().count() != 0
        || item.generics.const_params().count() != 0 {
            panic_span!(item.generics.span(), "Generics are not supported");
    }
    match item.data {
        Data::Enum(enum_data) => {
            let vis = item.vis;
            let ident = item.ident;
            let variants = enum_data.variants;

            let mut data = vec![];

            for variant in variants {
                if !variant.fields.is_empty() {
                    panic_span!(variant.fields.span(), "Fields are not supported");
                }
                match variant.discriminant {
                    Some((_, expr)) => {
                        data.push((variant.attrs, variant.ident, expr));
                    },
                    None => panic_span!(variant.span(), "Must include = <VALUE>")
                }
            }

            let top_level_attrs = item.attrs;

            let identifiers = data.iter().map(|(attrs, id, _)| quote!{
                #(
                    #attrs
                )*
                #id,
            });
            let ident_to_value = data.iter().map(|(_, id, expr)| quote!{#ident::#id => #expr});
            let has_value = Ident::new(&(ident.to_string() + "__HasValue__"), ident.span());

            TokenStream::from(quote!{
                #(
                    #top_level_attrs
                )*
                #vis enum #ident {
                    #( #identifiers )*
                }

                #[automatically_derived]
                #[allow(non_camel_case_types)]
                #vis trait #has_value {
                    fn value(&self) -> #input;
                }

                #[automatically_derived]
                impl #has_value for #ident {
                    fn value(&self) -> #input {
                        match self {
                            #( #ident_to_value, )*
                        }
                    }
                }
            })
        },
        _ => panic!("data_enum can only be applied to enums")
    }
}

