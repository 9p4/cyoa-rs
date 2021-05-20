use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct PathOpt {
    jump: usize,
    text: String,
}

impl PathOpt {
    /// Gets the place where the path is meant to jump
    pub fn get_jump(&self) -> usize {
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
    paths: HashMap<usize, Path>,
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
    pub fn get_path(&self, id: usize) -> &Path {
        &self.paths.get(&id).unwrap()
    }
    /// Gets the size of the game (in pages)
    pub fn get_path_len(&self) -> usize {
        self.paths.len()
    }
    /// Checks to make sure that a path exists
    pub fn check_path(&self, path: &usize) -> bool {
        self.paths.contains_key(path)
    }
}
