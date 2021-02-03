use std::fs;

fn main() {
    println!("Hello, world!");
    create_schema();
}


fn create_schema() {//schema: String) {
    // This function is meant to choose a schema and create it in the database.
    let paths = fs::read_dir("./database/schemas/").unwrap();
    for path in paths {
        println!("File name: {}", path.unwrap().path().display());
    }
}