use std::result::Result;

pub trait DataLoader {
    fn load_data(&self) -> Result<Array2<f32>, String>;
}
