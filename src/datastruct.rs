use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct PathOpt {
    jump: u16,
    text: String,
}

impl PathOpt {
    pub fn get_jump(&self) -> u16 {
        self.jump
    }
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
    pub fn get_text(&self) -> &String {
        &self.text
    }
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
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_author(&self) -> &String {
        &self.author
    }
    pub fn get_slug(&self) -> &String {
        &self.slug
    }
    pub fn get_path(&self, id: u16) -> &Path {
        &self.paths.get(&id).unwrap()
    }
    pub fn get_path_text(&self, path: &u16) -> &String {
        &self.paths.get(path).unwrap().text
    }
    pub fn get_path_opt(&self, path: &u16) -> &Vec<PathOpt> {
        &self.paths.get(path).unwrap().options
    }
    pub fn check_path(&self, path: &u16) -> bool {
        self.paths.contains_key(path)
    }
}
