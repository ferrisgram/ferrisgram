#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;
use std::path::Path;

pub const IMP_URL: &str = "crate";

pub const WARNING_COMMENT: &str = 
"// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

";

pub const tg_type_string: &str = "String";
pub const tg_type_boolean: &str = "Boolean";
pub const tg_type_float: &str = "Float";
pub const tg_type_integer: &str = "Integer";
pub const tg_type_file: &str = "InputFile";

pub const RequiresCustomDemarshaller: &str = "MaybeInaccessibleMessage";
pub const Untagged: &str = "InputMessageContent"; 

const SOURCE_PATH: &str = "../src/";

pub fn is_dtype_builtin(r#type: &str) -> bool {
    for dtype in [tg_type_boolean, tg_type_string, tg_type_float, tg_type_integer] {
        if r#type.contains(dtype) {
            return true;
        }
    }
    return false;
}

pub fn get_data_type(raw_type: &String) -> String {
    raw_type
    .replace(tg_type_string, "String")
    .replace(tg_type_boolean, "bool")
    .replace(tg_type_integer, "i64")
    .replace(tg_type_float, "f64")
    .replace(tg_type_file, "String")
}

pub fn get_good_field_name(name: &String) -> String {
    if name.as_str() == "type" {
        return String::from("r#type");
    }
    name.clone()
}

fn handle_array(raw_type: &String) -> String {
    let mut num: i16 = 0;
    let mut good_type = raw_type.clone();
    for mem in raw_type.split_whitespace() {
        if mem == "Array" {
            num += 1;
        }
    }
    if num == 0 {
        return good_type;
    }
    for _ in 0..num {
        good_type = format!("Vec<{}>", good_type.replace("Array of ", ""));
    }
    good_type
}

pub fn get_type(raw_type: &String, is_required: bool, should_box: bool) -> String {
    let mut good_type = raw_type.clone();
    if good_type.contains("Array of") {
        good_type = handle_array(raw_type)
    } else if should_box {
        good_type = format!("Box<{}>", good_type)
    }
    if is_required {
        return good_type;
    } else {
        return with_optional(good_type);
    }
}

pub fn with_optional(good_type: String) -> String {
    return format!("Option<{}>", good_type);
}

pub fn create_file(name: String, text: String) -> bool {
    let root_path = String::from(SOURCE_PATH).add(&name);
    let path = Path::new(root_path.as_str());
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(text.as_bytes()) {
        Err(why) => {
            println!("couldn't write to {}: {}", display, why);
            false
        },
        Ok(_) => true,
    }
}