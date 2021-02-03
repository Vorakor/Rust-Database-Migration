use std::fs;
use std::path::Path;

fn main() {
    create_schema("old_schema");
}


fn create_schema(schema: &str) {
    // This function is meant to choose a schema and create it in the database.
    let paths = fs::read_dir(&Path::new(& "./database/schemas/")).unwrap();

    let names = paths.filter_map(|entry| {
        entry.ok().and_then(|e|
            e.path().file_name().and_then(|n| n.to_str().map(|s| String::from(s)))
        )
    }).collect::<Vec<String>>();
    // So this here allows us to reuse the memory allocation from schema for the new comp_schema
    // The mut keyword means mutable or in otherwords it is a signal that this variable will change.
    let mut comp_schema: String = schema.to_owned();
    comp_schema.push_str(".json");
    for file in &names {
        if file.to_string() == comp_schema {
            println!("{}", file);
        }
    }
}