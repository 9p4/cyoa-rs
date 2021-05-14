use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct PathOpt {
    jump: u16,
    text: String,
}

#[derive(Serialize, Deserialize)]
struct Path {
    text: String,
    options: HashMap<u16, PathOpt>
}

#[derive(Serialize, Deserialize)]
struct Game {
    name: String,
    author: String,
    slug: String,
    paths: HashMap<u16, Path>
}
