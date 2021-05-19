mod datastruct;

pub struct State {
    pub config: datastruct::Game,
    current_path: u16,
    history: Vec<u16>,
}

impl State {
    pub fn goto(&mut self, path: u16) {
        self.current_path = path;
        self.history.push(path)
    }

    pub fn get_path(&self) -> u16 {
        self.current_path
    }

    // TODO: Implement
    pub fn export_save(&self) -> String {
        String::new()
    }

    // TODO: Implement
    pub fn import_save(&mut self, save: &String) {}

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
