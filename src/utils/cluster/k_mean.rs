use super::error;
use crate::utils::cluster::point;

#[derive(Debug, Clone, PartialEq)]
pub struct KMeanOutput {
    pub centroids: Vec<point::Point>,
    pub labels: Vec<usize>,
}

fn init_centroids(points: &Vec<point::Point>, k: usize) -> Vec<point::Point> {
    let mut centroids = vec![];
    for _ in 0..k {
        let target_i: u32 = rand::random::<u32>() % points.len() as u32;
        centroids.push(points[target_i as usize].clone());
    }
    centroids
}

// compute the mean of each dimension to get the new centroid
fn calculate_centroid(cluster: &Vec<point::Point>) -> point::Point {
    let mut centroid = point::Point::default();
    for point in cluster {
        centroid = centroid + point.clone();
    }
    centroid.div(cluster.len())
}

/// Compute k-centroid from the input data using the k_mean algorithm.
pub fn k_mean(
    points: Vec<point::Point>,
    k: usize,
    iterations: usize,
) -> Result<KMeanOutput, error::Error> {
    if k == 0 {
        return Err(error::Error::InvalidClusterNumber);
    } else if iterations == 0 {
        return Err(error::Error::InvalidIterations);
    } else if points.len() < k {
        return Err(error::Error::NotEnoughPoints);
    }

    let mut centroids: Vec<point::Point> = init_centroids(&points, k);
    let mut labels: Vec<usize> = vec![0; points.len()];
    let mut converged: bool = false;

    while !converged {
        let mut clusters = vec![Vec::new(); k];

        for i in 0..points.len() {
            let point = &points[i];
            let mut closest_index = 0;
            let mut min_distance = point.distance_from(&centroids[0]);
            for j in 1..k {
                let distance = point.distance_from(&centroids[j]);
                if distance < min_distance {
                    closest_index = j;
                    min_distance = distance;
                }
            }
            clusters[closest_index].push(point.clone());
            labels[i] = closest_index;
        }

        let mut new_centroids = vec![];
        for i in 0..k {
            new_centroids.push(calculate_centroid(&clusters[i]))
        }

        if new_centroids == centroids {
            converged = true;
        } else {
            centroids = new_centroids;
        }
    }

    Ok(KMeanOutput { centroids, labels })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_mean() {
        let points = vec![
            point::Point::new(vec![1.0, 2.0]),
            point::Point::new(vec![1.5, 1.8]),
            point::Point::new(vec![5.0, 8.0]),
            point::Point::new(vec![8.0, 8.0]),
            point::Point::new(vec![1.0, 0.6]),
            point::Point::new(vec![9.0, 11.0]),
        ];
        let k = 2;
        let iterations = 1000;
        let kmean = k_mean(points, k, iterations).expect("Should not return an error");

        let expected = KMeanOutput {
            centroids: vec![
                point::Point::new(vec![1.1666666, 1.4666667]),
                point::Point::new(vec![7.3333335, 9.0]),
            ],
            labels: vec![0, 0, 1, 1, 0, 1],
        };
        assert_eq!(kmean, expected);
    }

    #[test]
    fn should_throw_error_if_k_is_0() {
        let points = vec![point::Point::new(vec![1.0, 2.0])];
        let k = 0;
        let iterations = 10;
        let kmean = k_mean(points, k, iterations);
        assert_eq!(kmean.unwrap_err(), error::Error::InvalidClusterNumber);
    }

    #[test]
    fn should_throw_error_if_iterations_is_0() {
        let points = vec![point::Point::new(vec![1.0, 2.0])];
        let k = 2;
        let iterations = 0;
        let kmean = k_mean(points, k, iterations);
        assert_eq!(kmean.unwrap_err(), error::Error::InvalidIterations);
    }

    #[test]
    fn should_throw_error_if_points_is_less_than_k() {
        let points = vec![point::Point::new(vec![1.0, 2.0])];
        let k = 2;
        let iterations = 10;
        let kmean = k_mean(points, k, iterations);
        assert_eq!(kmean.unwrap_err(), error::Error::NotEnoughPoints);
    }
}
