use std::ops::Add;
use crate::common::{IMP_URL, create_file, get_good_field_name, WARNING_COMMENT, get_data_type, get_type, is_dtype_builtin, RequiresCustomDemarshaller, Untagged};
use crate::types::should_box_field;
use crate::spec_types;
use convert_case::{Case, Casing};

pub async fn generate_helpers(spec: &spec_types::ApiDescription) {
    let has_custom_demarshaller: Vec<&str> = RequiresCustomDemarshaller.split(" ").collect();
    let is_untagged: Vec<&str> = Untagged.split(" ").collect();
    let mut data = String::new();
    for (_, obj) in spec.types.iter() {
        let good_file_name = generate_helper(obj, spec, &has_custom_demarshaller, &is_untagged).await;
        if good_file_name == "" {
            continue;
        }
        data = data.add(format!("\nmod {good_file_name};").as_str());
    }
    create_file(String::from("helpers/mod.rs"), data);
}

async fn generate_helper(obj: &spec_types::TypeDescription, spec: &spec_types::ApiDescription, has_custom_demarshaller: &Vec<&str>, is_untagged: &Vec<&str>) -> String {
    if obj.subtypes.is_some() {
        return String::new();
    }
    let good_file_name = &obj.name.to_case(Case::Snake);
    let imports = format!("use {IMP_URL}::types::{};", obj.name);
    let (mut imports, new_fn) = create_new_fn(spec, obj, imports, has_custom_demarshaller, is_untagged).await;
    // let (mut imports, new_enum_fn) = create_new_enum_fn(obj, imports).await;
    let new_enum_fn = String::new();
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
    format!("{header}{imports}

impl {objname} {{
{new_fn}{new_enum_fn}{helper_fn}
}}", header=WARNING_COMMENT, objname=&obj.name)
    );
    return good_file_name.clone();
}

#[allow(unused_variables, unused_mut)]
async fn create_helper_fn(obj: &spec_types::TypeDescription, mut imports: String, helper_name: String, method: &spec_types::MethodDescription) -> (String, String) {
    // let mut args = String::new();
    // let mut vals = String::new();
    // if method.fields.is_some() {
    //     for field in method.fields.as_ref().unwrap() {
    //         if !field.required {
    //             continue;
    //         }
    //         let field_name = get_good_field_name(&field.name);
    //         let field_type = field.types[0].clone();
    //         let dtype = common::get_type(field, &common::get_data_type(&field_type));
    //         args = args.add(format!(", {field_name}: {dtype}").as_str());
    //         vals = vals.add(format!(", {field_name}").as_str())
    //     }
    // }
//     return (String::new(), format!(
// "    /// This is a helper for {method_name}.
//     pub fn {helper_name}({args}) {{
//         // body
//     }}", method_name=&method.name))
    return (imports, String::new())
}

#[allow(dead_code)]
async fn create_new_enum_fn(obj: &spec_types::TypeDescription, mut imports: String) -> (String, String) {
    if obj.subtypes.is_none() {
        return (imports, String::new())
    }
    imports = imports.add(format!("\nuse {IMP_URL}::types::{};", obj.subtypes.as_ref().unwrap()[0]).as_str());
    (imports, format!(
"    /// This function creates an empty struct for the enum {name}.
    pub fn new() -> Self {{
        Self::{least_subtype}({least_subtype}::new())
    }}
", name=obj.name, least_subtype=obj.subtypes.as_ref().unwrap()[0]))
}

async fn create_new_fn(spec: &spec_types::ApiDescription, obj: &spec_types::TypeDescription, mut imports: String, has_custom_demarshaller: &Vec<&str>, is_untagged: &Vec<&str>) -> (String, String) {
    // Do not check for fields to be none since there are some types that are being used as placeholders.
    // For example: CallbackGame etc.
    if obj.subtypes.is_some() {
        return (imports, String::new());
    }
    let mut function_params: Vec<String> = Vec::new();
    let mut attrs = String::new();
    if obj.fields.is_some() {
        let mut imported = Vec::new();
        let fields = obj.fields.as_ref().unwrap();
        for field in fields.iter() {
            if obj.subtype_of.is_some() && 
            !has_custom_demarshaller.contains(&obj.subtype_of.clone().unwrap()[0].as_str()) &&
            !is_untagged.contains(&obj.subtype_of.clone().unwrap()[0].as_str()) {
                if field.name == fields.first().unwrap().name {
                    continue;
                }
            }
            if field.required {
                let field_name = get_good_field_name(&field.name);
                let mut field_type = field.types[0].clone();
                let dtype = get_type(&get_data_type(&field_type), field.required, should_box_field(spec, obj.name.clone(), field_type.to_owned()));
                if !is_dtype_builtin(&field_type) {
                    field_type = field_type.replace("Array of", "");
                    println!("TODO: Implement InputFile and remove these ugly hardcodes to ignore it helpers.rs/L110");
                    if !imported.contains(&field_type) && field_type != "InputFile" {
                        imported.push(field_type.clone());
                        imports = imports.add(format!("\nuse {IMP_URL}::types::{};", field_type).as_str());
                    }
                }
                function_params.push(format!("{}: {}", field_name.clone(), dtype));
                attrs = attrs.add(format!("\n            {field_name},").as_str());
            } else {
                attrs = attrs.add(format!("\n            {}: None,", get_good_field_name(&field.name)).as_str());
            }
        }
    }
    return (imports, format!(
"    /// This function creates an empty struct for the object {name}.
    pub fn new({params}) -> Self {{
        Self {{{attrs}
        }}
    }}", name=&obj.name, params=function_params.join(", ")))
}