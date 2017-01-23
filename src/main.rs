extern crate mustache;
extern crate ego_tree;
extern crate walkdir;

#[macro_use]
extern crate clap;


use std::fs::File;
use mustache::MapBuilder;
use walkdir::WalkDir;
use std::path::Path;
use std::fs;

/// Compiles a mustache file and then spits out the output to a file
///
/// # Examples
///
/// ```
/// compile_mustache_file(Path::new("input.txt"), Path::new("output.txt");)
/// ```
fn compile_mustache_file(input : &Path, output : &Path) {
    let template = mustache::compile_path(input).unwrap();

    let data = MapBuilder::new()
        .insert_str("name", "Evan")
        .build();

    let mut buffer = File::create(output).unwrap();
    template.render_data(&mut buffer, &data).unwrap();
}

/// Compiles every file in mustache and outputs the result as a folder inside
/// the container folder.
///
/// # Examples
/// 
/// ```
/// compile_folder(Path::new("example_project"), Path::new("output_project"));
/// ```
fn compile_folder(input_folder : &Path, output_container_folder : &Path) {
    for entry in WalkDir::new(input_folder) {
        let entry = entry.unwrap();
        if !entry.path().is_dir() { // If it's a file, let's compile
            let input = entry.path().as_os_str();
            compile_mustache_file(entry.path(), output_container_folder.join(input).as_path());
        } else { // If a directory, create that directory
            fs::create_dir_all(output_container_folder.join(entry.path()));
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


    compile_folder(Path::new("example_project"), Path::new("output_project"));
}
