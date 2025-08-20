use std::fs::{self, File, remove_file};
use std::io::{self, Read, ErrorKind, Write};

pub fn run() {
    open_file();
    match_different_errors();
    expect();

    let mut file = File::create("username.txt").expect("Unable to create file");
    file.write_all(b"username").expect("Unable to write to file");

    propagating_errors();
    shortcut_propagate();
    shortcut_chain_propagate();
    even_shorter();

    // cleanup
    remove_file("username.txt").expect("Unable to remove file");
}

fn even_shorter() {
    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("username.txt")
    }

    match read_username_from_file() {
        Ok(username) => println!("Username: {:?}", username),
        Err(error) => println!("Problem reading the file: {:?}", error),
    }
}

fn shortcut_chain_propagate() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("username.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    match read_username_from_file() {
        Ok(username) => println!("Username: {:?}", username),
        Err(error) => println!("Problem reading the file: {:?}", error),
    }
}

fn shortcut_propagate() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("username.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    match read_username_from_file() {
        Ok(username) => println!("Username: {:?}", username),
        Err(error) => println!("Problem reading the file: {:?}", error),
    }
}

fn propagating_errors() {

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("username.txt");
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(error) => return Err(error),
        };
        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(error) => Err(error),
        }
    }

    match read_username_from_file() {
        Ok(username) => println!("Username: {:?}", username),
        Err(error) => println!("Problem reading the file: {:?}", error),
    }
}

fn expect() {
    // so you don't need to use match
    File::create("temp.txt").expect("Unable to create file");
    let _temp_file = File::open("temp.txt").expect("Unable to open file");
    // cleanup
    remove_file("temp.txt").expect("Unable to remove file");
}

fn match_different_errors() {
    let greeting_file_result = File::open("hello2.txt");
    // this can look nicer after I cover closures and unwrap_or_else
    match greeting_file_result {
        Ok(_file) => {
            println!("File opened successfully.");
            remove_file("hello2.txt").expect("Unable to remove file");
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello2.txt") {
                Ok(fc) => {
                    println!("Created file: {:?}", fc);
                    remove_file("hello2.txt").expect("Unable to remove file");
                },
                Err(e) => println!("Problem creating the file: {:?}", e),
            },
            _ => {
                println!("There was a problem opening the file: {:?}", error)
            }
        },
    };
}

fn open_file() {
    let greeting_file_result = File::open("hello.txt");
        match greeting_file_result {
            Ok(_file) => {
                println!("File opened successfully.");
            },
            Err(error) => {
                println!("There was a problem opening the file: {:?}", error)
            },
        };
}