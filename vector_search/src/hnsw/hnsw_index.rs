pub struct HnswIndex {
    pub data: Array2<f32>,
    layers: Vec<Layer>,
    max_edges_per_node: usize,
    ef_construction: usize,
}

impl HnswIndex {
    pub fn new(data: Array2<f32>, m: usize, ef_construction: usize) -> Self {
        HnswIndex {
            data,
            layers: Vec::new(), // Initialize layers as empty or however they should be initialized
            max_edges_per_node: m,
            ef_construction,
        }
    }
}
