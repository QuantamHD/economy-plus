extern crate mustache;

use std::fs::File;
use self::mustache::MapBuilder;
use std::path::Path;


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
