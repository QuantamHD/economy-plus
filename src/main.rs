extern crate walkdir;
extern crate mustache;
mod templates;

#[macro_use]
extern crate clap;

use walkdir::WalkDir;
use std::path::Path;
use std::fs;
use std::str;
use templates::mustache_template::compile::compile_to_file;

/// Compiles every file in mustache and outputs the result as a folder inside
/// the container folder.
///
/// # Examples
///
/// ```
/// compile_folder(Path::new("example_project"), Path::new("output_project"));
/// ```
fn compile_folder(input_folder : &Path, output_container_folder : &Path, name : &String) {
    for entry in WalkDir::new(input_folder) {
        let entry = entry.unwrap();
        if !entry.path().is_dir() { // If it's a file, let's compile
            let input = entry.path().as_os_str();
            compile_to_file(entry.path(),
                output_container_folder.join(input).as_path(),
                name);
        } else { // If a directory, create that directory
            let path_name = output_container_folder.join(entry.path());
            let mustached_folder : &str = path_name.to_str().unwrap();

            let template = mustache::compile_str(mustached_folder).unwrap();
            let data = mustache::MapBuilder::new()
                .insert_str("name", name)
                .build();

            let mut buffer : Vec<u8> = Vec::new();
            template.render_data(&mut buffer, &data).unwrap();

            fs::create_dir_all(Path::new(&String::from_utf8(buffer).unwrap()));
        }
    }
}

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
