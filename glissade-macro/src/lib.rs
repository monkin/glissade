use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, GenericParam};

#[derive(Debug)]
enum Error {
    CantDeriveForEnum,
    CantDeriveForUnion,
}

impl From<Error> for TokenStream {
    fn from(error: Error) -> TokenStream {
        match error {
            Error::CantDeriveForEnum => {
                quote! {
                    compile_error!("Mix cannot be derived for enums");
                }
            }
            Error::CantDeriveForUnion => {
                quote! {
                    compile_error!("Mix cannot be derived for unions");
                }
            }
        }
        .into()
    }
}

/// Derive the `Mix` trait for a struct.
/// It interpolates each field of the struct with the `Mix` trait.
#[proc_macro_derive(Mix)]
pub fn mix_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let fields = match input.data {
        syn::Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let fields_mix = fields
                    .named
                    .iter()
                    .map(|field| {
                        let name = &field.ident.as_ref().unwrap();
                        quote! {
                            #name: self.#name.mix(other.#name, t)
                        }
                    })
                    .collect::<Vec<_>>();

                quote! {
                    {
                      #(#fields_mix),*
                    }
                }
            }
            Fields::Unnamed(ref fields) => {
                let fields_mix = (0..fields.unnamed.len())
                    .map(syn::Index::from)
                    .map(|i| {
                        quote! {
                            self.#i.mix(other.#i, t)
                        }
                    })
                    .collect::<Vec<_>>();

                quote! {
                    (
                        #(#fields_mix),*
                    )
                }
            }
            Fields::Unit => TokenStream::default().into(),
        },
        syn::Data::Enum(_) => {
            return Error::CantDeriveForEnum.into();
        }
        syn::Data::Union(_) => {
            return Error::CantDeriveForUnion.into();
        }
    };

    let generic_params = input.generics.params;
    let generic_names = if generic_params.is_empty() {
        quote! {}
    } else {
        let names = generic_params
            .iter()
            .map(|param| match param {
                GenericParam::Type(t) => {
                    let name = t.ident.clone();
                    quote! { #name }
                }
                GenericParam::Lifetime(l) => {
                    let lifetime = l.lifetime.clone();
                    quote! { #lifetime }
                }
                GenericParam::Const(c) => {
                    let name = c.ident.clone();
                    quote! { #name }
                }
            })
            .collect::<Vec<_>>();
        quote! {
            <#(#names),*>
        }
    };

    let generic_params = if generic_params.is_empty() {
        quote! {}
    } else {
        quote! {
            <#generic_params>
        }
    };

    let where_clause = input.generics.where_clause;

    (quote! {
        impl #generic_params glissade::Mix for #name #generic_names #where_clause {
            fn mix(self, other: Self, t: f32) -> Self {
                Self #fields
            }
        }
    })
    .into()
}
