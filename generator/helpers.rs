use std::ops::Add;

use super::common::{create_file, get_good_field_name, tg_type_boolean, tg_type_float, tg_type_integer, tg_type_string};
use super::spec_types;
use convert_case::{Case, Casing};

pub async fn generate_helpers(spec: &spec_types::ApiDescription) {
    let mut data = String::new();
    for (_, obj) in spec.types.iter() {
        let good_file_name = generate_helper(obj, spec).await;
        if good_file_name == "" {
            continue;
        }
        data = data.add(format!("\nmod {good_file_name};").as_str());
    }
    create_file(String::from("helpers/mod.rs"), data);
} 

async fn generate_helper(obj: &spec_types::TypeDescription, spec: &spec_types::ApiDescription) -> String {
    let good_file_name = &obj.name.to_case(Case::Snake);
    let imports = format!("use crate::src::types::{};", obj.name);
    let (mut imports, new_fn) = create_new_fn(obj, imports).await;
    let mut helper_fn = String::new();
    for (_, method) in spec.methods.iter() {
        let helper_name = method.name.replace(&obj.name, "");
        if helper_name == method.name {
            continue;
        }
        let (temp_imports, temp_helper_fn) = create_helper_fn(obj, imports, helper_name.to_case(Case::Snake), method).await;
        imports = temp_imports;
        helper_fn = temp_helper_fn;
    }
    create_file(String::from(format!("helpers/{}.rs", good_file_name)), 
    format!("
{imports}

impl {objname} {{
{new_fn}
{helper_fn}
}}", objname=&obj.name)
    );
    return good_file_name.clone();
}

async fn create_helper_fn(obj: &spec_types::TypeDescription, mut imports: String, helper_name: String, method: &spec_types::MethodDescription) -> (String, String) {
    return (String::new(), format!(
"    /// This is a helper for {method_name}.
    pub fn {helper_name}() {{
        // body
    }}", method_name=&method.name))
}

async fn create_new_fn(obj: &spec_types::TypeDescription, mut imports: String) -> (String, String) {
    let mut attrs = String::new();
    if obj.fields.is_some() {
        let mut imported = Vec::new();
        for field in obj.fields.as_ref().unwrap().iter() {
            if field.required {
                let mut val = field.types[0].clone();
                if val == tg_type_string.to_string() {
                    val = "\"\".to_string()".to_string()
                } else if val == tg_type_boolean.to_string() {
                    val = "false".to_string()
                } else if val == tg_type_integer.to_string() {
                    val = "0".to_string()
                } else if val == tg_type_float.to_string() {
                    val = "0.0".to_string()
                } else if val.contains("Array") {
                    val = "Vec::new()".to_string()
                } else {
                    if !imported.contains(&val) {
                        imported.push(val.clone());
                        imports = imports.add(format!("\nuse crate::src::types::{};", &val).as_str());
                    }
                    val = format!("{}::new()", &val);
                }
                attrs = attrs.add(format!("\n            {}: {},", get_good_field_name(&field.name), &val).as_str());
            } else {
                attrs = attrs.add(format!("\n            {}: None,", get_good_field_name(&field.name)).as_str());
            }
        }
    }
    return (imports, format!(
"    /// This function creates an empty struct for the object {name}.
    pub fn new() -> Self {{
        Self {{{attrs}
        }}
    }}", name=&obj.name))
}