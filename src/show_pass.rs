use crate::input_wrapper;
use crate::decrypt;
use crate::password_file;
use crate::PASSWORD_PATH;
use input_wrapper::{get_input,get_input_verbose);
use decrypt::decrypt_text;
use password_file::get_file_as_byte_vec;
use simple_dmenu::dmenu;
use crate::Entry;
use std::{thread, time};
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use crate::make_db_safe::decode;
use csv::ReaderBuilder;

//allows the user to select an entry based on it's title using dmenu, then prints it's username and copies the password to the clipboard. Only works on Xorg
pub fn get_entry() {

println!("please type in your db password: \n");
let pass = get_input();
let ciphertextread = get_file_as_byte_vec(&PASSWORD_PATH.to_string());
let plaintext_str = decrypt_text(ciphertextread,pass.to_string());

let mut rdr = ReaderBuilder::new()
         .delimiter(b',')
         .from_reader(plaintext_str.as_bytes());
let mut vec_of_entries: Vec<Entry> = Vec::new();

for result in rdr.records() {
    let record = result.expect("could not open dns record");
    let title_extracted = &record[0];
    let username_extracted = &record[1];
    let password_extracted = &record[2];
    let new_entry = Entry { title: title_extracted.to_owned(), username: username_extracted.to_owned(), password: password_extracted.to_owned()};
    vec_of_entries.push(new_entry);
}

let vec_of_usernames: Vec<&str> = vec_of_entries.iter().map(|ent| ent.title.as_str()).collect();
for (i, username) in vec_of_usernames.iter().enumerate() {
    println!("\ntype {} to get password and username for {}\n",i,username);
}

let user_selection = get_input_verbose();
let output = vec_of_usernames[user_selection.parse().unwrap()];

let output_user: Vec<Entry> = vec_of_entries
        .into_iter()
        .filter(|ent| ent.title == output)
        .collect();
let profile = &output_user[0];

let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();


println!("\n Copying password for title {} and username {} to clipboard for 20 seconds\n",profile.title,profile.username);
let pass = decode(profile.password.to_owned());
let result_of_clipboard = ctx.set_contents(pass.clone());
match result_of_clipboard {
        Ok(()) => {},
        Err(error) => println!("\n \n FAILED TO WRITE TO CLIPBOARD -- {} --: \n writing password to standard output {}",error,pass),
    };
let twenty_seconds = time::Duration::from_secs(20);

thread::sleep(twenty_seconds);

}
