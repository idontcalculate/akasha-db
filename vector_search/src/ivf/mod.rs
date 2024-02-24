// ivf/mod.rs
// ivf/mod.rs
use ndarray::{Array1, Array2, Axis};
use rand::SeedableRng; // for a reproducible RNG
use rand_distr::{Distribution, Normal};

pub struct IvfIndex {
    pub data: Array2<f32>,       // Vector storage
    pub centroids: Array2<f32>,  // Centroid storage
    pub assignments: Vec<usize>, // Assignments of vectors to centroids
    // Other properties like inverted lists can be added here
}

impl IvfIndex {
    pub fn new(data: Array2<f32>) -> Self {
        // Initialization logic with random centroids
        // Normally, you would use k-means or another method to find centroids
        let n_centroids = 10; // for example
        let mut rng = rand::rngs::StdRng::from_entropy();
        let dist = Normal::new(0.0, 1.0).unwrap();
        let centroids = Array2::from_shape_fn((n_centroids, data.ncols()), |_| dist.sample(&mut rng));

        IvfIndex {
            data,
            centroids,
            assignments: vec![],
        }
    }

    pub fn add_items(&mut self, new_data: Array2<f32>) {
        // Add items to the index
        // Placeholder implementation
        // Here you would also need to assign the new items to centroids and update your inverted lists
        self.data = new_data;
    }

    // Other methods would go here...
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_new_ivf_index() {
        // Create a small dataset for testing
        let data = array![[1.0, 2.0], [3.0, 4.0]];
        let index = IvfIndex::new(data.clone());

        // The number of centroids should be 10 as specified in `new`
        assert_eq!(index.centroids.nrows(), 10);
        assert_eq!(index.centroids.ncols(), data.ncols());
        assert_eq!(index.data, data);
        assert!(index.assignments.is_empty());
    }

    #[test]
    fn test_add_items() {
        // Assume we have an existing index with some data
        let initial_data = array![[1.0, 2.0], [3.0, 4.0]];
        let mut index = IvfIndex::new(initial_data.clone());

        // New data to be added
        let new_data = array![[5.0, 6.0], [7.0, 8.0]];
        index.add_items(new_data.clone());

        // Verify the data was added
        assert_eq!(index.data, new_data);

        // Further tests should verify that centroids and assignments are updated accordingly
    }

    // Additional tests for `init_centroids`, `assign_vectors`, `build_index`, and `search` would go here...
}


