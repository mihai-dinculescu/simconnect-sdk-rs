use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::helpers::{get_attribute, mk_err};

#[derive(Debug, PartialEq, Eq)]
pub enum FieldType {
    Str,
    Int,
}

pub struct FieldInfo {
    pub field_type: FieldType,
    pub required: bool,
    pub accepted_values: Vec<String>,
}

pub static ALLOWED_CLASS_ATTRIBUTES: Lazy<HashMap<String, FieldInfo>> = Lazy::new(|| {
    let mut map = HashMap::new();

    map.insert(
        "period".to_string(),
        FieldInfo {
            field_type: FieldType::Str,
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
            field_type: FieldType::Str,
            required: false,
            accepted_values: vec!["none".to_string(), "changed".to_string()],
        },
    );
    map.insert(
        "interval".to_string(),
        FieldInfo {
            field_type: FieldType::Int,
            required: false,
            accepted_values: vec![],
        },
    );

    map
});

pub static ALLOWED_FIELD_ATTRIBUTES: Lazy<HashMap<String, FieldInfo>> = Lazy::new(|| {
    let mut map = HashMap::new();

    map.insert(
        "name".to_string(),
        FieldInfo {
            field_type: FieldType::Str,
            required: true,
            accepted_values: vec![],
        },
    );
    map.insert(
        "unit".to_string(),
        FieldInfo {
            field_type: FieldType::Str,
            required: false,
            accepted_values: vec![],
        },
    );

    map
});
pub const SUPPORTED_FIELD_TYPES: [&str; 3] = ["f64", "bool", "String"];

pub fn extract_attribute_properties(
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
                                            syn::Lit::Str(lit)
                                                if property.field_type == FieldType::Str =>
                                            {
                                                let value = lit.value();

                                                if !property.accepted_values.is_empty()
                                                    && !property.accepted_values.contains(&value)
                                                {
                                                    // found an invalid value
                                                    return Err(mk_err(
                                                        nv,
                                                        &format!(
                                                            r#"`{ident_string}` must be one of ["{}"]."#,
                                                            property
                                                                .accepted_values
                                                                .join(r#"", ""#)
                                                        ),
                                                    ));
                                                }

                                                results.insert(ident_string, value);
                                            }
                                            syn::Lit::Int(lit)
                                                if property.field_type == FieldType::Int =>
                                            {
                                                let value = lit.to_string();

                                                if !property.accepted_values.is_empty()
                                                    && !property.accepted_values.contains(&value)
                                                {
                                                    // found an invalid value
                                                    return Err(mk_err(
                                                        nv,
                                                        &format!(
                                                            r#"`{ident_string}` must be one of ["{}"]."#,
                                                            property
                                                                .accepted_values
                                                                .join(r#"", ""#)
                                                        ),
                                                    ));
                                                }

                                                results.insert(ident_string, value);
                                            }
                                            lit => {
                                                return Err(syn::Error::new_spanned(
                                                    lit,
                                                    format!("Expected {:?}", property.field_type,),
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

pub fn parse_field_attributes(
    field: &syn::Field,
) -> Result<(&proc_macro2::Ident, &syn::Path, HashMap<String, String>), proc_macro2::TokenStream> {
    let attr = get_attribute(&field.attrs);

    let error_message =
        "expected attribute `#[simconnect(name = \"...\", unit = \"...\")]`. `unit` is optional.";

    let name = field.ident.as_ref().expect("this should not happen");
    let ty = &field.ty;

    match attr {
        Some(attr) => {
            let properties =
                extract_attribute_properties(attr, &ALLOWED_FIELD_ATTRIBUTES, error_message);

            match properties {
                Ok(properties) => {
                    let error_message_supported_types = &format!(
                        r#"Field type must be one of ["{}"]."#,
                        SUPPORTED_FIELD_TYPES.join(r#"", ""#)
                    );

                    match ty {
                        syn::Type::Path(syn::TypePath { path, .. }) => {
                            let path_segments = &path.segments;
                            let path_idents = path_segments.iter().map(|s| &s.ident);

                            match path_idents.last() {
                                Some(value)
                                    if SUPPORTED_FIELD_TYPES
                                        .contains(&value.to_string().as_str()) =>
                                {
                                    Ok((name, path, properties))
                                }

                                _ => Err(mk_err(ty, error_message_supported_types)),
                            }
                        }
                        _ => Err(mk_err(ty, error_message_supported_types)),
                    }
                }
                Err(e) => Err(e),
            }
        }
        None => Err(mk_err(field, error_message)),
    }
}
