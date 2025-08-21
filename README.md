[![Crates.io version][crate-img]][crate] [![Documentation][docs-img]][docs]

----

### About

**ot_utils** is a Rust library that is designed to concatenate and generate .ot (slice) files for the [Elektron Octatrack](https://www.elektron.se/products/octatrack-mkii/) sampler.

This library was designed so it's easier to group samples into a single .wav file so they can be accessed on the Octatrack via different slices. The goal is to ultimately save slots in the Static and Flex machines (so, for example, multiple kick samples can be grouped into a single slot rather than using a slot per sample).
 
Each audio file added via the ```add_file``` function is appended to a temporary .wav file and a new slice is created for that audio file.
The ```generate_ot_file``` function generates the .ot file and renames the temporary .wav file to it's final name (same as the .ot file).

This library was inspired and based on the incredible [OctaChainer](https://github.com/KaiDrange/OctaChainer) tool created by [Kai Drange](https://github.com/KaiDrange). 

----

### Example

```rust
use std::fs;
use std::path::Path;
use ot_utils::Slicer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let folder_path = "path/to/sample/folder";
    let folder = Path::new(folder_path);

    // Validate directory
    if folder.is_dir() {
        // Create slicer instance
        let mut slicer = Slicer::default();
        slicer.output_folder = folder_path.to_string();
        slicer.output_filename = folder.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("output")
            .to_string();

        // Add .wav files in folder
        for entry in fs::read_dir(folder)? {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) == Some("wav") {
                slicer.add_file(path.to_string_lossy().to_string())?;
            }
        }

        // Generate .ot file
        slicer.generate_ot_file(false)?;
    }

    Ok(())
}
```

---

### Limitations

ot_utils currently only accepts mono, 16-bit, wav files.

----

Created by [@icaroferre](http://instagram.com/icaroferre)  
[spektroaudio.com](http://spektroaudio.com)

[crate]:         https://crates.io/crates/ot_utils
[crate-img]:     https://img.shields.io/crates/v/ot_utils.svg
[docs-img]:      https://img.shields.io/badge/docs-online-blue.svg
[docs]:          https://docs.rs/ot_utils/
