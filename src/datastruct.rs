use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct PathOpt {
    pub jump: u16,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct Path {
    pub text: String,
    pub options: Vec<PathOpt>,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    pub author: String,
    pub slug: String,
    pub paths: HashMap<u16, Path>,
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
    pub fn get_path_text(&self, path: &u16) -> &String {
        &self.paths.get(path).unwrap().text
    }
    pub fn get_path_opt(&self, path: &u16) -> &Vec<PathOpt> {
        &self.paths.get(path).unwrap().options
    }
}
