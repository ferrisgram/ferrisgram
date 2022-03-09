use std::ops::Add;
use super::{common, spec_types};

use common::{create_file, IMP_URL};
use convert_case::{Casing, Case};

pub fn create_import_crate(obj: &spec_types::TypeDescription) -> String {
    let mut import_data = match &obj.fields {
        Some(fields) => {
            let mut import_array: Vec<String> = Vec::new();
            for field in fields {
                if !common::is_dtype_builtin(&field.types[0]) {
                    let impname = field.types[0].replace("Array of ", "");
                    if obj.name == impname {
                        continue;
                    }
                    if import_array.contains(&impname) {
                        continue;
                    }
                    // use crate::src::types::PhotoSize
                    import_array.push(impname);
                }
            }
            if import_array.len() == 0 {
                return String::new();   
            }
            let mut imptxt = format!("use {IMP_URL}::types::");
            if import_array.len() == 1 {
                return imptxt.add(import_array[0].as_str()).add(";\n");
            }
            let mut num = 0;
            for imp in import_array.iter() {
                num += 1;
                if num == 1 {
                    imptxt = imptxt.add(format!("{{{}", imp).as_str());
                    continue;
                }
                imptxt = imptxt.add(format!(", {}", imp).as_str())
            }
            imptxt.add("};\n")
        },
        None => String::new(),
    };
    match &obj.subtypes {
        Some(subtypes) => {
            if subtypes.len() == 1 {
                import_data = import_data.add(format!("use {IMP_URL}::types::{subtype}", subtype=subtypes[0]).as_str());
            } else {
                import_data = import_data.add(format!("use {IMP_URL}::types::{{").as_str());
                let mut num = 0;
                for subtype in subtypes {
                    num += 1;
                    if num == 1 {
                        import_data = import_data.add(subtype.as_str());
                        continue;
                    }
                    import_data = import_data.add(format!(", {subtype}").as_str())
                }
                import_data = import_data.add("};\n")
            }
        },
        None => {},
    }
    import_data
}

pub async fn generate_types(spec: &spec_types::ApiDescription) {
    let mut data = String::from("");
    for (_, obj) in spec.types.iter() {
        let good_tname = generate_type(obj).await;
        data = data.add(format!("\nmod {};\npub use {}::{};\n", good_tname, good_tname, obj.name).as_str());
    }
    create_file(String::from("types/mod.rs"), data);
}

async fn generate_type(obj: &spec_types::TypeDescription) -> String {
    let name = &obj.name.to_case(Case::Snake); 
    let mut data = String::from(common::WARNING_COMMENT);
    data = data.add(&create_import_crate(obj));
    data = data.add("use serde::{Deserialize, Serialize};\n\n");
    for d in obj.description.iter() {
        data = data.add(format!("\n/// {}", &d).as_str())
    }
    data = data.add(format!("\n/// <{}>", obj.href).as_str());
    if obj.subtypes.is_some() {
        data = data.add(format!("
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum {} {{{}}}
", &obj.name, generate_subtypes(obj).await).as_str())
    } else {
        data = data.add(format!("
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct {} {{{}
}}", &obj.name, generate_fields(obj).await).as_str());
    }
    // let mut data = String::new();
    create_file(String::from(format!("types/{}.rs", &name)), data);
    name.clone()
}

async fn generate_fields(obj: &spec_types::TypeDescription) -> String {
    match &obj.fields {
        Some(fields) => {
            let mut generated_fields_string = String::new();
            for field in fields.iter() {
                let mut field_type = field.types[0].clone();
                if obj.name == field_type || (obj.name == "Chat" && field_type == "Message") || obj.name == "Update" && field_type == "Message" {
                    field_type = format!("Box<{}>", field_type)
                }
                generated_fields_string = generated_fields_string.add(format!("\n    /// {}", field.description).as_str());
                if !field.required {
                    generated_fields_string = generated_fields_string.add("\n    #[serde(skip_serializing_if = \"Option::is_none\")]");
                }
                generated_fields_string = generated_fields_string.add(format!("\n    pub {name}: {dtype},",name=common::get_good_field_name(&field.name), dtype=common::get_type(field, &common::get_data_type(&field_type))).as_str())
            }
            generated_fields_string
        },
        None => String::new(),
    }
}

async fn generate_subtypes(obj: &spec_types::TypeDescription) -> String {
    match &obj.subtypes {
        Some(subtypes) => {
            let mut generated_subtype_string = String::from("");
            for subtype in subtypes.iter() {
                generated_subtype_string = generated_subtype_string.add(format!("\n    {subtype}({subtype}),").as_str());
            }
            generated_subtype_string.add("\n")
        },
        None => String::new(),
    }
}