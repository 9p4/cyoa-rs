use cyoa;
use std::env;
use std::fs;
use std::io;

fn main() {
    println!("Loading...");
    // Get game config
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Something went wrong reading the game config");
    let mut game = cyoa::load(&contents);
    println!("Done!");

    // Print intro page
    println!("{}", game.config.get_name());
    println!("{}", game.config.get_slug());
    println!("Created by: {}", game.config.get_author());

    // Main loop
    loop {
        // Print the text
        println!("{}", game.config.get_path_text(&game.get_path()));
        println!("What do you do?");
        for option in game.config.get_path_opt(&game.get_path()) {
            println!("{}", option.text);
        }
        let mut selection = String::new();

        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");

        let selection: u16 = selection.trim().parse().expect("Please type a number!");
        game.goto(game.config.get_path_opt(&game.get_path())[(selection - 1) as usize].jump)
    }
}
