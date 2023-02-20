use crate::encrypt::encrypt_text;
use crate::password_file::save;
use crate::PASSWORD_PATH;
use crate::input_wrapper::get_input;


pub fn mk_db() {
println!("\n Making a password database in {} \n ",PASSWORD_PATH);
println!("please type a password to lock your database: \n");
let pass = get_input();
println!("please confirm password_database: \n");
let pass_0 = get_input();
if pass == pass_0 {
    let encrypted_nothing = encrypt_text("title,username,password".to_string(),pass.to_string());
    save(encrypted_nothing,&PASSWORD_PATH);
    println!("blank password database made");
}
else {
    println!("Passwords did not match -- Database not made")
}
}
