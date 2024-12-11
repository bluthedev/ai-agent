use std::fs;
use std::io::{self};

use super::characteristics::Characteristics;

pub struct InstructionBuilder {
    instructions: String,
}

impl InstructionBuilder {
    pub fn new() -> Self {
        Self {
            instructions: String::new(),
        }
    }

    // Read base instructions from a file
    pub fn get_base(character_name: &str) -> io::Result<String> {
        let path = format!("./characters/{}/instructions/base.txt", character_name);
        fs::read_to_string(&path)
    }

    // Read suffix instructions from a file
    pub fn get_suffix(character_name: &str) -> io::Result<String> {
        let path = format!("./characters/{}/instructions/suffix.txt", character_name);
        fs::read_to_string(&path)
    }

    // Add instruction to the internal buffer
    pub fn add_instruction(&mut self, instruction: &str) {
        self.instructions.push_str(instruction);
    }

    // Add multiple instructions (array equivalent)
    pub fn add_instructions(&mut self, instructions: Vec<String>) {
        for instruction in instructions {
            self.add_instruction(&instruction);
        }
    }

    // Build the complete instructions
    pub fn build_instructions(&mut self, character_name: &str) -> io::Result<()> {
        self.instructions.clear();

        let characteristics = Characteristics::build_characteristics_instructions(character_name);

        // Add base instructions
        if let Ok(base) = Self::get_base(character_name) {
            self.add_instruction(&base);
        }

        // Add characteristics instructions
        self.add_instruction(&characteristics);

        // Add suffix instructions
        if let Ok(suffix) = Self::get_suffix(character_name) {
            self.add_instruction(&suffix);
        }

        Ok(())
    }

    // Get the complete instructions
    pub fn get_instructions(&self) -> &str {
        &self.instructions
    }
}
