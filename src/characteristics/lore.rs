use std::fs;
use std::io;

use crate::core::characteristics::Characteristic;

pub struct Lore;

impl Characteristic for Lore {
    fn get_header(&self) -> String {
        "This is your lore.".to_string()
    }

    fn get_traits(&self, character_name: &str) -> io::Result<String> {
        let path = format!("./characters/{}/lore_traits.txt", character_name);
        fs::read_to_string(&path)
    }
}
