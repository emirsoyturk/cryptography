use cryptography::Point;
use cryptography::Number;

pub struct LagrangeInterpolation {
    
}

impl LagrangeInterpolation {
    pub fn new() -> LagrangeInterpolation {
        LagrangeInterpolation{}
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lagrange_float_32() {
        let mut lagrange = LagrangeInterpolation::new();
        let points = vec![Point{x: Number::from(1.0 as f32), y: Number::from(1.0 as f32)}, Point{x: Number::from(2.0 as f32), y: Number::from(2.0 as f32)}, Point{x: Number::from(3.0 as f32), y: Number::from(3.0 as f32)}];
        let result = lagrange.interpolate(points, Number::from(4.0 as f32));
        assert_eq!(result, Number::from(4.0 as f64));
    }
    #[test]
    fn test_lagrange_float_64() {
        let mut lagrange = LagrangeInterpolation::new();
        let points = vec![Point{x: Number::from(1.0 as f64), y: Number::from(1.0 as f64)}, Point{x: Number::from(2.0 as f64), y: Number::from(2.0 as f64)}, Point{x: Number::from(3.0 as f64), y: Number::from(3.0 as f64)}];
        let result = lagrange.interpolate(points, Number::from(4.0 as f64));
        assert_eq!(result, Number::from(4.0 as f64));
    }
}