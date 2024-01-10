use reqwest::multipart::Part;
pub trait InputFile {
    fn get_part(&self) -> Part;
}

pub struct NamedFile {
    pub file_name: String,
    pub file_data: Vec<u8>,
}

impl InputFile for NamedFile {
    fn get_part(&self) -> Part {
        Part::bytes(self.file_data.clone()).file_name(self.file_name.clone())
    }
}

impl InputFile for String {
    fn get_part(&self) -> Part {
        Part::text(self.clone())
    }
}

impl InputFile for &str {
    fn get_part(&self) -> Part {
        Part::text(self.to_string())
    }
}
