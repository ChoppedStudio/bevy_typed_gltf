use proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse::Parse, spanned::Spanned, Expr, ExprLit, Lit, MetaNameValue};

const GLTF: &'static str = "gltf";
const SCENE: &'static str = "scene";

#[proc_macro_derive(TypedGltf, attributes(gltf))]
pub fn typed_gltf_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let span = input.span();

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let (init, named) = match input.data {
        syn::Data::Struct(st) => {
            let (fields, named) = match st.fields {
                syn::Fields::Named(fields) => (fields.named, true),
                syn::Fields::Unnamed(fields) => (fields.unnamed, false),
                syn::Fields::Unit => {
                    return quote_spanned! {
                        span => compile_error!("Interesting try...")
                    }
                    .into_token_stream()
                    .into()
                }
            };
            let mut init = quote! {};
            for field in fields {
                for attr in field
                    .attrs
                    .iter()
                    .filter(|attribute| attribute.path().is_ident(GLTF))
                {
                    let label = attr.parse_args_with(MetaNameValue::parse).unwrap();
                    let value = match &label.path {
                        p if p.is_ident(SCENE) => {
                            match label.value {
                                Expr::Lit(ExprLit { lit: Lit::Int(index), .. }) => {
                                    quote! { gltf.scenes.get(#index).cloned().ok_or(::bevy_typed_gltf::GltfTypeError)? }
                                }
                                Expr::Lit(ExprLit { lit: Lit::Str(name), .. }) => {
                                    quote! { gltf.named_scenes.get(#name).cloned().ok_or(::bevy_typed_gltf::GltfTypeError)? }
                                }
                                _ => {
                                    return quote_spanned! {
                                        span => compile_error!("`scene` labels are either described with an index or name.")
                                    }
                                    .into_token_stream()
                                    .into()
                                }
                            }
                        },
                        _ => {
                            return quote_spanned! {
                                span => compile_error!("Only `scene` are valid labels.")
                            }
                            .into_token_stream()
                            .into()
                        }
                    };
                    if let Some(name) = &field.ident {
                        init = quote! {
                            #init
                            #name: #value
                            ,
                        };
                    } else {
                        init = quote! {
                            #init
                            #value
                            ,
                        };
                    }
                }
            }
            (init, named)
        }
        _ => {
            return quote_spanned! {
                span => compile_error!("Only structs can be used for typed gltfs.")
            }
            .into_token_stream()
            .into()
        }
    };

    let constructor = if named {
        quote! { Ok(Self { #init }) }
    } else {
        quote! { Ok(Self(#init)) }
    };

    quote! {
        impl #impl_generics ::bevy_typed_gltf::TypedGltf for #name #ty_generics #where_clause {
            fn from_gltf(gltf: &::bevy_typed_gltf::Gltf) -> Result<Self, ::bevy_typed_gltf::GltfTypeError> {
                #constructor
            }
        }
    }
    .into()
}
