#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Point {
    inner: Vec<f32>,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Point {}

impl Point {
    pub fn new(inner: Vec<f32>) -> Self {
        Point { inner }
    }

    pub fn get(&self) -> Vec<f32> {
        self.inner.clone()
    }

    pub fn div(&self, n: usize) -> Point {
        if n == 0 {
            return self.clone();
        }
        let mut result = self.inner.clone();
        for value in result.iter_mut() {
            *value /= n as f32;
        }
        Point::new(result)
    }

    pub fn distance_from(&self, other_point: &Point) -> f32 {
        if self.inner.len() != other_point.inner.len() {
            return f32::MAX;
        }
        let mut sum = 0.0;
        for i in 0..self.inner.len() {
            let diff = self.inner[i] - other_point.inner[i];
            sum += diff * diff;
        }
        sum.sqrt()
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other_point: Self) -> Self::Output {
        let mut result = self.inner.clone();
        for (i, &value) in other_point.inner.iter().enumerate() {
            if i < result.len() {
                result[i] += value;
            } else {
                result.push(value);
            }
        }
        Point::new(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn should_correctly_compute_euclidian_distance() {
        let point = Point::new(vec![2.0, 8.0, 1.0, 4.3]);
        let point2 = Point::new(vec![1.0, -5.0, 0.0, 3.0]);

        assert_eq!(point.distance_from(&point2), 13.141157)
    }

    #[test]
    fn should_correctly_compute_sum() {
        let point = Point::new(vec![2.0, 8.0, 1.0, 4.3]);
        let point2 = Point::new(vec![1.0, -5.0, 0.0, 3.0]);

        assert_eq!(point + point2, Point::new(vec![3.0, 3.0, 1.0, 7.3]))
    }

    #[test]
    fn should_correctly_divide() {
        let point = Point::new(vec![2.0, 8.0, 1.0, 4.3]);
        assert_eq!(point.div(2), Point::new(vec![1.0, 4.0, 0.5, 2.15]))
    }
}
