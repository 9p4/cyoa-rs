// Copyright 2021 Sambhav Saggi
// Licensed under the MIT license (see LICENSE or https://opensource.org/licenses/MIT)

// TODO: Docs
//

pub mod fileutils;
pub use crate::fileutils as futils;

mod game {
    struct Config {
        title: String,
        author: String,
        minwidth: u16,
        minheight: u16,
        paths: Vec<Path>,
    }
    
    impl Config {
        pub fn get_author(&self) -> String {
            self.author.clone()
        }
        pub fn get_title(&self) -> String {
            self.title.clone()
        }
        pub fn get_minwidth(&self) -> u16 {
            self.minwidth
        }
        pub fn get_minheight(&self) -> u16 {
            self.minheight
        }
    }

    struct PathOption {
        key: u32,
        data: String,
        pathkey: u32,
    }

    struct Path {
        key: u32,
        data: String,
        options: Vec<PathOption>,
    }

    struct State {
        position: u32,
        history: String,
    }

    pub mod parser {
        pub fn parse(config: &String) {
        } // Parse compressed game config file

        // Convert text file into compressed game config file
        pub fn convert(text: &String) -> String {
            for line in text.lines() {
                
            }
            "aaa".to_string()
        }
    }

    mod player {
        fn initialize(config: &String) {} // Load game assets
        fn start() {} // Starting point for the game
        fn help() {} // Return help
        fn choose() {} // Choose option from step, return next step
        fn get_step() {} // Return text for step
        fn load(posititon: &String) {} // Load position
        fn credits() {} // Return credits for game
    }

    #[cfg(test)]
    mod tests {} // TODO: Tests
}
