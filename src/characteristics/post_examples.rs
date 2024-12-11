use std::fs;
use std::io;

use crate::core::characteristics::Characteristic;

pub struct PostExamples;

impl Characteristic for PostExamples {
    fn get_header(&self) -> String {
        "These are previous post examples.".to_string()
    }

    fn get_traits(&self, character_name: &str) -> io::Result<String> {
        let path = format!("./characters/{}/post_examples_traits.txt", character_name);
        fs::read_to_string(&path)
    }
}
