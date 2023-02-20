use std::fs::File;
use crate::add_entry::add_entry_mass;
use crate::input_wrapper::get_input;

//simpily parses a csv, loops over the rows and adds them to the database file
pub fn mass_import(csv_file_path: &str) {

    let file = File::open(csv_file_path).expect("could not open file path");
    let mut rdr = csv::Reader::from_reader(file);
    println!("Please type in your db password: \n");
    let db_pass = get_input();
    for result in rdr.records() {
        let record = result.expect("could not open dns record");
        let title = &record[0];
        let username = &record[1];
        let password = &record[2];
        add_entry_mass(title.to_string(),username.to_string(),password.to_string(),db_pass.to_string());
    }


}
