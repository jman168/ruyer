use glam::{Vec3, vec3};
use rand::prelude::*;

/// Samples a cosine-weighted point on a unit hemisphere such that it's PDF is cos(theta) / pi.
///
/// This is very useful for performing Monte Carlo integration of the rendering equation.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CosineWeightedUnitHemisphere {}

impl CosineWeightedUnitHemisphere {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Distribution<Vec3> for CosineWeightedUnitHemisphere {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        loop {
            let x = rng.random_range(-1.0f32..1.0f32);
            let y = rng.random_range(-1.0f32..1.0f32);
            let d = x * x + y * y;

            if d > 1.0 {
                continue;
            }

            return vec3(x, y, (1.0 - d).sqrt());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_abs_diff_eq;
    use glam::vec3;

    #[test]
    fn test_cosine_hemisphere() {
        let dist = CosineWeightedUnitHemisphere::default();
        let mut x = Vec3::ZERO;
        let mut rng = rand::rng();

        for _ in 0..1_000_000 {
            x += rng.sample(&dist);
        }
        x /= 1_000_000.0;

        assert_abs_diff_eq!(x, vec3(0.0, 0.0, 2.0 / 3.0), epsilon = 1e-2);
    }
}
