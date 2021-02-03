use std::fs;

fn main() {
    create_schema();
}


fn create_schema() {//schema: String) {
    // This function is meant to choose a schema and create it in the database.
    let paths = fs::read_dir("./database/schemas/").unwrap();
    let mut filename: String;
    for path in paths {
        filename = path.unwrap().path().display().to_string();
        println!("Name: {}", filename);
    }
}