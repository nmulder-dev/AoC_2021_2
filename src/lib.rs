use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    filename: String,
}

struct Location {
    distance: u32,
    depth: u32,
    aim: u32,
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
    let mut loc = Location {
        distance: 0,
        depth: 0,
        aim: 0,
    };

    for line in lines {
        let (dir, dist) = parse_command(line);
        if dir == "forward" {
            loc.distance += dist;
            loc.depth += loc.aim * dist;
        } else if dir == "up"{
            loc.aim -= dist;
        } else {    //dir == "down"
            loc.aim += dist;
        }
    }

    println!("Result: {}", loc.depth * loc.distance);
    Ok(())
}

fn parse_command(command: &str) -> (String, u32) {
    let mut cmd: String = String::new();
    let mut int: u32 = 0;
    for (pos, c) in command.chars().enumerate() {
        if c != ' ' {
            cmd.push(c);
        } else {
            let temp = &command[pos+1..];
            int = temp.parse::<u32>().unwrap();
            break;
        }
    }
    
    (cmd,int)
}

fn get_lines_vec(content: &str) -> Vec<&str> {
    // collect lines of content and return it
    content.lines().collect()
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