use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Data, DeriveInput, LitStr, Result, Variant,
};
mod kw {
    syn::custom_keyword!(other);
}
pub struct MyAttr {
    pub name: Option<LitStr>,
    pub is_default: bool,
}
impl Parse for MyAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            Ok(Self {
                name: Some(input.parse()?),
                is_default: false,
            })
        } else if input.peek(kw::other) {
            input.parse::<kw::other>()?;
            return Ok(Self {
                name: None,
                is_default: true,
            });
        } else {
            return Err(input.error("expected a string or `other`"));
        }
    }
}
pub struct EnumWithOtherVariant {
    pub variant: Variant,
    pub attr: MyAttr,
}
impl EnumWithOtherVariant {
    fn new(name: Variant) -> Result<Self> {
        let Some(attr) = name
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("my_attr"))
            .map(|attr| attr.parse_args::<MyAttr>())
            .transpose()?
        else {
            return Err(syn::Error::new_spanned(
                name,
                "expected at least one `#[my_attr(\"...\"))]` attribute",
            ));
        };

        Ok(Self {
            variant: name,
            attr,
        })
    }

    pub fn match_from_str(&self) -> TokenStream {
        let name = &self.variant.ident;
        let attr = &self.attr;
        if attr.is_default {
            quote! {
                _ => Self::#name(value.to_owned()),
            }
        } else {
            let value = attr.name.as_ref().unwrap();
            quote! {
                #value => Self::#name,
            }
        }
    }
    pub fn match_from_string(&self) -> TokenStream {
        let name = &self.variant.ident;
        let attr = &self.attr;
        if attr.is_default {
            quote! {
                _ => Self::#name(value),
            }
        } else {
            let value = attr.name.as_ref().unwrap();
            quote! {
                #value => Self::#name,
            }
        }
    }

    pub fn match_display(&self) -> TokenStream {
        let name = &self.variant.ident;
        let attr = &self.attr;
        if attr.is_default {
            quote! {
                Self::#name(value) => write!(f, "{}", value),
            }
        } else {
            let value = attr.name.as_ref().unwrap();
            quote! {
                Self::#name => write!(f, "{}", #value),
            }
        }
    }
    pub fn match_as_ref(&self) -> TokenStream {
        let name = &self.variant.ident;
        let attr = &self.attr;
        if attr.is_default {
            quote! {
                Self::#name(d) => d,
            }
        } else {
            let value = attr.name.as_ref().unwrap();
            quote! {
                Self::#name => #value,
            }
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
        variants.push(EnumWithOtherVariant::new(variant)?);
    }
    let match_from_str: Vec<_> = variants
        .iter()
        .map(|variant| variant.match_from_str())
        .collect();
    let match_from_string: Vec<_> = variants
        .iter()
        .map(|variant| variant.match_from_string())
        .collect();
    let match_display: Vec<_> = variants
        .iter()
        .map(|variant| variant.match_display())
        .collect();
    let match_as_ref: Vec<_> = variants
        .iter()
        .map(|variant| variant.match_as_ref())
        .collect();
    let result = quote! {
        impl std::convert::From<&str> for #ident {
            fn from(value: &str) -> Self {
                match value {
                    #(
                        #match_from_str
                    )*
                }
            }
        }
        impl std::convert::From<String> for #ident {
            fn from(value: String) -> Self {
                match value.as_str() {
                    #(
                        #match_from_string
                    )*
                }
            }
        }
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(
                        #match_display
                    )*
                }
            }
        }
        impl std::convert::AsRef <str> for #ident {
            fn as_ref(&self) -> &str {
                match self {
                    #(
                        #match_as_ref
                    )*
                }
            }
        }
        impl serde::Serialize for #ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let value = self.as_ref();
                serializer.serialize_str(value)
            }
        }
        impl<'de> serde::Deserialize<'de> for #ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Ok(Self::from(value))
            }
        }
        impl sqlx::Type<::sqlx::Postgres> for #ident {
            fn type_info() -> ::sqlx::postgres::PgTypeInfo {
                sqlx::postgres::PgTypeInfo::with_name("TEXT")
            }
        }
        impl sqlx::postgres::PgHasArrayType for #ident {
            fn array_type_info() -> ::sqlx::postgres::PgTypeInfo {
                sqlx::postgres::PgTypeInfo::array_of("TEXT")
            }
        }
        impl<'q, DB: sqlx::Database> sqlx::encode::Encode<'q, DB> for #ident
        where
            String: sqlx::encode::Encode<'q, DB>,
        {
            fn encode_by_ref(
                &self,
                buf: &mut <DB as sqlx::Database>::ArgumentBuffer<'q>,
            ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
                let val: &str = self.as_ref();
                let val: String = val.to_owned();
                val.encode(buf)
            }
            fn size_hint(&self) -> usize {
                self.as_ref().size_hint()
            }
        }
        impl<'r> sqlx::decode::Decode<'r, ::sqlx::postgres::Postgres> for #ident {
            fn decode(
                value: sqlx::postgres::PgValueRef<'r>,
            ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
                let value =
                    <&'r str as sqlx::decode::Decode<'r, sqlx::postgres::Postgres>>::decode(value)?;
                Ok(Self::from(value))
            }
        }
    };
    Ok(result)
}
