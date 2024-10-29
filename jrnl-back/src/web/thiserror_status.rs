use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{parse_macro_input, Data, DeriveInput, LitInt, Meta, Path, Variant};

// stripped version of https://docs.rs/axum_thiserror/0.1.0/src/axum_thiserror/lib.rs.html#1-119
#[proc_macro_derive(ErrorStatus, attributes(status))]
#[allow(clippy::missing_panics_doc)]
pub fn derive_error_status(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);

    let Data::Enum(data) = ast.data else {
        panic!(
            "#[derive(ErrorStatus)] is only available for enums, other types are not supported."
        )
    };

    let cases = data.variants.iter()
        .map(impl_enum_variant)
        .collect::<Punctuated<TokenStream, Comma>>();

    let enum_ident = ast.ident;
    (quote! {
        impl axum::response::IntoResponse for #enum_ident {
            fn into_response(self) -> axum::response::Response {
                match self {
                    #cases
                }
            }
        }
    }).into()
}

fn impl_enum_variant(variant: &Variant) -> TokenStream {
    let status_code = find_status_code(variant);
    let ident = &variant.ident;

    let enum_syntax = if variant.fields.is_empty() {
        quote!(#ident)
    } else if variant.fields.iter().any(|f| f.ident.is_none()) {
        quote!(#ident( .. ))
    } else {
        quote!(#ident { .. })
    };
    
    let quoted_ident = format!("\"{ident}\"");
    quote! {
        Self::#enum_syntax => {
            let m = format!("{}", self);
            let status: u16 = #status_code.into();
            
            let mut r = axum::response::IntoResponse::into_response(axum::Json(serde_json::json!({ "status": status, "msg": m, "code": #quoted_ident })));
            *r.status_mut() = #status_code;
            
            r
        }
    }
}

fn find_status_code(input: &Variant) -> TokenStream {
    let Some(Meta::List(l)) = input.attrs
        .iter()
        .find(|attr| attr.path().is_ident("status"))
        .map(|attr| &attr.meta)
    else {
        return quote_spanned! { input.span() => compile_error!("Invalid #[status] syntax") };
    };

    if let Ok(number) = l.parse_args::<LitInt>() {
        return quote! { axum::http::StatusCode::from_u16(#number as u16).unwrap() };
    }

    if let Ok(expr) = l.parse_args::<Path>() {
        return quote! { #expr };
    }

    quote_spanned! { l.span() => compile_error!("Invalid #[status] syntax") }
}