use cryptography::Number;
use cryptography::Point;

pub struct LagrangeInterpolation {}

impl LagrangeInterpolation {
    pub fn new() -> LagrangeInterpolation {
        LagrangeInterpolation {}
    }

    pub fn interpolate(&mut self, points: Vec<Point>, x: Number) -> Number {
        let mut result = Number::from(0 as f64);

        for i in 0..points.len() {
            let mut term = points[i].y;
            for j in 0..points.len() {
                if i != j {
                    term = term * (x - points[j].x) / (points[i].x - points[j].x);
                }
            }
            result = result + term;
        }

        result
    }
}

impl Default for LagrangeInterpolation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lagrange_float_32() {
        let mut lagrange = LagrangeInterpolation::new();
        let points = vec![
            Point {
                x: Number::from(1.0_f32),
                y: Number::from(1.0_f32),
            },
            Point {
                x: Number::from(2.0_f32),
                y: Number::from(2.0_f32),
            },
            Point {
                x: Number::from(3.0_f32),
                y: Number::from(3.0_f32),
            },
        ];
        let result = lagrange.interpolate(points, Number::from(4.0_f32));
        assert_eq!(result, Number::from(4.0_f64));
    }
    #[test]
    fn test_lagrange_float_64() {
        let mut lagrange = LagrangeInterpolation::new();
        let points = vec![
            Point {
                x: Number::from(1.0_f64),
                y: Number::from(1.0_f64),
            },
            Point {
                x: Number::from(2.0_f64),
                y: Number::from(2.0_f64),
            },
            Point {
                x: Number::from(3.0_f64),
                y: Number::from(3.0_f64),
            },
        ];
        let result = lagrange.interpolate(points, Number::from(4.0_f64));
        assert_eq!(result, Number::from(4.0_f64));
    }
}
