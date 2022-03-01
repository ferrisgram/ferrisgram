use std::ops::Add;
use super::{common, spec_types};

use common::create_file;
use convert_case::{Casing, Case};

pub fn create_import_crate(obj: &spec_types::TypeDescription) -> String {
    match &obj.fields {
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
            let mut imptxt = String::from("use crate::types::");
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
    }
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
    data = data.add(format!("
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct {} {{{}
}}", &obj.name, generate_fields(obj).await.as_str()).as_str());
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