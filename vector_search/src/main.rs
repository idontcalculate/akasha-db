// At the top of main.rs
mod loaders;
mod hnsw;
mod ivf;

use loaders::{FileLoader, DataLoader}; // This will work if DataLoader is re-exported from loaders/mod.rs
use hnsw::HnswIndex;

fn main() {
    let file_loader = FileLoader {
        file_path: "path/to/your/data.csv".to_string(),
    };

    match file_loader.load_data() {
        Ok(data) => {
            let m: usize = 16; // Example parameter for max_edges_per_node
            let ef_construction: usize = 200; // Example parameter for ef_construction
            let hnsw_index = HnswIndex::new(data, m, ef_construction);

            // hnsw_index can now be used for further operations
        },
        Err(e) => {
            eprintln!("Failed to load data: {}", e);
        },
    }
}


