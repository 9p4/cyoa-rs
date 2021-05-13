pub mod cyoafile {
    use std::io::Write;
    pub fn compress(config: &String, output: &mut std::fs::File) {
        for line in config.lines() {
            let instruction = line.split_whitespace().next().unwrap();
            match instruction {
                "END" => output.write(&[0x0000u8]),
                _ => panic!("Unknown instruction in file!"),
            };
        }
    }
}
