extern crate mustache;
mod templates;

#[macro_use]
extern crate clap;

use std::path::Path;
use std::str;
use templates::mustache_template::compile::compile_folder;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.0.1")
        (author: "Ethan M. and Evan C.")
        (about: "Does awesome things")
        (@arg NAME: +required "Sets the input file to use")
    ).get_matches();

    let name = matches.value_of("NAME").unwrap();

    println!("The name of the command {}", name);

    compile_folder(Path::new("example_project"), Path::new("output_project"), &name.to_string());
}
