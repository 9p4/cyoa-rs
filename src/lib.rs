// Copyright 2021 Sambhav Saggi
// Licensed under the MIT license (see LICENSE or https://opensource.org/licenses/MIT)

// TODO: Docs

pub mod parser {
    pub fn parse(config: &String) {} // Parse compressed game config file
    pub fn convert(text: &String) -> String {
        return text.clone();
    } // Convert text file into compressed game config file
}

mod player {
    fn initialize() {} // Load game assets
    fn start() {} // Starting point for the game
    fn help() {} // Return help
    fn choose() {} // Choose option from step, return next step
    fn get_step() {} // Return text for step
    fn credits() {} // Return credits for game
}

#[cfg(test)]
mod tests {} // TODO: Tests
