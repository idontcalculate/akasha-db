// In loader_trait.rs
use ndarray::Array2;

pub trait DataLoader {
    fn load_data(&self) -> Result<Array2<f32>, String>;
}
