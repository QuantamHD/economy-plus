extern crate mustache;

use std::io::*;
use std::fs::File;
use mustache::MapBuilder;

fn main() {
    let template = mustache::compile_path("example.md").unwrap();

    let data = MapBuilder::new()
        .insert_str("name", "Evan")
        .build();

    let mut buffer = File::create("foo.txt").unwrap();
    template.render_data(&mut buffer, &data).unwrap();
    println!("");

}
