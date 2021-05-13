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
                    text = Some(instruction.collect::<Vec<&str>>().join(" "));
                }
                "QUIT" => {
                    byte = 0b0110;
                }
                "AUTHOR" => {
                    byte = 0b1001;
                    text = Some(instruction.collect::<Vec<&str>>().join(" "));
                }
                "TEXT" => {
                    byte = 0b1010;
                    text = Some(instruction.collect::<Vec<&str>>().join(" "));
                }
                "TITLE" => {
                    byte = 0b1011;
                    text = Some(instruction.collect::<Vec<&str>>().join(" "));
                }
                "WIDTH" => {
                    byte = 0b1100;
                    param = match instruction.next() {
                        Some(i) => Some(i.parse().expect("Expected number after PATH")),
                        None => panic!("Expected number after PATH"),
                    };
                }
                "HEIGHT" => {
                    byte = 0b1101;
                    param = match instruction.next() {
                        Some(i) => Some(i.parse().expect("Expected number after PATH")),
                        None => panic!("Expected number after PATH"),
                    };
                }
                _ => panic!("Unknown command {}", command),
            };
            output.write(&[byte]);
            if let Some(value) = param {
                // Split the one u16 into two u8-s
                output.write(&convert_u16_to_two_u8s_be(value as u16));
            }
            if let Some(value) = &text {
                output.write(&[0b1010]);
                let length = value.len();
                output.write(&convert_u16_to_two_u8s_be(length as u16));
                output.write(value.as_bytes());
            }
        }
    }

    fn convert_u16_to_two_u8s_be(integer: u16) -> [u8; 2] {
        [(integer >> 8) as u8, integer as u8]
    }
}
