#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn mul(self, scalar: f32) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn normalize(self) -> Vec2 {
        let magnitude = (self.x.powi(2) + self.y.powi(2)).sqrt();
        if magnitude != 0.0 {
            Vec2 {
                x: self.x / magnitude,
                y: self.y / magnitude,
            }
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive_vectors() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        let result = v1.add(v2);
        let expected = Vec2::new(4.0, 6.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_negative_vectors() {
        let v1 = Vec2::new(-1.0, -2.0);
        let v2 = Vec2::new(-3.0, -4.0);
        let result = v1.add(v2);
        let expected = Vec2::new(-4.0, -6.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_mixed_vectors() {
        let v1 = Vec2::new(5.0, -3.0);
        let v2 = Vec2::new(-2.0, 4.0);
        let result = v1.add(v2);
        let expected = Vec2::new(3.0, 1.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mul_positive_scalar() {
        let v = Vec2::new(2.0, 3.0);
        let scalar = 4.0;
        let result = v.mul(scalar);
        let expected = Vec2::new(8.0, 12.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mul_negative_scalar() {
        let v = Vec2::new(2.0, -3.0);
        let scalar = -2.0;
        let result = v.mul(scalar);
        let expected = Vec2::new(-4.0, 6.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mul_zero_scalar() {
        let v = Vec2::new(5.0, -7.0);
        let scalar = 0.0;
        let result = v.mul(scalar);
        let expected = Vec2::new(0.0, 0.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_normalize_positive_vector() {
        let v = Vec2::new(3.0, 4.0);
        let result = v.normalize();
        let expected = Vec2::new(0.6, 0.8);
        assert!((result.x - expected.x).abs() < 1e-6);
        assert!((result.y - expected.y).abs() < 1e-6);
    }

    #[test]
    fn test_normalize_negative_vector() {
        let v = Vec2::new(-3.0, -4.0);
        let result = v.normalize();
        let expected = Vec2::new(-0.6, -0.8);
        assert!((result.x - expected.x).abs() < 1e-6);
        assert!((result.y - expected.y).abs() < 1e-6);
    }

    #[test]
    fn test_normalize_zero_vector() {
        let v = Vec2::new(0.0, 0.0);
        let result = v.normalize();
        let expected = Vec2::new(0.0, 0.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_normalize_unit_vector() {
        let v = Vec2::new(1.0, 0.0);
        let result = v.normalize();
        let expected = Vec2::new(1.0, 0.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_combined_operations() {
        let v1 = Vec2::new(1.0, 1.0);
        let v2 = Vec2::new(2.0, 3.0);
        let added = v1.add(v2);
        let multiplied = added.mul(2.0);
        let normalized = multiplied.normalize();
        let magnitude = (normalized.x.powi(2) + normalized.y.powi(2)).sqrt();

        // The normalized vector should have a magnitude of 1.0
        assert!((magnitude - 1.0).abs() < 1e-6);
    }
}
