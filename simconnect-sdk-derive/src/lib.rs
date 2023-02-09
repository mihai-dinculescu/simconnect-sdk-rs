//! This crate provides the [`crate::SimConnectObject`] derive macro of simconnect-sdk.

extern crate proc_macro;

use std::collections::HashMap;

use fields::{extract_attribute_properties, parse_field_attributes, ALLOWED_CLASS_ATTRIBUTES};
use helpers::{get_attribute, mk_err};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod fields;
mod helpers;

/// SimConnectObject derive macro.
///
/// # Struct Arguments
/// * `period` - Required. One of `once`, `visual-frame`, `sim-frame`, `second`.
/// * `condition` - Optional. Defaults to `none`. The condition of the data. Must be either `none` or `changed`. `changed` = Data will only be sent to the client when one or more values have changed. All the variables in a data definition will be returned if just one of the values changes.
/// * `interval` - Optional. Defaults to `0`. The number of period events that should elapse between transmissions of the data. `0` means the data is transmitted every Period, `1` means that the data is transmitted every other Period, etc.
///
/// # Field Arguments
/// * `name` - Required. The name of the field. One from <https://www.prepar3d.com/SDKv5/sdk/references/variables/simulation_variables.html>.
/// * `unit` - Optional. The unit of the field. For `string`s and `bool`s it should be left out or be empty string. For numeric fields it should be one from <https://www.prepar3d.com/SDKv5/sdk/references/variables/simulation_variables.html>.
///
/// # Example
///
/// ```rust
/// # use simconnect_sdk_derive::SimConnectObject;
///
/// #[derive(Debug, Clone, SimConnectObject)]
/// #[simconnect(period = "second")]
/// struct AirplaneData {
///     #[simconnect(name = "TITLE")]
///     title: String,
///     #[simconnect(name = "CATEGORY")]
///     category: String,
///     #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
///     lat: f64,
///     #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
///     lon: f64,
///     #[simconnect(name = "PLANE ALTITUDE", unit = "feet")]
///     alt: f64,
///     #[simconnect(name = "SIM ON GROUND", unit = "bool")]
///     sim_on_ground: bool,
/// }
/// ```
#[proc_macro_derive(SimConnectObject, attributes(simconnect))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name_ident = &ast.ident;
    let packed_ident = syn::Ident::new(&format!("{name_ident}CPacked"), name_ident.span());

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        return mk_err(
            ast,
            "Unsupported field type. Only named fields are supported.",
        )
        .into();
    };

    // parse the fields and their attributes
    let mut parsed_fields = Vec::with_capacity(fields.len());
    for field in fields {
        let result = parse_field_attributes(field);

        match result {
            Ok(field) => {
                parsed_fields.push(field);
            }
            Err(e) => return e.into(),
        }
    }

    // packed struct fields
    let packed_fields = parsed_fields
        .iter()
        .map(|(ident, path, _)| build_packed_field(ident, path));
    let packed_fields_assignments = parsed_fields
        .iter()
        .map(|(ident, path, _)| build_packed_field_assignment(ident, path));

    // SC fields
    let sc_definition = parsed_fields
        .iter()
        .map(|(_, path, properties)| build_sc_definition(path, properties));
    let sc_request = build_sc_request(&ast);

    // put everything together
    let expanded = quote! {
        #[repr(C, packed)]
        struct #packed_ident {
            #(#packed_fields,)*
        }
        impl simconnect_sdk::SimConnectObjectExt for #name_ident {
            fn register(client: &mut simconnect_sdk::SimConnect, id: u32) -> Result<(), simconnect_sdk::SimConnectError> {
                #(#sc_definition)*
                #sc_request
                Ok(())
            }
        }
        impl TryFrom<&simconnect_sdk::Object> for #name_ident {
            type Error = simconnect_sdk::SimConnectError;
            fn try_from(value: &simconnect_sdk::Object) -> Result<Self, Self::Error> {
                let raw = value.try_transmute::<#name_ident, #packed_ident>()?;
                Ok(#name_ident {
                    #(#packed_fields_assignments,)*
                })
            }
        }
    };

    expanded.into()
}

fn build_packed_field(ident: &proc_macro2::Ident, path: &syn::Path) -> proc_macro2::TokenStream {
    let path_segments = &path.segments;
    let path_idents = path_segments.iter().map(|s| &s.ident);

    match path_idents.last() {
        Some(value) if value == "String" => {
            quote! {
                #ident: [std::primitive::i8; 256]
            }
        }
        _ => {
            quote! {
                #ident: #path
            }
        }
    }
}

fn build_packed_field_assignment(
    ident: &proc_macro2::Ident,
    path: &syn::Path,
) -> proc_macro2::TokenStream {
    let path_segments = &path.segments;
    let path_idents = path_segments.iter().map(|s| &s.ident);

    match path_idents.last() {
        Some(value) if value == "String" => {
            quote! {
                #ident: simconnect_sdk::fixed_c_str_to_string(&raw.#ident)
            }
        }
        _ => {
            quote! {
                #ident: raw.#ident
            }
        }
    }
}

fn build_sc_definition(
    path: &syn::Path,
    properties: &HashMap<String, String>,
) -> proc_macro2::TokenStream {
    let error_message =
        "expected attribute `#[simconnect(name = \"...\", unit = \"...\")]`. `unit` is optional.";

    let path_segments = &path.segments;
    let path_idents = path_segments.iter().map(|s| &s.ident);

    let name = properties.get("name").expect("this should never happen");
    let unit = match properties.get("unit") {
        Some(unit) => unit,
        None => "",
    };

    match path_idents.last() {
        Some(value) if value == "f64" => {
            quote! {
                client.add_to_data_definition(id, #name, #unit, simconnect_sdk::DataType::Float64)?;
            }
        }
        Some(value) if value == "bool" => {
            quote! {
                client.add_to_data_definition(id, #name, #unit, simconnect_sdk::DataType::Bool)?;
            }
        }
        Some(value) if value == "String" => {
            quote! {
                client.add_to_data_definition(id, #name, #unit, simconnect_sdk::DataType::String)?;
            }
        }
        _ => {
            // this error is already caught in `parse_field_attributes`
            mk_err(path, error_message)
        }
    }
}

fn build_sc_request(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let attr = get_attribute(&ast.attrs);
    let error_message = "expected attribute `#[simconnect(period = \"...\", condition = \"...\", interval = ...)]`. `condition` and `interval` are optional.";

    match attr {
        Some(attr) => {
            let properties =
                extract_attribute_properties(attr, &ALLOWED_CLASS_ATTRIBUTES, error_message);

            match properties {
                Ok(properties) => {
                    let period = match properties.get("period") {
                        Some(p) if p == "once" => {
                            quote! {
                                simconnect_sdk::Period::Once
                            }
                        }
                        Some(p) if p == "visual-frame" => {
                            quote! {
                                simconnect_sdk::Period::VisualFrame
                            }
                        }
                        Some(p) if p == "sim-frame" => {
                            quote! {
                                simconnect_sdk::Period::SimFrame
                            }
                        }
                        _ => {
                            quote! {
                                simconnect_sdk::Period::Second
                            }
                        }
                    };

                    let condition = match properties.get("condition") {
                        Some(c) if c == "changed" => {
                            quote! {
                                simconnect_sdk::Condition::Changed
                            }
                        }
                        _ => {
                            quote! {
                                simconnect_sdk::Condition::None
                            }
                        }
                    };

                    let interval = match properties.get("interval") {
                        Some(i) => i.parse::<u32>().unwrap_or_default(),
                        None => 0,
                    };

                    quote! {
                        client.request_data_on_sim_object(id, #period, #condition, #interval)?;
                    }
                }
                Err(e) => e,
            }
        }
        None => mk_err(ast, error_message),
    }
}
