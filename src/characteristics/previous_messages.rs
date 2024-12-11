use std::fs;
use std::io;

use crate::core::characteristics::Characteristic;

pub struct PreviousMessages;

impl Characteristic for PreviousMessages {
    fn get_header(&self) -> String {
        "These are examples of your previous messages.".to_string()
    }

    fn get_traits(&self, character_name: &str) -> io::Result<String> {
        let path = format!("./characters/{}/previous_messages_traits.txt", character_name);
        fs::read_to_string(&path)
    }
}
