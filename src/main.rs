extern crate mustache;
extern crate ego_tree;
extern crate walkdir;

use std::fs::File;
use mustache::MapBuilder;
use walkdir::WalkDir;
use std::path::Path;
use std::fs;

/**
 * Compiles a mustache file and then spits out the output to a file
 */
fn compile_mustache_file(input : &Path, output : &Path) {
    let template = mustache::compile_path(input).unwrap();

    let data = MapBuilder::new()
        .insert_str("name", "Evan")
        .build();

    let mut buffer = File::create(output).unwrap();
    template.render_data(&mut buffer, &data).unwrap();
}

/**
 * Compiles every file in mustache and outputs the result as a folder inside
 * the container folder.
 */
fn compile_folder(input_folder : &Path, output_container_folder : &Path) {
    for entry in WalkDir::new(input_folder) {
        let entry = entry.unwrap();
        if !entry.path().is_dir() {
            let input = entry.path().as_os_str();
            compile_mustache_file(entry.path(), output_container_folder.join(input).as_path());
        } else {
            fs::create_dir_all(output_container_folder.join(entry.path()));
        }
    }
}

fn main() {



    compile_folder(Path::new("example_project"), Path::new("output_project"));
}
