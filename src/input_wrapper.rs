use std::io::{stdin,stdout,Write};
use rpassword::read_password;

pub fn get_input() -> String {


    print!("Type a password: ");
    std::io::stdout().flush().unwrap();
    let mut input = read_password().unwrap();
    // flush terminal buffer
    stdout().flush().expect("there has been an error when flushing terminal ");
    // read terminal input into the input variable

    //remove \n if on linux
    if let Some('\n')=input.chars().next_back() {
        input.pop();
    }
    //remove \r if on windows
    if let Some('\r')=input.chars().next_back() {
        input.pop();
    }

    return input;
}

pub fn get_input_verbose() -> String {

    let mut input = String::new();

    // flush terminal buffer
    stdout().flush().expect("there has been an error when flushing terminal ");
    // read terminal input into the input variable
    stdin().read_line(&mut input).expect("Did not enter a correct string");
    //remove \n if on linux
    if let Some('\n')=input.chars().next_back() {
        input.pop();
    }
    //remove \r if on windows
    if let Some('\r')=input.chars().next_back() {
        input.pop();
    }

    return input;
}
