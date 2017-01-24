extern crate mustache;
extern crate walkdir;


use std::fs::File;
use self::mustache::MapBuilder;
use std::path::Path;
use std::result::Result;
use self::walkdir::WalkDir;
use std::fs;



/// Compiles a mustache file and then spits out the output to a file
///
/// # Examples
///
/// ```
/// compile_to_file(Path::new("input.txt"), Path::new("output.txt");)
/// ```
pub fn compile_to_file(input : &Path, output : &Path, name : &String) {
    let template = mustache::compile_path(input).unwrap();

    let data = MapBuilder::new()
        .insert_str("name", name)
        .build();

    let mut buffer = File::create(output).unwrap();
    template.render_data(&mut buffer, &data).unwrap();
}

/// Compiles a string and spits back the output of that string
///
/// # Examples
///
/// ```
///
/// ```
///
pub fn compile_string(input : &String, name: &String) -> String {

    let template = mustache::compile_str(input).unwrap();
    let data = mustache::MapBuilder::new()
        .insert_str("name", name)
        .build();

    let mut buffer : Vec<u8> = Vec::new();
    template.render_data(&mut buffer, &data).unwrap();

    String::from_utf8(buffer).unwrap()
}

/// Compiles every file in mustache and outputs the result as a folder inside
/// the container folder.
///
/// # Examples
///
/// ```
/// compile_folder(Path::new("example_project"), Path::new("output_project"));
/// ```
pub fn compile_folder(input_folder : &Path, output_container_folder : &Path, name : &String) {
    for entry in WalkDir::new(input_folder) {
        let entry = entry.unwrap();

        if !entry.path().is_dir() {

            // If it's a file, let's compile
            let input = entry.path().as_os_str();
            compile_to_file(entry.path(),
                output_container_folder.join(input).as_path(),
                name);
        } else {

            // If a directory, create that directory
            let path_name = output_container_folder.join(entry.path());
            let mustached_folder : &str = path_name.to_str().unwrap();
            let compiled_path_name = compile_string(&String::from(mustached_folder), name);
            fs::create_dir_all(Path::new(&compiled_path_name)); // TODO Error check
        }
    }
}
