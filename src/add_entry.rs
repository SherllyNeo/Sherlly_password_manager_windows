use crate::PASSWORD_PATH;
use crate::encrypt::encrypt_text;
use crate::decrypt::decrypt_text;
use crate::password_file::get_file_as_byte_vec;
use crate::password_file::save;
use crate::input_wrapper::get_input;
use crate::make_db_safe::encode;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;



//function to add one entry to database
pub fn add_entry(title_dirty: String, username_dirty: String, password_dirty: String) {

println!("Please type your db password: \n");
let db_pass = get_input();
let title =  encode(title_dirty);
let username = encode(username_dirty);
let password =  encode(password_dirty);

//get and decrypt file
let ciphertextread = get_file_as_byte_vec(&PASSWORD_PATH.to_string());
let plaintext_str: String = decrypt_text(ciphertextread,db_pass.to_string());

//make new entry and append
let append_text = format!("{}\n{},{},{}",plaintext_str,title,username,password);


let encrypted_new = encrypt_text(append_text,db_pass);
save(encrypted_new,&PASSWORD_PATH);


}


//a function to mass add entries to the database so that you are not asked multiple times for the password
pub fn add_entry_mass(title_dirty: String, username_dirty: String, password_dirty: String,db_pass: String) {

let title =  encode(title_dirty);
let username = encode(username_dirty);
let password =  encode(password_dirty);

//get and decrypt file
let ciphertextread = get_file_as_byte_vec(&PASSWORD_PATH.to_string());
let plaintext_str: String = decrypt_text(ciphertextread,db_pass.to_string());
let append_text = format!("{}\n{},{},{}",plaintext_str,title,username,password);
//make new entry and append
let encrypted_new = encrypt_text(append_text,db_pass);
save(encrypted_new,&PASSWORD_PATH);


}

//function that generates a password with specific length and adds it to the database
pub fn add_entry_gen(title_dirty: String, username_dirty: String,len: usize) {


let password_dirty: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(len)
    .map(char::from)
    .collect();

println!("Please type your db password: \n");
let db_pass = get_input();
let title =  encode(title_dirty);
let username = encode(username_dirty);
let password =  encode(password_dirty);

//get and decrypt file
let ciphertextread = get_file_as_byte_vec(&PASSWORD_PATH.to_string());
let plaintext_str: String = decrypt_text(ciphertextread,db_pass.to_string());


//make new entry and append
let append_text = format!("{}\n,{},{},{}",plaintext_str,title,username,password);

let encrypted_new = encrypt_text(append_text,db_pass);
save(encrypted_new,&PASSWORD_PATH);


}
