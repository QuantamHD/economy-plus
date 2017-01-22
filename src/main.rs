extern crate mustache;

use std::io;
use mustache::MapBuilder;

fn main() {
    let template = mustache::compile_path("example.md").unwrap();

    let data = MapBuilder::new()
        .insert_str("name", "Evan")
        .build();

    template.render_data(&mut io::stdout(), &data).unwrap();
    println!("");
}
