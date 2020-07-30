
//! ## About this library
//! 
//! 
//! This library is designed to create .wav and .ot files for the Elektron Octatrack by
//! concatenating other audio samples (.wav) and settings each added file as a slice in the final
//! file.

use std::path::Path;
use std::fs;
use std::io::Write;

extern crate hound;

/// Struct used for the individual slices
pub struct OTSlice {
    pub loop_point: u32,
    pub start_point : u32,
    pub length: u32
}



/// The Slicer struct is the main struct of the library and it's responsable for parsing .wav files and generating the final .wav and .ot files 
pub struct Slicer {
    /// Folder to which the final .ot and .wav files will be generated
    pub output_folder : String,
    /// Name of the final .ot and .wav files (without extension)
    pub output_filename : String,
    /// Sample rate of the audio samples
    pub sample_rate : u32,
    /// Vector of slices
    pub slices : Vec<OTSlice>,
    // List of files to be processed
    pub filelist : Vec<std::path::PathBuf>,
    // Determines if the output file will be stereo or mono (not implemented yet)
    pub stereo : bool,
    max_file_length : usize,
    start_offset : u32,
    /// Tempo / BPM of the final .wav file
    pub tempo : u32

}


impl Slicer {


    /// Creates a new instance of the Slicer struct
    pub fn new () -> Self {
        
        Self {
            slices : Vec::new(),
            filelist : Vec::new(),
            output_folder : "".to_string(),
            output_filename : "output".to_string(),
            sample_rate : 44100,
            start_offset : 0,
            max_file_length : 0,
            stereo : false,
            tempo: 124
        }
    }

    /// Clears out the slice vector and resets sample offset
    pub fn clear(&mut self) {
        self.slices.clear();
        self.start_offset = 0;
        self.sample_rate = 44100;
        self.tempo = 124;
    }

    /// Adds file to the list of files to be processed
    pub fn add_file (&mut self, filepath : String)  -> Result< &'static str, &'static str> {
        let path = std::path::PathBuf::from(filepath.clone());
        println!("Adding file to list: {}", filepath);

        // Define valid sample format
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: self.sample_rate.clone(),
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };


        // Check if file exists
        match path.is_file() {
            true => {
                // Open file
                let mut reader = hound::WavReader::open(filepath).unwrap();
                // Check if file specs are valid
                if reader.spec() == spec {
                    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();

                    // Check if file length is greater than max_file_length (used for creating evenly spaced sample chains)
                    let total_samples = samples.len();
                    if total_samples > self.max_file_length {
                        self.max_file_length = total_samples;
                    }

                    // Add file path to list of files to be processed
                    self.filelist.push(path);
                    Ok("File added succesfully.")
                } else {
                    Err("Invalid file (invalid sample rate / bit rate / channel number)")
                }
                
            },
            false => {
                Err("File not found.")
            }
        }

    }
    
    /// Appends new audio file (.wav) to the concatenated wav file and creates a new slice
    fn process_file (&mut self, filepath : std::path::PathBuf, evenly_spaced : bool) -> Result< &'static str, &'static str> {
        println!("Processing file: {}", filepath.display());

        // Define valid sample format
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: self.sample_rate.clone(),
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let result =  {
            if self.slices.len() < 65 {
            
                // Open file
                let mut reader = hound::WavReader::open(filepath).unwrap();

                // Return aray of samples (i16)
                    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();


                    // Create path for temporary concat file
                    let output_folder_path : &Path = self.output_folder.as_ref();
                    let mut wav_file_name : String = self.output_filename.clone();
                    wav_file_name.push_str(".wav");
                    let temp_file_path = Path::join(output_folder_path, wav_file_name);

                    let slice_len : u32 = match temp_file_path.is_file() {
                        true => {
                            // Append samples if temporary files already exists
                            let temp_wav_file = hound::WavWriter::append(temp_file_path).unwrap();
                            self.fill_wav_file(temp_wav_file, samples, evenly_spaced)

                        },
                        false => {
                            // Create new file (based on specified specs) and add samples
                            let temp_wav_file = hound::WavWriter::create(temp_file_path, spec).unwrap();
                            self.fill_wav_file(temp_wav_file, samples, evenly_spaced)
                        }
                    };

                    // Create new slice and append it to slices vector
                    let new_ot_slice = OTSlice{start_point: self.start_offset, length: slice_len as u32, loop_point: slice_len as u32};
                    self.slices.push(new_ot_slice);

                    // Add sample length to start offset
                    self.start_offset += slice_len;
                    
                    Ok("File succesfully parsed.")
               
            } else {
                Err("No more slice slots available.")
            } 
        };

        result
        
    }

    fn fill_wav_file(&mut self, mut writer : hound::WavWriter<std::io::BufWriter<std::fs::File>>, samples: Vec<i16>, evenly_spaced : bool) -> u32 {
        match evenly_spaced {
            true => {
                for i in 0..self.max_file_length {
                    let mut sample_value : i16 = 0;
                    if i < samples.len() {
                        sample_value = samples[i].clone()
                    }
                    writer.write_sample(sample_value).unwrap();
                };
            },
            false => {
                for i in 0..samples.len() {
                    writer.write_sample(samples[i].clone()).unwrap();
                };
            }
        }
        
        writer.finalize().unwrap();
        samples.len() as u32
    } 

    /// Generates the .ot file for the Octatrack and renames the concat .wav file to the same name as the .ot file
    pub fn generate_ot_file(&mut self, evenly_spaced: bool) -> Result<&'static str, &'static str> {

        println!("Generating Octatrack files...");

        // Remove existing sample chain wav file 
        let output_folder_path : &Path = self.output_folder.as_ref();
        let mut wav_file_name : String = self.output_filename.clone();
        wav_file_name.push_str(".wav");
        let final_wav_file = Path::join(output_folder_path, wav_file_name);
        if final_wav_file.is_file() {
            println!("Removing existing wav file: {}", final_wav_file.display());
            fs::remove_file(final_wav_file).unwrap();
        }

        // Process files in filelist
        for _ in 0..self.filelist.len() {
            let file_path : std::path::PathBuf = self.filelist.pop().unwrap();
            self.process_file(file_path, evenly_spaced).unwrap();
        }


        // OT File header
        let mut file_data : Vec<u8> = vec![0x46,0x4F,0x52,0x4D,0x00,0x00,0x00,0x00,0x44,0x50,0x53,0x31,0x53,0x4D,0x50,0x41, 0x00,0x00,0x00,0x00,0x00,0x02,0x00];
        
        
        let tempo : u32 = self.tempo * 6 * 4;

        // Count the total number of samples 
        let mut total_samples : u32 = 0;
        for i in 0..self.slices.len() {
            total_samples += self.slices[i].length;
        }


        
        println!("Total samples: {}", total_samples);

        // Calculate the number of bars
        let bars_mult : f32 = (124.0 *total_samples as f32) / (self.sample_rate * 60) as f32 + 0.5;
        let bars : u32 = bars_mult as u32 * 25;

        // Add data to the .ot buffer
        file_data = self.push_u32(file_data, tempo); // Tempo
        file_data = self.push_u32(file_data, bars.clone()); // Trimlen
        file_data = self.push_u32(file_data, bars.clone()); // loopLen
        file_data = self.push_u32(file_data, 0); // Stretch
        file_data = self.push_u32(file_data, 0); // Loop
        file_data = self.push_u16(file_data, 48); // Gain
        file_data.push(255); // Quantize
        file_data = self.push_u32(file_data, 0); // trimStart
        file_data = self.push_u32(file_data, total_samples.clone()); // trimEnd
        file_data = self.push_u32(file_data, 0); // loopPoint


        // Add data for each of the slices
        for i in 0..64 {
            if i < self.slices.len() { 
                file_data = self.push_u32(file_data, self.slices[i].start_point);
                file_data = self.push_u32(file_data, self.slices[i].start_point + self.slices[i].length);
                file_data = self.push_u32(file_data, self.slices[i].loop_point);
            } else {
                file_data = self.push_u32(file_data, 0);
                file_data = self.push_u32(file_data, 0);
                file_data = self.push_u32(file_data, 0);
            }
        }

        file_data = self.push_u32(file_data, self.slices.len() as u32); // Slice Count

        println!("Number of slices: {}", self.slices.len());
        
        let mut checksum : u16 = 0;
        
        // Checksum formula (basically add all values except the header)
        let len = file_data.len();
        for i in 16..len {
            checksum += file_data[i] as u16;
        }

        file_data = self.push_u16(file_data, checksum); // Push Checksum


        let output_folder_path : &Path = self.output_folder.as_ref();

        let mut ot_file_name : String = self.output_filename.clone();
        ot_file_name.push_str(".ot");

        println!("\nGenerating Octatrack .ot file: {}", ot_file_name);

        // Remove .ot file if it already exists for some weird reason
        let ot_file_path = Path::join(output_folder_path, ot_file_name.clone());
        if ot_file_path.is_file() {
            fs::remove_file(ot_file_path.clone()).unwrap();
        };
        
        // Create .ot file and write the file_data buffer to the file
        let mut buffer = fs::File::create(ot_file_path).unwrap();
        buffer.write_all(&file_data).unwrap();

        println!("Finished writing Octatrack .ot file.");

        self.clear();

        Ok("Temporary WAV file renamed succesfully.")

    }



    fn push_u32(&mut self, mut vector : Vec<u8>, num : u32) -> Vec<u8> {
        let array = num.to_le_bytes();
        for i in 0..4 {
            vector.push(array[3-i]);
        }
        vector
    }

    fn push_u16(&mut self, mut vector : Vec<u8>, num : u16) -> Vec<u8> {
        let array = num.to_le_bytes();
        vector.push(array[1]);
        vector.push(array[0]);
        vector
    }
}
