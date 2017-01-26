extern crate mustache;
mod templates;

#[macro_use]
extern crate clap;

use std::path::Path;
use std::str;
use std::process::exit;
use templates::mustache_template::compile::compile_folder;

fn plus_templates_exists() -> bool {
    Path::new("plus_templates").exists()
}

fn template_exists(name : &String) -> bool {
    Path::new("plus_templates").join(Path::new(&name)).as_path().exists()
}

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.0.1")
        (author: "Ethan M. and Evan C.")
        (about: "Does awesome things")
        (@arg TEMPLATE: +required "Sets the template to use")
        (@arg NAME: +required "Sets the input file to use")
    ).get_matches();

    // Guard against no plus_template
    if !plus_templates_exists() {
        println!("I can't find a plus_templates folder! Are you in the Root directory?");
        exit(1);
    }

    let template = matches.value_of("TEMPLATE").unwrap();
    let name = matches.value_of("NAME").unwrap();
    if template_exists(&String::from(template)) {
        compile_folder(Path::new("plus_templates"), Path::new("output_project"), &name.to_string());
    } else {
        println!("I can't find a folder named '{}' in plus_templates!", template);
    }
}
