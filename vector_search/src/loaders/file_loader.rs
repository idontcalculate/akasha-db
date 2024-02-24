use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use csv::ReaderBuilder;
use ndarray::Array2;
use ndarray_csv::Array2Reader;

pub struct FileLoader {
    pub file_path: PathBuf,
}

impl FileLoader {
    pub fn load_data(&self) -> Result<Array2<f32>, Box<dyn Error>> {
        // Open the file
        let file = File::open(&self.file_path)?;

        // Create a CSV reader with default configuration
        let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);

        // Read the CSV file into an Array2 of f32
        let array: Array2<f32> = reader.deserialize_array2_dynamic()?;
        
        Ok(array)
    }
}



