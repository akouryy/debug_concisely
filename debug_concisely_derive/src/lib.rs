use itertools::Itertools;
use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Data, DeriveInput, Fields, Ident, parse_macro_input};

#[proc_macro_derive(DebugConcisely)]
pub fn derive_debug_concisely(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let body = match input.data {
        Data::Struct(ref data_struct) => {
            build_struct_or_variant_handler(false, &name, &data_struct.fields)
        }
        Data::Enum(ref data_enum) => {
            let match_arms = data_enum.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                build_struct_or_variant_handler(true, variant_name, &variant.fields)
            });
            quote! {
                match self {
                    #(#match_arms)*
                }
            }
        }
        Data::Union(_) => panic!("DebugConcisely only supports structs and enums"),
    };

    TokenStream::from(quote! {
        #[automatically_derived]
        impl std::fmt::Debug for #name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                #body
            }
        }
    })
}

fn build_struct_or_variant_handler(
    is_variant: bool,
    name: &Ident,
    fields: &Fields,
) -> proc_macro2::TokenStream {
    let name_str = name.to_string();

    let copy_alternate = quote! { let alternate = formatter.alternate(); };

    let field_vars = (0..fields.len())
        .map(|i| format_ident!("v{}", i))
        .collect_vec();

    let field_expressions = fields
        .iter()
        .zip(field_vars.iter())
        .map(|(field, var)| {
            let typename = field
                .ty
                .to_token_stream()
                .into_iter()
                .next()
                .unwrap()
                .to_string();
            match typename.as_str() {
                "Option" => quote! {
                    &match #var {
                        Some(value) =>
                            ::debug_concisely::DebugConciselyProxy(
                                if alternate {
                                    format!("Some $ {:#?}", value)
                                } else {
                                    format!("Some $ {:?}", value)
                                }
                            ),
                        None => ::debug_concisely::DebugConciselyProxy("None".to_string()),
                    }
                },
                "Vec" | "Vector" => quote! {
                    &if #var.len() == 1 {
                        if alternate {
                            ::debug_concisely::DebugConciselyProxy(format!("[{:#?}]", #var[0]))
                        } else {
                            ::debug_concisely::DebugConciselyProxy(format!("[{:?}]", #var[0]))
                        }
                    } else {
                        if alternate {
                            ::debug_concisely::DebugConciselyProxy(format!("{:#?}", #var))
                        } else {
                            ::debug_concisely::DebugConciselyProxy(format!("{:?}", #var))
                        }
                    }
                },
                _ => quote! { #var },
            }
        })
        .collect_vec();

    match fields {
        Fields::Unnamed(fields) => {
            let prefix = if is_variant {
                quote! { Self::#name(#(#field_vars),*) => }
            } else {
                quote! { let Self(#(#field_vars),*) = self; }
            };

            if fields.unnamed.len() == 1 {
                quote! {
                    #prefix {
                        #copy_alternate
                        if alternate {
                            write!(formatter, "{} $ {:#?}", #name_str, #(#field_expressions)*)
                        } else {
                            write!(formatter, "{} $ {:?}", #name_str, #(#field_expressions)*)
                        }
                    }
                }
            } else {
                quote! {
                    #prefix {
                        #copy_alternate
                        formatter.debug_tuple(#name_str)
                            #(.field(#field_expressions))*
                            .finish()
                    }
                }
            }
        }
        Fields::Named(fields) => {
            let field_names = fields.named.iter().map(|f| &f.ident).collect_vec();

            let prefix = if is_variant {
                quote! { Self::#name { #(#field_names: #field_vars),* } => }
            } else {
                quote! { let Self { #(#field_names: #field_vars),* } = self; }
            };

            if fields.named.len() == 1 {
                quote! {
                    #prefix {
                        #copy_alternate
                        if alternate {
                            write!(formatter, "{} $ {:#?}", #name_str, #(#field_expressions)*)
                        } else {
                            write!(formatter, "{} $ {:?}", #name_str, #(#field_expressions)*)
                        }
                    }
                }
            } else {
                quote! {
                    #prefix {
                        #copy_alternate
                        formatter.debug_struct(#name_str)
                            #(.field(stringify!(#field_names), #field_expressions))*
                            .finish()
                    }
                }
            }
        }
        Fields::Unit => {
            let prefix = if is_variant {
                quote! { Self::#name => }
            } else {
                quote! {}
            };

            quote! {
                #prefix {
                    #copy_alternate
                    write!(formatter, #name_str)
                }
            }
        }
    }
}
