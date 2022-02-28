use std::ops::Add;

use crate::common::get_good_field_name;

use super::{common, spec_types};
use common::create_file;
use convert_case::{Casing, Case};

pub fn create_import_crate(obj: &spec_types::MethodDescription) -> String {
    let (mut impdata, import_array) = match &obj.fields {
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
                    // use crate::types::PhotoSize
                    import_array.push(impname);
                }
            }
            if import_array.len() == 0 {
                (String::new(), import_array)
            } else {    
                let mut imptxt = String::from("use crate::types::");
                if import_array.len() == 1 {
                    (imptxt.add(import_array[0].as_str()).add(";\n"), import_array)
                } else {
                    let mut num = 0;
                    for imp in import_array.iter() {
                        num += 1;
                        if num == 1 {
                            imptxt = imptxt.add(format!("{{{}", imp).as_str());
                            continue;
                        }
                        imptxt = imptxt.add(format!(", {}", imp).as_str())
                    }
                    (imptxt.add("};\n"), import_array)
                }   
            }
        },
        None => (String::new(), Vec::new()),
    };
    if obj.returns.len() > 0 && !common::is_dtype_builtin(&obj.returns[0]) {
        let mut found = false;
        for imptype in import_array.iter() {
            if &obj.returns[0] == imptype {
                found = true;
                break;
            }
        }
        if !found {
            impdata = impdata.add(format!("use crate::types::{};\n", &obj.returns[0].replace("Array of ", "")).as_str());
        }
    }
    impdata
}

pub async fn generate_methods(spec: &spec_types::ApiDescription) {
    let mut data = String::new();
    for (_, method) in spec.methods.iter() {
        let (good_tname, builder_name) = generate_method(method).await;
        data = data.add(format!("\nmod {};\npub use {}::{};\n", good_tname, good_tname, builder_name).as_str());
    }
    create_file(String::from("methods/mod.rs"), data);
}

async fn generate_method(method: &spec_types::MethodDescription) -> (String, String) {
    let name = &method.name.to_case(Case::Snake); 
    let mut data = String::from(common::WARNING_COMMENT);
    // data = data.add(&create_import_crate(obj));
    data = data.add("use serde::Serialize;\n\n");
    data = data.add("use crate::Bot;\n");
    data = data.add("use crate::error::Result;\n");
    data = data.add(create_import_crate(method).as_str());
    // data = data.add(format!("\n/// <{}>", method.href).as_str());
    let builder_name = format!("{}Builder", &method.name.to_case(Case::UpperCamel)); 
    data = data.add(generate_bot_imp(&method, &builder_name).as_str());
    data = data.add(generate_builder(&method, &builder_name).as_str());
    data = data.add(generate_imp(&method, &builder_name).as_str());
    create_file(String::from(format!("methods/{}.rs", &name)), data);
    (name.clone(), builder_name)
}

// impl Bot {
//     pub fn send_message(&self, chat_id: i64, text: String) -> SendMessageBuilder {
//         SendMessageBuilder::new(&self, chat_id, text)
//     }
// }
fn generate_bot_imp(method: &spec_types::MethodDescription, builder_name: &String) -> String {
    let mut args = String::new();
    let mut vals = String::new();
    let method_name = method.name.to_case(Case::Snake);
    if method.fields.is_some() {
        for field in method.fields.as_ref().unwrap() {
            if !field.required {
                continue;
            }
            let field_name = get_good_field_name(&field.name);
            let field_type = field.types[0].clone();
            let dtype = common::get_type(field, &common::get_data_type(&field_type));
            args = args.add(format!(", {field_name}: {dtype}").as_str());
            vals = vals.add(format!(", {field_name}").as_str())
        }
    }
    let mut desc = String::new();
    for d in method.description.iter() {
        desc = desc.add(format!("\n    /// {}", &d).as_str())
    }
    desc = desc.add(format!("\n    /// <{}>", &method.href).as_str());
    let data = format!("
impl Bot {{{desc}
    pub fn {method_name}(&self{args}) -> {builder_name} {{
        {builder_name}::new(self{vals})
    }}
}}

");
    data
}

fn generate_builder(method: &spec_types::MethodDescription, builder_name: &String) -> String {
    let mut data = String::new(); 
    data = data.add(format!("#[derive(Serialize)]
pub struct {}<'a> {{
    #[serde(skip)]
    bot: &'a Bot,", builder_name).as_str());
    data = data.add(generate_fields(method).as_str());
    data = data.add("\n}\n\n");
    data
}

fn generate_imp(method: &spec_types::MethodDescription, builder_name: &String) -> String {
    let (new_fn_attr, new_fn_cntnt, builder_fns) = match &method.fields {
        Some(fields) => {
            let mut builder_fns = String::new();
            let mut new_fn_attr = String::new();
            let mut new_fn_cntnt = String::new();
            for field in fields.iter() {
                let field_name = get_good_field_name(&field.name);
                let field_type = field.types[0].clone();
                let dtype = common::get_type(field, &common::get_data_type(&field_type));
                let field_value: String; 
                if field.required {
                    field_value = field_name.clone();
                    new_fn_attr = new_fn_attr.add(format!(", {}: {}", field_name, dtype).as_str());
                    new_fn_cntnt = new_fn_cntnt.add(format!("\n            {},", field_name).as_str());
                } else {
                    field_value = format!("Some({})", field_name);
                    new_fn_cntnt = new_fn_cntnt.add(format!("\n            {}: None,", field_name).as_str());
                }
                builder_fns = builder_fns.add(format!("
    pub fn {field_name}(mut self, {field_name}: {rtype}) -> Self {{
        self.{field_name} = {field_value};
        self
    }}
                ", rtype=common::get_type_without_optional(&common::get_data_type(&field_type))).as_str());
            }
        (new_fn_attr, new_fn_cntnt, builder_fns)
        },
        None => {
            (String::new(), String::new(), String::new())
        }
    };
    let builder_fns = builder_fns.add(format!("
    pub async fn send(self) -> Result<{to_return}> {{
        let form = serde_json::to_value(&self)?;
        self.bot.get::<{to_return}>(\"{raw_method_name}\", Some(&form)).await
    }}
", to_return=common::get_type_without_optional(&common::get_data_type(&method.returns[0])), raw_method_name=method.name).as_str());

    let impl_new_fn_string = format!("
impl <'a> {}<'a> {{
    pub fn new(bot: &'a Bot{}) -> Self {{
        Self{{
            bot,{}
        }}
    }}
{}
}}", builder_name, new_fn_attr, new_fn_cntnt, builder_fns);
    impl_new_fn_string
}

fn generate_fields(obj: &spec_types::MethodDescription) -> String {
    match &obj.fields {
        Some(fields) => {
            let mut generated_fields_string = String::new();
            for field in fields.iter() {
                let mut field_type = field.types[0].clone();
                if obj.name == field_type || (obj.name == "Chat" && field_type == "Message") || obj.name == "Message" && field_type == "Chat" {
                    field_type = format!("Box<{}>", field_type)
                }
                generated_fields_string = generated_fields_string.add(format!("\n    /// {}", field.description).as_str());
                if !field.required {
                    generated_fields_string = generated_fields_string.add(format!("\n    #[serde(skip_serializing_if = \"Option::is_none\")]").as_str());
                }
                generated_fields_string = generated_fields_string.add(format!("\n    pub {name}: {dtype},",name=common::get_good_field_name(&field.name), dtype=common::get_type(field, &common::get_data_type(&field_type))).as_str())
            }
            generated_fields_string
        },
        None => String::new(),
    }
}