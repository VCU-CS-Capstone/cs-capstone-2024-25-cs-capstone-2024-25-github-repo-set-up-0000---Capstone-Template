use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Data, DeriveInput, LitStr, Result, Variant,
};
mod kw {
    syn::custom_keyword!(id);
    syn::custom_keyword!(other_id);
}
#[derive(Debug)]
pub struct QuestionTypeAttr {
    pub red_cap_id: LitStr,
    pub red_cap_other_id: Option<LitStr>,
}
impl Parse for QuestionTypeAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut red_cap_id = None;
        let mut red_cap_other_id = None;
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::id) {
                input.parse::<kw::id>()?;
                input.parse::<syn::Token![=]>()?;
                red_cap_id = Some(input.parse()?);
            } else if lookahead.peek(kw::other_id) {
                input.parse::<kw::other_id>()?;
                input.parse::<syn::Token![=]>()?;
                red_cap_other_id = Some(input.parse()?);
            } else {
                return Err(lookahead.error());
            }
            // Check for trailing comma
            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }
        let red_cap_id = red_cap_id.ok_or_else(|| input.error("Expected an id"))?;
        Ok(Self {
            red_cap_id,
            red_cap_other_id,
        })
    }
}
#[derive(Debug)]
pub struct QuestionTypeVariant {
    pub variant: Variant,
    pub attr: QuestionTypeAttr,
}
impl QuestionTypeVariant {
    pub fn new(variant: Variant) -> Result<Self> {
        let Some(attr) = variant
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("red_cap"))
            .map(|attr| attr.parse_args::<QuestionTypeAttr>())
            .transpose()?
        else {
            return Err(syn::Error::new_spanned(
                variant,
                "expected at least one `#[red_cap(id = \"...\")]` attribute",
            ));
        };

        Ok(Self { variant, attr })
    }
    pub fn get_id_variant(&self) -> TokenStream {
        let variant = &self.variant.ident;
        let attr = &self.attr;
        let id = &attr.red_cap_id;
        quote! {
            Self::#variant => #id,
        }
    }
    pub fn get_other_id_variant(&self) -> TokenStream {
        let variant = &self.variant.ident;
        let attr = &self.attr;
        let other_id = &attr.red_cap_other_id;
        if let Some(other_id) = other_id {
            quote! {
                Self::#variant => Some(#other_id),
            }
        } else {
            quote! {
                Self::#variant => None,
            }
        }
    }
    pub fn match_id_to_variant(&self) -> TokenStream {
        let variant = &self.variant.ident;
        let attr = &self.attr;
        let id = &attr.red_cap_id;
        quote! {
            #id => Some(Self::#variant),
        }
    }
    pub fn match_other_id_to_variant(&self) -> Option<TokenStream> {
        let variant = &self.variant.ident;
        let attr = &self.attr;
        if let Some(other_id) = &attr.red_cap_other_id {
            let result = quote! {
                #other_id => Some(Self::#variant),
            };
            Some(result)
        } else {
            None
        }
    }
}

pub fn expand(input: DeriveInput) -> Result<TokenStream> {
    let DeriveInput { ident, data, .. } = input;

    let Data::Enum(data_enum) = data else {
        return Err(syn::Error::new_spanned(ident, "expected enum"));
    };

    let mut variants = vec![];
    for variant in data_enum.variants {
        variants.push(QuestionTypeVariant::new(variant)?);
    }

    let id_variants: Vec<_> = variants
        .iter()
        .map(|variant| variant.get_id_variant())
        .collect();
    let other_id_variants: Vec<_> = variants
        .iter()
        .map(|variant| variant.get_other_id_variant())
        .collect();
    let match_id_variants: Vec<_> = variants
        .iter()
        .map(|variant| variant.match_id_to_variant())
        .collect();
    let match_other_id_variants: Vec<_> = variants
        .iter()
        .filter_map(|variant| variant.match_other_id_to_variant())
        .collect();
    let result = quote! {
        impl #ident{
            pub fn red_cap_id(&self) -> &'static str {
                match self {
                    #(#id_variants)*
                }
            }
            pub fn red_cap_other_id(&self) -> Option<&'static str> {
                match self {
                    #(#other_id_variants)*
                }
            }
            pub fn find_from_red_cap_id(id: &str) -> Option<Self> {
                match id {
                    #(#match_id_variants)*
                    _ => None,
                }
            }
            pub fn find_from_red_cap_other_id(id: &str) -> Option<Self> {
                match id {
                    #(#match_other_id_variants)*
                    _ => None,
                }
            }
        }

    };

    Ok(result)
}
