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
    let mut game = cyoa::State::new(&contents);
    println!("Done!");

    // Print intro page
    println!("{}", game.config.get_name());
    println!("{}", game.config.get_slug());
    println!("Created by: {}", game.config.get_author());

    // Main loop
    loop {
        // Print the text
        println!("{}", game.get_path().get_text());
        println!("What do you do?");
        let mut count = 1;
        for option in game.get_path().get_options() {
            println!("{}: {}", count, option.get_text());
            count+=1;
        }
        let mut selection = String::new();

        // Read and jump
        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");

        let selection: u16 = selection.trim().parse().expect("Please type a number!");
        let jump = game.get_path().get_options()[(selection-1) as usize].get_jump();
        game.jump(jump)
    }
}
