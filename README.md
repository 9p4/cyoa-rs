# CYOA-rs

A text-based choose-your-own-adventure game engine written in Rust.

## Game File Format Reference

| Binary | Instruction | Notes |
| ------ | ----------- | ----- |
| 0001   | END         | Ends the current path block |
| 0010   | GOTO        | The next byte (u16) represents the path to go to |
| 0011   | PATH        | Is the compliment of GOTO. The next byte (u16) represents the position |
| 0100   | OPTIONS     | Beings an "options" block of instructions |
| 0101   | OPTION      | The next byte (u16) represents the position the option should bring you to, then a string for the option |
| 0110   | QUIT        | Stops the program |
| 1001   | AUTHOR      | The next bytes of length u16 contain the string for the Author's name |
| 1010   | TEXT        | Begins printing string from this point of length u16 |
| 1011   | TITLE       | The next bytes of length u16 contain the program title |
| 1100   | WIDTH       | The minimum width for this game to play |
| 1101   | HEIGHT      | The minimum height for this game to play |
