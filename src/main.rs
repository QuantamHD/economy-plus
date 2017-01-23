extern crate mustache;
extern crate ego_tree;
extern crate walkdir;

use std::fs::File;
use mustache::MapBuilder;
use walkdir::WalkDir;
use std::ffi::OsStr;
use std::path::Path;
use std::fs;

fn compile_mustache(input : &Path, output : &Path) {
    let template = mustache::compile_path(input).unwrap();

    let data = MapBuilder::new()
        .insert_str("name", "Evan")
        .build();

    let mut buffer = File::create(output).unwrap();
    template.render_data(&mut buffer, &data).unwrap();
}

fn move_over() {
    let project_folder_name = "example_project";
    let output_folder = Path::new("output_project");

    for entry in WalkDir::new(project_folder_name) {
        let entry = entry.unwrap();
        if !entry.path().is_dir() {
            let input = entry.path().as_os_str();
            compile_mustache(entry.path(), output_folder.join(input).as_path());
        } else {
            println!("{:?}", output_folder.join(entry.path()));
            fs::create_dir_all(output_folder.join(entry.path()));
        }

    }
}

fn main() {
    move_over();
}
