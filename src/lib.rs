mod datastruct;
use crate::datastruct::Path;

pub struct State {
    pub config: datastruct::Game,
    current_path: u16,
    history: Vec<u16>,
}

impl State {
    pub fn jump(&mut self, path: u16) {
        if !self.config.check_path(&path) {
            panic!("Path doesn't exist");
        }
        self.current_path = path;
        self.history.push(path)
    }

    pub fn get_path(&self) -> &Path {
        &self.config.get_path(self.current_path)
    }

    // TODO: Implement
    pub fn export_save(&self) -> String {
        String::new()
    }

    pub fn import_save(&mut self, save: &String) {
        self.history.clear();
        for item in save.split(",") {
            self.history.push(item.parse::<u16>().unwrap());
        }
    }

    // Expensive but I don't expect it to be used much, so it's probably fine
    pub fn backtrack(&mut self) {
        if self.history.len() >= 1 {
            let prev = self.history.last().unwrap();
            self.current_path = *prev;
        }
    }
}
pub fn load(data: &String) -> State {
    State {
        config: serde_json::from_str(data).unwrap(),
        current_path: 0,
        history: Vec::<u16>::new(),
    }
}
