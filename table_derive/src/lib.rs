extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Table)]
pub fn parse(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source)
        .expect("failed to parse rust syntax");
    let gen = impl_parse(&ast);
    gen.parse()
        .expect("failed to serialize into rust syntax")
}

fn impl_parse(ast: &syn::DeriveInput) -> quote::Tokens {
    use syn::{Body, VariantData};

    let variants = match ast.body {
        Body::Struct(VariantData::Struct(ref vars)) => vars,
        _ => panic!("#[derive(Parse)] is only defined for braced structs"),
    };

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let ident = &ast.ident;
    let tys = variants.iter()
        .filter(|field| field.ident.as_ref().unwrap() != "buffer")
        .map(|field| &field.ty);
    let parse = variants.iter()
        .filter(|field| field.ident.as_ref().unwrap() != "buffer")
        .map(|field| {
            let ident = field.ident.as_ref().unwrap();
            let ty = &field.ty;

            quote! {
                let #ident = buf.parse::<#ty>()?;
            }
        });
    let build = variants
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .map(|id| quote! { #id : #id });

    quote! {
        impl #impl_generics SizedTable for #ident #ty_generics #where_clause {
            fn size() -> usize {
                #(<#tys>::size())+*
            }
        }

        impl<'tbl> Table<'tbl> for #ident #ty_generics #where_clause {
            fn parse(buffer: &'tbl [u8]) -> Result<Self> {
                if buffer.len() < Self::size() {
                    return Err(Error::UnexpectedEof)
                }

                let mut buf = buffer;
                #(#parse)*

                Ok(#ident {
                    // buffer: buffer, (if it exists)
                    // var1: va1,
                    #(#build),*
                })
            }
        }
    }
}