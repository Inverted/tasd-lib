extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Expr, Field, ItemStruct, Meta};

enum MatchedType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    Boolean,
    String,
    LengthString,
    Enum(syn::Type),
    SmallEnum(syn::Type),
    BigEnum(syn::Type),
    Vec,
    DateTime,
    Packet,
    Unknown,
}

fn match_type(ty: &syn::Type) -> MatchedType {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return match segment.ident.to_string().as_str() {
                "u8" => MatchedType::U8,
                "u16" => MatchedType::U16,
                "u32" => MatchedType::U32,
                "u64" => MatchedType::U64,
                "i8" => MatchedType::I8,
                "i16" => MatchedType::I16,
                "i32" => MatchedType::I32,
                "i64" => MatchedType::I64,
                "bool" => MatchedType::Boolean,
                "String" => MatchedType::String,
                "DateTime" => MatchedType::DateTime,
                "Vec" => MatchedType::Vec,
                "Option" => MatchedType::Packet,
                _ => MatchedType::Enum(ty.clone()),
            };
        }
    }
    MatchedType::Unknown
}

struct ParsedField {
    ident: Option<Ident>,
    field_type: MatchedType,
}

impl From<Field> for ParsedField {
    fn from(value: Field) -> Self {
        let ident = value.ident;
        let mut field_type = match_type(&value.ty);
        for attr in value.attrs {
            if attr.path().is_ident("small_enum") {
                if let MatchedType::Enum(enum_name) = field_type {
                    field_type = MatchedType::SmallEnum(enum_name);
                }
            }

            if attr.path().is_ident("big_enum") {
                if let MatchedType::Enum(enum_name) = field_type {
                    field_type = MatchedType::BigEnum(enum_name);
                }
            }

            if attr.path().is_ident("has_length") {
                if let MatchedType::String = field_type {
                    field_type = MatchedType::LengthString;
                }
            }
        }

        ParsedField { ident, field_type }
    }
}

impl ParsedField {
    fn deserialize(&self) -> proc_macro2::TokenStream {
        let name = self.ident.as_ref().unwrap();
        let read_method = match &self.field_type {
            MatchedType::U8 | MatchedType::SmallEnum(_) | MatchedType::Boolean => {
                quote! { let (payload, #name) = nom::number::streaming::be_u8(payload)?; }
            }
            MatchedType::U16 | MatchedType::BigEnum(_) => {
                quote! { let (payload, #name) = nom::number::streaming::be_u16(payload)?; }
            }
            MatchedType::U32 => {
                quote! { let (payload, #name) = nom::number::streaming::be_u32(payload)?; }
            }
            MatchedType::U64 => {
                quote! { let (payload, #name) = nom::number::streaming::be_u64(payload)?; }
            }
            MatchedType::I8 => {
                quote! { let (payload, #name) = nom::number::streaming::be_i8(payload)?; }
            }
            MatchedType::I16 => {
                quote! { let (payload, #name) = nom::number::streaming::be_i16(payload)?; }
            }
            MatchedType::I32 => {
                quote! { let (payload, #name) = nom::number::streaming::be_i32(payload)?; }
            }
            MatchedType::I64 | MatchedType::DateTime => {
                quote! { let (payload, #name) = nom::number::streaming::be_i64(payload)?; }
            }
            MatchedType::String | MatchedType::Vec => {
                quote! { let (payload, #name) = nom::bytes::streaming::take(payload.len())(payload)?; }
            }
            MatchedType::LengthString => quote! {
                let (payload, #name) = nom::number::streaming::be_u8(payload)?;
                let (payload, #name) = nom::bytes::streaming::take(#name)(payload)?;
            },
            MatchedType::Packet => quote! {
                let mut #name: Option<Box::<Packet>> = None;
                if payload.len() > 0 {
                    let (payload, temp_name) = Packet::deserialize(payload).expect("Error parsing internal packet");
                    #name = Some(Box::new(temp_name));
                }
            },
            _ => quote! {},
        };
        let conversion_method = match &self.field_type {
            MatchedType::Enum(enum_name)
            | MatchedType::SmallEnum(enum_name)
            | MatchedType::BigEnum(enum_name) => quote! {
                let #name: #enum_name = #enum_name::try_from(#name).expect("Unknown enum in #enum_name");
            },
            MatchedType::String | MatchedType::LengthString => quote! {
                let #name = std::str::from_utf8(#name).expect("Invalid string").to_string();
            },
            MatchedType::DateTime => quote! {
                let #name = chrono::DateTime::from_timestamp(#name, 0).expect("Invalid timestamp");
            },
            MatchedType::Vec => quote! {
                let #name = #name.to_vec();
            },
            MatchedType::Boolean => quote! {
                let #name = #name != 0;
            },
            _ => quote! {},
        };

        quote! {
            #read_method
            #conversion_method
        }
    }

    fn serialize(&self) -> proc_macro2::TokenStream {
        let name = self.ident.as_ref().unwrap();
        match &self.field_type {
            MatchedType::U8 | MatchedType::SmallEnum(_) | MatchedType::Boolean => quote! {
                bytes.push((self.#name as u8));
            },
            MatchedType::U16 | MatchedType::BigEnum(_) => quote! {
                bytes.extend((self.#name as u16).to_be_bytes());
            },
            MatchedType::U32 => quote! {
                bytes.extend((self.#name as u32).to_be_bytes());
            },
            MatchedType::U64 => quote! {
                bytes.extend((self.#name as u64).to_be_bytes());
            },
            MatchedType::I8 => quote! {
                bytes.extend((self.#name as i8).to_be_bytes());
            },
            MatchedType::I16 => quote! {
                bytes.extend((self.#name as i16).to_be_bytes());
            },
            MatchedType::I32 => quote! {
                bytes.extend((self.#name as i32).to_be_bytes());
            },
            MatchedType::I64 => quote! {
                bytes.extend((self.#name as i64).to_be_bytes());
            },
            MatchedType::DateTime => quote! {
               bytes.extend((self.#name.timestamp() as i64).to_be_bytes());
            },
            MatchedType::LengthString => quote! {
                let length = self.#name.len() as u8;
                bytes.push(length);
                bytes.extend(self.#name.as_bytes());
            },
            MatchedType::String => quote! {
                bytes.extend(self.#name.as_bytes());
            },
            MatchedType::Vec => quote! {
                bytes.extend(&self.#name);
            },
            MatchedType::Packet => quote! {
                if let Some(inner_packet) = &self.#name {
                    bytes.extend(inner_packet.serialize());
                }
            },
            _ => quote! {},
        }
    }

    pub fn size(&self) -> proc_macro2::TokenStream {
        let name = self.ident.as_ref().unwrap();
        match &self.field_type {
            MatchedType::U8 | MatchedType::SmallEnum(_) | MatchedType::Boolean => quote! {
                plen += 1u64;
            },
            MatchedType::U16 | MatchedType::BigEnum(_) => quote! {
                plen += 2u64;
            },
            MatchedType::U32 => quote! {
                plen += 4u64;
            },
            MatchedType::U64 => quote! {
                plen += 8u64;
            },
            MatchedType::I8 => quote! {
                plen += 1u64;
            },
            MatchedType::I16 => quote! {
                plen += 2u64;
            },
            MatchedType::I32 => quote! {
                plen += 4u64;
            },
            MatchedType::I64 => quote! {
                plen += 8u64;
            },
            MatchedType::DateTime => quote! {
                plen += 8u64;
            },
            MatchedType::LengthString => quote! {
                let length = self.#name.len();
                plen += 1u64 + (length as u64);
            },
            MatchedType::String => quote! {
                plen += (self.#name.len() as u64);
            },
            MatchedType::Vec => quote! {
                plen += (self.#name.len() as u64);
            },
            MatchedType::Packet => quote! {
                if let Some(inner_packet) = &self.#name {
                    let inner_packet_size = (*inner_packet).size();
                    plen += inner_packet_size;

                    if inner_packet_size <= (u8::MAX as u64) {
                        plen += 4;
                    } else if inner_packet_size <= (u16::MAX as u64) {
                        plen += 5;
                    } else if inner_packet_size <= (u32::MAX as u64) {
                        plen += 7;
                    } else {
                        plen += 11;
                    }
                }
            },
            _ => quote! {},
        }
    }
}

#[proc_macro_derive(Packet, attributes(key, small_enum, big_enum, has_length))]
pub fn packet(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let mut key = None::<Expr>;
    for attr in input.attrs {
        if attr.path().is_ident("key") {
            if let Meta::NameValue(nv) = attr.meta {
                key = Some(nv.value);
            }
        }
    }

    let mut deserialize_fields = Vec::<proc_macro2::TokenStream>::with_capacity(input.fields.len());
    let mut serialize_fields = Vec::<proc_macro2::TokenStream>::with_capacity(input.fields.len());
    let mut field_sizes = Vec::<proc_macro2::TokenStream>::with_capacity(input.fields.len());
    let mut field_names = Vec::<Ident>::with_capacity(input.fields.len());
    for field in input.fields {
        let field: ParsedField = field.into();
        deserialize_fields.push(field.deserialize());
        serialize_fields.push(field.serialize());
        field_sizes.push(field.size());
        field_names.push(field.ident.unwrap());
    }

    let name = &input.ident;
    let expanded = quote! {
        impl tasd_lib_traits::Serializable for #name {
            fn serialize(&self) -> Vec<u8> {
                let mut bytes: Vec<u8> = Vec::new();
                bytes.extend((#key as u16).to_be_bytes());
                let plen = self.size();

                if plen <= (u8::MAX as u64) {
                    bytes.push(1);
                    bytes.extend((plen as u8).to_be_bytes());
                } else if plen <= (u16::MAX as u64) {
                    bytes.push(2);
                    bytes.extend((plen as u16).to_be_bytes());
                } else if plen <= (u32::MAX as u64) {
                    bytes.push(4);
                    bytes.extend((plen as u32).to_be_bytes());
                } else {
                    bytes.push(8);
                    bytes.extend((plen as u64).to_be_bytes());
                }

                #(#serialize_fields)*

                bytes
            }

            fn size(&self) -> u64 {
                let mut plen: u64 = 0u64;
                #(#field_sizes)*



                plen
            }

            fn deserialize(input: &[u8]) -> nom::IResult<&[u8], Self> {
                // Packet header
                let (input, key) = nom::number::streaming::be_u16(input)?;
                if key != #key {
                    let err = nom::error::Error::new(input, nom::error::ErrorKind::Tag);
                    return Err(nom::Err::Error(err))
                }
                let (input, p_exp) = nom::number::streaming::be_u8(input)?;
                let (input, p_len) = nom::bytes::streaming::take(p_exp)(input)?;
                let mut p_len_dst: [u8;8] = [0;8];
                p_len_dst[(8-p_len.len())..8].copy_from_slice(p_len);
                let p_len: u64 = u64::from_be_bytes(p_len_dst);
                let (input, payload) = nom::bytes::streaming::take(p_len)(input)?;


                // Packet fields
                #(#deserialize_fields)*
                Ok((input, #name { #(#field_names),*} ))
            }

        }
    };

    TokenStream::from(expanded)
}
