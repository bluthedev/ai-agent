use std::io;

use crate::characteristics::{
    adjectives::Adjectives, bio::Bio, lore::Lore, post_examples::PostExamples,
    previous_messages::PreviousMessages, styles::Styles, topics::Topics,
};

// Trait to simulate each characteristic module
pub trait Characteristic {
    fn get_header(&self) -> String;
    fn get_traits(&self, character_name: &str) -> io::Result<String>;
}

pub struct Characteristics;

impl Characteristics {
    // Simulate getCharacteristics
    pub fn get_characteristics() -> Vec<Box<dyn Characteristic>> {
        vec![
            Box::new(Bio),
            Box::new(Lore),
            Box::new(PreviousMessages),
            Box::new(PostExamples),
            Box::new(Adjectives),
            Box::new(Topics),
            Box::new(Styles),
        ]
    }

    // Simulate buildCharacteristicsInstructions
    pub fn build_characteristics_instructions(character_name: &str) -> String {
        let mut chars_instruction = String::new();
        let characteristics = Self::get_characteristics();

        for characteristic in characteristics {
            chars_instruction += &characteristic.get_header();
            chars_instruction += "\n";
            chars_instruction += &characteristic.get_traits(character_name).unwrap();
            chars_instruction += "\n";
        }

        chars_instruction
    }

    // Simulate getCharacterInstructions
    pub fn get_character_instructions(chars_instruction: &String) -> &String {
        chars_instruction
    }
}
