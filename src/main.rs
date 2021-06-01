use std::fs;
use std::path::{Path, PathBuf};
use serde_json;

fn main() {
    create_schema("old_schema");
}


fn create_schema(schema: &str) {
    // This function is meant to choose a schema and create it in the database.
    let paths = fs::read_dir(&Path::new(& "./database/schemas/")).unwrap();

    let mut names: Vec<PathBuf> = Vec::new();
    for path in paths {
        names.push(path.unwrap().path());
    }
    // So this here allows us to reuse the memory allocation from schema for the new comp_schema
    // The mut keyword means mutable or in otherwords it is a signal that this variable will change.
    let mut comp_schema: String = schema.to_owned();
    comp_schema.push_str(".json");
    let mut json_schema: PathBuf = PathBuf::new();
    for name in &names {
        let file = name.display().to_string().split("/").last().unwrap().to_string();
        if file == comp_schema {
            // Leaving this here for debug purposes
            // println!("File name: {}\nPath: {}", file, name.display());
            json_schema = name.to_path_buf();
        }
    }
    println!("Path: {}", json_schema.display());
    let json_file = fs::File::open(json_schema.display().to_string()).unwrap();
    let json: serde_json::Value = serde_json::from_reader(json_file).unwrap();
    println!("{}", json.get("databaseEngine").unwrap());
    println!("{}", json.get("databaseName").unwrap());
}