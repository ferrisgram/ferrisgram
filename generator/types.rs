use std::ops::Add;
use super::{common, spec_types};
use std::collections::HashMap;

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
    let mut bound_types_map: HashMap<String, String> = HashMap::new();
    let mut scfv_map: HashMap<String, HashMap<String, String>> = HashMap::new();
    for (_, obj) in spec.types.iter() {
        if !obj.name.contains("ResultCached") {
            if obj.subtype_of.is_some() && obj.fields.is_some() {
                // status
                let cf = &obj.fields.as_ref().unwrap()[0];
                // ChatMember
                let stof = &obj.subtype_of.as_ref().unwrap()[0];
                match scfv_map.get(stof) {
                    Some(x) => {
                        let mut x = x.clone();
                        x.insert((&obj.name).clone(), get_subtype_cfv(&cf.description));
                        scfv_map.remove(stof);
                        scfv_map.insert(stof.to_string(), x);
                    },
                    None => {
                        let mut x = HashMap::new();
                        x.insert((&obj.name).clone(), get_subtype_cfv(&cf.description));
                        scfv_map.insert(stof.to_string(), x);
                    },
                };
                // 0th index field is the common field 
                bound_types_map.insert(stof.to_string(), cf.name.to_string());
            }   
        }
    }
    for (_, obj) in spec.types.iter() {
        let good_tname = generate_type(spec, obj, &bound_types_map, &scfv_map);
        data = data.add(format!("\nmod {};\npub use {}::{};\n", good_tname, good_tname, obj.name).as_str());
    }
    create_file(String::from("types/mod.rs"), data);
}

fn generate_type(spec: &spec_types::ApiDescription, obj: &spec_types::TypeDescription, btm: &HashMap<String, String>, sm: &HashMap<String, HashMap<String, String>>) -> String {
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
#[serde(tag = \"{common_field}\")]
pub enum {} {{{}}}
", &obj.name, generate_subtypes(obj, sm), common_field={
    let cf = btm.get(&obj.name);
    if cf.is_none() {
        "None"
    } else {
        cf.unwrap()
    }
}).as_str())
    } else {
        data = data.add(format!("
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct {} {{{}
}}", &obj.name, generate_fields(spec, obj)).as_str());
    }
    // let mut data = String::new();
    create_file(String::from(format!("types/{}.rs", &name)), data);
    name.clone()
}

pub fn should_box_field(spec: &spec_types::ApiDescription, obj_name: String, field_type: String) -> bool {
    if obj_name == field_type {
        return true;
    }
    let obj = spec.types.get(&field_type);
    if obj.is_none() {
        return false;
    }
    let fields = &obj.unwrap().fields;
    if fields.is_some() {
        for field in fields.as_ref().unwrap() {
            if field.types[0] == obj_name {
                return true;
            }
        }
    }
    let subtypes = &obj.unwrap().subtypes;
    if subtypes.is_some() {
        for subtype in subtypes.as_ref().unwrap() {
            if subtype == &obj_name {
                return true;
            }
        }
    }
    return false;
}

pub fn generate_fields(spec: &spec_types::ApiDescription, obj: &spec_types::TypeDescription) -> String {
    match &obj.fields {
        Some(fields) => {
            let mut generated_fields_string = String::new();
            for field in fields.iter() {
                // if obj.subtype_of.is_some() {
                //     if field.name == fields.first().unwrap().name {
                //         continue;
                //     }
                //     // let stof = &obj.subtype_of.as_ref().unwrap()[0];
                //     // let cf = btm.get(stof).unwrap();
                //     // if field.
                // }
                let field_type = field.types[0].clone();
                // if obj.name == field_type || (obj.name == "Chat" && field_type == "Message") || (obj.name == "Update" && field_type == "Message") || (obj.name == "Message" && field_type == "MaybeInaccessibleMessage") || (obj.name == "Message" && field_type == "GiveawayCompleted") {
                //     field_type = format!("Box<{}>", field_type)
                // }
                generated_fields_string = generated_fields_string.add(format!("\n    /// {}", field.description).as_str());
                if !field.required {
                    generated_fields_string = generated_fields_string.add("\n    #[serde(skip_serializing_if = \"Option::is_none\")]");
                }
                generated_fields_string = generated_fields_string.add(format!("\n    pub {name}: {dtype},",name=common::get_good_field_name(&field.name), dtype=common::get_type(&common::get_data_type(&field_type), field.required, should_box_field(spec, obj.name.clone(), field_type))).as_str())
            }
            generated_fields_string
        },
        None => String::new(),
    }
}

fn generate_subtypes(obj: &spec_types::TypeDescription, sm: &HashMap<String, HashMap<String, String>>) -> String {
    match &obj.subtypes {
        Some(subtypes) => {
            let mut generated_subtype_string = String::from("");
            let cfvs = (&sm).get(&obj.name);
            for subtype in subtypes.iter() {
                if cfvs.is_some() {
                    let cfv_opt = cfvs.unwrap().get(subtype);
                    if cfv_opt.is_some() {
                        let cfv = cfv_opt.unwrap();
                        generated_subtype_string = generated_subtype_string.add(format!("\n    #[serde(rename = \"{cfv}\")]").as_str());
                    }
                }
                generated_subtype_string = generated_subtype_string.add(format!("\n    {subtype}({subtype}),").as_str());
            }
            generated_subtype_string.add("\n")
        },
        None => String::new(),
    }
}

fn get_subtype_cfv(desc: &String) -> String {
    let elem = desc.split_whitespace().last().unwrap();
    elem.replace("\"", "")
}