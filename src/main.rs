extern crate bincode;
extern crate dirs;
extern crate clipboard;
extern crate csv;
mod encrypt;
mod decrypt;
mod password_file;
mod input_wrapper;
mod show_pass;
mod make_db;
mod add_entry;
mod del_entry;
mod make_db_safe;
mod mass_import;
use std::env;
use show_pass::get_entry;
use make_db::mk_db;
use add_entry::add_entry;
use add_entry::add_entry_gen;
use del_entry::delete_entry;
use mass_import::mass_import;
use std::process::exit;
extern crate lazy_static;


const PASSWORD_PATH: &str = "/home/sherlly/usb/password.db";

#[derive(Debug,Clone)]
struct Entry {
    title: String,
    username: String,
    password: String
}
fn main() {
println!("\n setting database password in {} \n change in source code",&PASSWORD_PATH);

println!("\n Welcome to Sherlly's password manager (SPM) \n use flag -h for help" );
let args: Vec<String> = env::args().collect();
let mode = &args[1];

if mode == "-h" || mode == "--help" {
    println!("\n
SET WHERE THE DB FILE SHOULD GO IN THE SOURCE CODE BEFORE COMPILING \n
SOME EMOJI CANNOT BE USED IN PASSWORDS AS IT RUINS THE PARSING - ANY USED IN THE SOURCE CODE ARE OFFLIMITS \n
--show_entries will launch dmenu to selected a password from it's title \n
--make_db will make a new password file in PASSWORD_PATH and you will be able to set the password \n
--add_entry will let you add an entry to the password database in the form of title username password. You can also generate apassword with title username -g <len> where len is a number to specificy the kength of the password \n
--mass_import will take a file path to a csv. The first row of the csv will be treated as a header. It will be of the form \"title\",\"username\",\"password\" \n
--del_entry will delete an entry in the database from the title alone. Note, you can use make_db to overwrite the file to delete them all \n
             ");
}

if mode == "--show_entries" {
get_entry();
}

else if mode == "--make_db" {
    mk_db();
}

else if mode == "--add_entry" {

let title = args[2].parse::<String>().unwrap();
let username = args[3].parse::<String>().unwrap();
let password = args[4].parse::<String>().unwrap();
if password == "-g" {
    let len = args[5].parse::<usize>().unwrap();
    add_entry_gen(title,username,len);
    println!("\n added entry \n");
    exit(0);
}

add_entry(title,username,password);
println!("\n added entry \n");
}

else if mode == "--mass_import" {

let file_path = args[2].parse::<String>().unwrap();
mass_import(&file_path);
println!("mass imported from csv");
}

else if mode == "--del_entry" {
let title = args[2].parse::<String>().unwrap();

delete_entry(title);
}
else {
    println!("please type in a mode in the format rust_pwd_manager --mode <options> where the modes are: show_entries, make_db, add_entry, del_entry, mass_import");


}


}
