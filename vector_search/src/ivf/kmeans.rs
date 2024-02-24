// src/ivf/kmeans.rs

// Define your k-means clustering implementation here
// kmeans.rs

use ndarray::{Array1, Array2, Axis};
use rand::SeedableRng; // for a reproducible RNG
use rand::rngs::StdRng;
use rand_distr::{Uniform};

pub struct KMeans {
    pub centroids: Array2<f32>, // Centroid storage
}

impl KMeans {
    // Initialize KMeans with randomly selected centroids from the data
    pub fn new(k: usize, data: &Array2<f32>) -> Self {
        let mut rng = StdRng::from_entropy();
        let uniform = Uniform::new(0, data.nrows());
        let mut centroids = Array2::zeros((k, data.ncols()));
        for mut row in centroids.genrows_mut(Axis(0)) {
            let sample_index = uniform.sample(&mut rng);
            row.assign(&data.slice(s![sample_index, ..]));
        }

        KMeans { centroids }
    }

    // Assigns each vector in the dataset to the nearest centroid
    fn assign_vectors(&self, data: &Array2<f32>) -> Vec<usize> {
        data.outer_iter().map(|vector| {
            self.centroids.outer_iter().enumerate().map(|(index, centroid)| {
                // Compute the squared Euclidean distance to avoid sqrt for performance
                (index, (vector - &centroid).mapv(|x| x.powi(2)).sum())
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(index, _)| index)
            .unwrap()
        })
        .collect()
    }

    // Updates centroids based on the current assignments
    fn update_centroids(&mut self, data: &Array2<f32>, assignments: &[usize]) {
        let k = self.centroids.nrows();
        let mut counts = vec![0; k];
        let mut sums = Array2::zeros((k, data.ncols()));
        
        for (assignment, vector) in assignments.iter().zip(data.outer_iter()) {
            sums.row_mut(*assignment).add_assign(&vector);
            counts[*assignment] += 1;
        }

        for (index, mut centroid) in self.centroids.outer_iter_mut().enumerate() {
            if counts[index] > 0 {
                centroid.assign(&(&sums.row(index) / counts[index] as f32));
            }
        }
    }

    // Performs the K-Means clustering algorithm
    pub fn fit(&mut self, data: &Array2<f32>, max_iterations: usize) {
        for _ in 0..max_iterations {
            let assignments = self.assign_vectors(data);
            let previous_centroids = self.centroids.clone();
            self.update_centroids(data, &assignments);

            // Check for convergence
            if previous_centroids == self.centroids {
                break; // Centroids have stabilized
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_new_kmeans() {
        let data = array![[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]];
        let k = 2;
        let kmeans = KMeans::new(k, &data);
        assert_eq!(kmeans.centroids.nrows(), k);
        assert_eq!(kmeans.centroids.ncols(), data.ncols());
    }

    #[test]
    fn test_fit_kmeans() {
        let data = array![[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]];
        let k = 2;
        let mut kmeans = KMeans::new(k, &data);
        kmeans.fit(&data, 100);
        // This test should ideally check for convergence or correctness of the centroids
        // For simplicity, it's left as is, but consider adding more rigorous checks
    }
}


