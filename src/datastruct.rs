use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct PathOpt {
    jump: u16,
    text: String,
}

impl PathOpt {
    /// Gets the place where the path is meant to jump
    pub fn get_jump(&self) -> u16 {
        self.jump
    }
    /// Gets the text for the option
    pub fn get_text(&self) -> &String {
        &self.text
    }
}

#[derive(Serialize, Deserialize)]
pub struct Path {
    text: String,
    options: Vec<PathOpt>,
}

impl Path {
    /// Gets the text for the path
    pub fn get_text(&self) -> &String {
        &self.text
    }
    /// Returns the options available for the path
    pub fn get_options(&self) -> &Vec<PathOpt> {
        &self.options
    }
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    name: String,
    author: String,
    slug: String,
    paths: HashMap<u16, Path>,
}

impl Game {
    /// Getter for the name of the game
    pub fn get_name(&self) -> &String {
        &self.name
    }
    /// Getter for the author of the game
    pub fn get_author(&self) -> &String {
        &self.author
    }
    /// Getter for the slug of the game
    pub fn get_slug(&self) -> &String {
        &self.slug
    }
    /// Getter for an arbitrary path
    pub fn get_path(&self, id: u16) -> &Path {
        &self.paths.get(&id).unwrap()
    }
    /// Checks to make sure that a path exists
    pub fn check_path(&self, path: &u16) -> bool {
        self.paths.contains_key(path)
    }
}
