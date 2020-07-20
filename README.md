[![Crates.io version][crate-img]][crate]

[![Documentation][docs-img]][docs]


# About

**ot_utils** is a Rust library that is designed to concatenate and generate .ot (slice) files for the [Elektron Octatrack](https://www.elektron.se/products/octatrack-mkii/) sampler.

This library was designed so it's easier to group samples into a single .wav file so they can be accessed on the Octatrack via different slices. The goal is to ultimately save slots in the Static and Flex machines (so, for example, multiple kick samples can be grouped into a single slot rather than using a slot per sample).
 
Each audio file added via the ```add_file``` function is appended to a temporary .wav file and a new slice is created for that audio file.
The ```generate_ot_file``` function generates the .ot file and renames the temporary .wav file to it's final name (same as the .ot file).

This library was inspired and based on the incredible [OctaChainer](https://github.com/KaiDrange/OctaChainer) tool created by [Kai Drange](https://github.com/KaiDrange). 

## Example

```rust

extern crate ot_utils;

use ot_utils::Slicer;
 

let folder_path = "path/to/sample/folder".to_string();
let check_file: &Path = &folder_path.as_ref();

// Validate directory
if check_file.is_dir() {

    // Get list of files
    let paths = fs::read_dir(&folder_path).unwrap();

    // Set output folder
    OT_Slicer.output_folder = folder_path.clone();

    // Set final .ot and .wav filename
    OT_Slicer.output_filename = check_file.file_name().unwrap().to_str().unwrap().to_string();


    for path in paths {
        // Get file info (path, name, and extension)
        let file_path = path.unwrap().path();
        let file_name = &file_path.file_name();
        let file_ext = match &file_path.extension(){
            &Some(x) => x.to_str().unwrap(),
            &None => " "
        };

        if file_ext == "wav" {
            OT_Slicer.add_file(new_file);
        } 
    }
}
        
OT_Slicer.generate_ot_file();

```

## Limitations

ot_utils currently only accepts mono, 16-bit, wav files.


----

Created by [@icaroferre](http://instagram.com/icaroferre)
spektroaudio.com

[crate]:         https://crates.io/crates/ot_utils
[crate-img]:     https://img.shields.io/crates/v/ot_utils.svg
[docs-img]:      https://img.shields.io/badge/docs-online-blue.svg
[docs]:          https://docs.rs/ot_utils/
