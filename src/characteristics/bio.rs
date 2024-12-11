use std::fs;
use std::io;

use crate::core::characteristics::Characteristic;

pub struct Bio;

impl Characteristic for Bio {
    fn get_header(&self) -> String {
        "This is your background.".to_string()
    }

    fn get_traits(&self, character_name: &str) -> io::Result<String> {
        let path = format!("./characters/{}/bio_traits.txt", character_name);
        fs::read_to_string(&path)
    }
}
