pub mod cyoafile {
    use std::io::Write;
    pub fn compress(config: &String, output: &mut std::fs::File) {
        let lines = config.lines();
        for line in lines {
            let byte: u8;
            let mut param: Option<u16> = None;
            let mut text: Option<String> = None;
            let mut instruction = line.split_whitespace();
            let command = match instruction.next() {
                Some(i) => i,
                None => continue,
            };
            println!("Command: {}", command);
            match command {
                "END" => byte = 0b0001,
                "GOTO" => {
                    byte = 0b0010;
                    param = match instruction.next() {
                        Some(i) => Some(i.parse().expect("Expected number after GOTO")),
                        None => panic!("Expected number after GOTO"),
                    };
                }
                "PATH" => {
                    byte = 0b0011;
                    param = match instruction.next() {
                        Some(i) => Some(i.parse().expect("Expected number after PATH")),
                        None => panic!("Expected number after PATH"),
                    };
                }
                "OPTIONS" => byte = 0b0100,
                "OPTION" => {
                    byte = 0b0101;
                    param = match instruction.next() {
                        Some(i) => Some(i.parse().expect("Expected number after OPTION")),
                        None => panic!("Expected number after OPTION"),
                    };
                    instruction.next();
                    text = Some(instruction.collect::<Vec<&str>>().join(" "));
                }
                "QUIT" => {
                    byte = 0b0110;
                    instruction.next();
                    text = Some(instruction.collect::<Vec<&str>>().join(" "));
                }
                "AUTHOR" => {
                    byte = 0b1001;
                    instruction.next();
                    text = Some(instruction.collect::<Vec<&str>>().join(" "));
                }
                "TEXT" => {
                    byte = 0b1010;
                    instruction.next();
                    text = Some(instruction.collect::<Vec<&str>>().join(" "));
                    param = Some(&text.unwrap().len() as u16);
                }
                "TITLE" => byte = 0b1011,
                "WIDTH" => byte = 0b1100,
                "HEIGHT" => byte = 0b1101,
                _ => panic!("Unknown command {}", command),
            };
            println!("Command: {}", command);
            if let Some(value) = param {
                println!("Param: {}", value);
            }
            if let Some(value) = text {
                println!("Text: {}", value);
            }
            output.write(&[byte]);
        }
    }
}
