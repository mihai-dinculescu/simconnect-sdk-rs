extern crate proc_macro;
use std::collections::HashMap;

use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

struct FieldInfo {
    required: bool,
    accepted_values: Vec<String>,
}

static ALLOWED_CLASS_ATTRIBUTES: Lazy<HashMap<String, FieldInfo>> = Lazy::new(|| {
    let mut map = HashMap::new();

    map.insert(
        "period".to_string(),
        FieldInfo {
            required: true,
            accepted_values: vec![
                "once".to_string(),
                "visual-frame".to_string(),
                "sim-frame".to_string(),
                "second".to_string(),
            ],
        },
    );
    map.insert(
        "condition".to_string(),
        FieldInfo {
            required: false,
            accepted_values: vec!["none".to_string(), "changed".to_string()],
        },
    );

    map
});

static ALLOWED_FIELD_ATTRIBUTES: Lazy<HashMap<String, FieldInfo>> = Lazy::new(|| {
    let mut map = HashMap::new();

    map.insert(
        "name".to_string(),
        FieldInfo {
            required: true,
            accepted_values: vec![],
        },
    );
    map.insert(
        "unit".to_string(),
        FieldInfo {
            required: true,
            accepted_values: vec![],
        },
    );

    map
});
const SUPPORTED_FIELD_TYPES: [&str; 2] = ["f64", "bool"];

/// SimConnectObject derive macro.
///
/// # Struct Arguments
/// * `period` - Required. One of `once`, `visual-frame`, `sim-frame`, `second`.
/// * `condition` - Optional. The condition of the data. Must be either `none` or `changed`. Defaults to `none`.
///
/// # Field Arguments
/// * `name` - Required. The name of the field. One from <http://www.prepar3d.com/SDKv3/LearningCenter/utilities/variables/simulation_variables.html#Simulation%20Variables>.
/// * `unit` - Required. The unit of the field.
///
/// # Example
///
/// ```rust
/// # use simconnect_sdk_derive::SimConnectObject;
///
/// #[derive(Debug, Clone, SimConnectObject)]
/// #[simconnect(period = "second")]
/// struct GpsData {
///     #[simconnect(name = "PLANE LATITUDE", unit = "degrees")]
///     lat: f64,
///     #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
///     lon: f64,
///     #[simconnect(name = "PLANE LONGITUDE", unit = "degrees")]
///     alt: f64,
/// }
/// ```
#[proc_macro_derive(SimConnectObject, attributes(simconnect))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

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

    let build_fields = fields.iter().map(parse_field);
    let request_data = request_data(&ast);

    let expanded = quote! {
        impl simconnect_sdk::SimConnectObjectExt for #name {
            fn register(client: &mut simconnect_sdk::SimConnect, id: u32) -> Result<(), simconnect_sdk::SimConnectError> {
                #(#build_fields)*

                #request_data

                Ok(())
            }
        }
        impl TryFrom<&simconnect_sdk::Object> for #name {
            type Error = simconnect_sdk::SimConnectError;

            fn try_from(value: &simconnect_sdk::Object) -> Result<Self, Self::Error> {
                value.try_transmute::<#name>()
            }
        }
    };

    expanded.into()
}

fn parse_field(f: &syn::Field) -> proc_macro2::TokenStream {
    let error_message = "expected attribute `#[simconnect(name = \"...\", unit = \"...\")]`";

    let attr = get_attribute(&f.attrs);

    match attr {
        Some(attr) => {
            let ty = &f.ty;
            let properties =
                extract_attribute_string_properties(attr, &ALLOWED_FIELD_ATTRIBUTES, error_message);

            match properties {
                Ok(properties) => {
                    let error_message_supported_types = &format!(
                        "Field type must be one of ['{}']",
                        SUPPORTED_FIELD_TYPES.join("', '")
                    );

                    match ty {
                        syn::Type::Path(syn::TypePath { path, .. }) => {
                            let path = &path.segments;
                            let path = path.iter().map(|s| &s.ident);

                            let name = properties.get("name").expect("this should never happen");
                            let unit = properties.get("unit").expect("this should never happen");

                            match path.last() {
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
                                _ => mk_err(f, error_message_supported_types),
                            }
                        }
                        _ => mk_err(f, error_message_supported_types),
                    }
                }
                Err(e) => e,
            }
        }
        None => mk_err(f, error_message),
    }
}

fn request_data(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let attr = get_attribute(&ast.attrs);
    let error_message = "expected attribute `#[simconnect(period = \"...\", condition = \"...\")]`";

    match attr {
        Some(attr) => {
            let properties =
                extract_attribute_string_properties(attr, &ALLOWED_CLASS_ATTRIBUTES, error_message);

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

                    quote! {
                        client.request_data_on_sim_object(id, #period, #condition, 0)?;
                    }
                }
                Err(e) => e,
            }
        }
        None => mk_err(ast, error_message),
    }
}

fn get_attribute(attrs: &[syn::Attribute]) -> Option<&syn::Attribute> {
    attrs
        .iter()
        .find(|&attr| attr.path.segments.len() == 1 && attr.path.segments[0].ident == "simconnect")
}

fn extract_attribute_string_properties(
    attr: &syn::Attribute,
    allowed_properties: &HashMap<String, FieldInfo>,
    error_message: &str,
) -> Result<HashMap<String, String>, proc_macro2::TokenStream> {
    let mut results = HashMap::new();

    match attr.parse_meta() {
        Ok(syn::Meta::List(nvs)) => {
            for item in nvs.nested.iter() {
                match &item {
                    syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) => {
                        match nv.path.get_ident() {
                            Some(ident) => {
                                let ident_string = ident.to_string();

                                let allowed_property = allowed_properties.get(&ident_string);

                                match allowed_property {
                                    Some(property) => {
                                        if results.contains_key(&ident_string) {
                                            // found a duplicate property name
                                            return Err(mk_err(nvs.clone(), error_message));
                                        }

                                        match &nv.lit {
                                            syn::Lit::Str(s) => {
                                                let value = s.value();

                                                if !property.accepted_values.is_empty()
                                                    && !property.accepted_values.contains(&value)
                                                {
                                                    // found an invalid value
                                                    return Err(mk_err(
                                                        nv,
                                                        &format!(
                                                            "`{ident_string}` must be one of ['{}']",
                                                            property.accepted_values.join("', '")
                                                        ),
                                                    ));
                                                }

                                                results.insert(ident_string, value);
                                            }
                                            lit => {
                                                return Err(syn::Error::new_spanned(
                                                    nv,
                                                    format!("expected string, found {lit:?}"),
                                                )
                                                .to_compile_error())
                                            }
                                        }
                                    }
                                    None => {
                                        // found an unexpected property name
                                        return Err(mk_err(nvs.clone(), error_message));
                                    }
                                }
                            }
                            None => {
                                // no ident found
                                return Err(mk_err(nvs.clone(), error_message));
                            }
                        }
                    }
                    meta => {
                        // nvc.nested[] was not k = v
                        return Err(mk_err(meta, error_message));
                    }
                }
            }

            // check that all required properties are specified
            for (field, _) in allowed_properties.iter().filter(|(_, fi)| fi.required) {
                if !results.contains_key(field) {
                    return Err(mk_err(nvs, error_message));
                }
            }
        }
        Ok(meta) => {
            // inside of #[] there was just an identifier (`#[simconnect]`)
            // or a key-value mapping (`#[simconnect = "foo"]`), neither of which are okay.

            return Err(mk_err(meta, error_message));
        }
        Err(e) => {
            return Err(e.to_compile_error());
        }
    };

    Ok(results)
}

fn mk_err<T: quote::ToTokens>(t: T, message: &str) -> proc_macro2::TokenStream {
    syn::Error::new_spanned(t, message).to_compile_error()
}
