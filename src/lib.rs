use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // skip first arg
        args.next();

        // get filename from args or return error
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing input filename"),
        };

        Ok(Config {
            filename: filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let lines = get_lines_vec(&contents);

    // YOUR CODE HERE!
    for line in lines {
        parse_command(line)
    }

    Ok(())
}

fn parse_command(command: &str) -> (&str, u32) {
    let command_chars = command.chars().collect();
    let mut cmd: &str;
    for (pos, c) in chars {
        if c != '' {
            cmd += c;
        } else {
            let temp = command[pos + 1..cmd.len()+1]
            let int: u32 = temp.parse().unwrap();
        }
    }
    (cmd, int)
}

fn adjust_dist(units: u32) -> u32 {
    0
}

fn get_lines_vec(content: &str) -> Vec<u32> {
    // collect lines of content and return it
    content.lines().map(|l| l.parse::<u32>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_length_greater_than_one() {
        let content = "\
Dolore eiusmod irure laboris consequat eiusmod irure ad occaecat aute qui veniam mollit officia officia.
Non irure nisi ut non cillum pariatur pariatur ex officia consectetur tempor adipisicing veniam. Dolore
irure deserunt aliquip minim incididunt ipsum eu mollit eiusmod Lorem qui dolor. Ipsum sint nostrud
anim labore ex adipisicing excepteur velit fugiat pariatur fugiat ex incididunt.";
        
        let test_vec = get_lines_vec(content);

        assert!(test_vec.len() > 1);
    }

    #[test]
    fn vector_length_equal_to_zero() {
        let content = "";

        let test_vec = get_lines_vec(content);

        assert_eq!(test_vec.len(), 0);
    }
}