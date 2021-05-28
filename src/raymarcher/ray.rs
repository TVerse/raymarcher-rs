use crate::primitives::UnitVec3;
use crate::scene::scenemap::material::MaterialIndex;
use crate::scene::scenemap::sdf::Sdf;
use crate::{Point3, Vec3};
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub struct Ray {
    origin: Point3,
    direction: UnitVec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: UnitVec3) -> Self {
        Self { origin, direction }
    }

    pub fn new_unnormalized(origin: Point3, unnormalized_direction: Vec3) -> Self {
        Self {
            origin,
            direction: unnormalized_direction.unit(),
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &UnitVec3 {
        &self.direction
    }

    pub fn find_target(
        &self,
        find_target_settings: &FindTargetSettings,
        sdf: &dyn Sdf,
    ) -> Option<FindTargetResult> {
        DepthIterator::new(sdf, &self, find_target_settings.t_min)
            .into_iter()
            .take_while(|DepthResult { total_depth, .. }| total_depth < &find_target_settings.t_max)
            .find(|DepthResult { dist, .. }| *dist < find_target_settings.epsilon)
            .map(|dr| FindTargetResult {
                point: dr.point,
                material_index: dr.mat_idx,
            })
    }

    pub fn soft_shadow(
        &self,
        find_target_settings: &FindTargetSettings,
        sdf: &dyn Sdf,
        k: f64,
    ) -> f64 {
        // Correct t_max a bit so we don't always find a hit with the actual target.
        let corrected_t_max =
            find_target_settings.t_max * (1.0 - 3.0 * find_target_settings.epsilon);

        DepthIterator::new(sdf, &self, find_target_settings.t_min)
            .into_iter()
            .take_while(|DepthResult { total_depth, .. }| *total_depth < corrected_t_max)
            .fold_while(1.0, |acc: f64, sr| {
                if sr.dist < find_target_settings.epsilon {
                    Done(acc)
                } else {
                    Continue(acc.min(k * sr.dist / sr.total_depth))
                }
            })
            .into_inner()
    }
}

#[derive(Debug, Clone)]
pub struct FindTargetSettings {
    pub t_min: f64,
    pub t_max: f64,
    pub epsilon: f64,
}

impl FindTargetSettings {
    pub fn new(t_min: f64, t_max: f64, epsilon: f64) -> Self {
        Self {
            t_min,
            t_max,
            epsilon,
        }
    }
}

pub struct FindTargetResult {
    pub point: Point3,
    pub material_index: Option<MaterialIndex>,
}

struct DepthIterator<'a> {
    sdf: &'a dyn Sdf,
    r: &'a Ray,
    prev_dist: f64,
    cur_depth: f64,
}

impl<'a> DepthIterator<'a> {
    fn new(sdf: &'a dyn Sdf, r: &'a Ray, t_min: f64) -> Self {
        Self {
            sdf,
            r,
            prev_dist: 0.0,
            cur_depth: t_min,
        }
    }
}

#[derive(Debug)]
struct DepthResult {
    point: Point3,
    dist: f64,
    mat_idx: Option<MaterialIndex>,
    total_depth: f64,
}

impl<'a> Iterator for DepthIterator<'a> {
    type Item = DepthResult;

    fn next(&mut self) -> Option<Self::Item> {
        let total_depth = self.prev_dist + self.cur_depth;
        let point = Point3(self.r.origin.as_ref() + self.r.direction.as_ref() * total_depth);
        let (dist, mat_idx) = self.sdf.value_at(&point);

        self.cur_depth = total_depth;
        self.prev_dist = dist;

        Some(Self::Item {
            point,
            dist,
            mat_idx,
            total_depth,
        })
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::black_box;
    use test::Bencher;

    use float_cmp::ApproxEq;

    use crate::test_constants::MARGIN;

    use super::*;
    use crate::scene::scenemap::sdf::primitives::{NegY, Sphere};

    #[test]
    fn evaluate_sdf_hit() {
        let ray = Ray::new(
            Point3::new(-10.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0).unit(),
        );

        let settings = FindTargetSettings::new(0.0, 100.0, 1e-5);

        let sdf = Sphere::default();

        let ftr = ray.find_target(&settings, &sdf).unwrap();

        assert!(ftr.point.approx_eq(&Point3::new(-1.0, 0.0, 0.0), MARGIN));
    }

    #[test]
    fn evaluate_sdf_miss() {
        let ray = Ray::new(
            Point3::new(-10.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0).unit(),
        );

        let settings = FindTargetSettings::new(0.0, 100.0, 1e-5);

        let sdf = Sphere::default();

        assert!(ray.find_target(&settings, &sdf).is_none());
    }

    #[bench]
    fn target_hit(b: &mut Bencher) {
        let ray = Ray::new_unnormalized(Point3::new(-10.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));

        let settings = FindTargetSettings::new(0.0, 100.0, 1e-5);

        let sdf = Sphere::default();

        b.iter(|| {
            let find_target_settings = black_box(&settings);
            let sdf = black_box(&sdf);
            ray.find_target(find_target_settings, sdf)
        })
    }

    #[bench]
    fn target_miss(b: &mut Bencher) {
        let ray = Ray::new_unnormalized(Point3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0));

        let settings = FindTargetSettings::new(0.0, 100.0, 1e-5);

        let sdf = NegY;

        b.iter(|| {
            let find_target_settings = black_box(&settings);
            let sdf = black_box(&sdf);
            ray.find_target(find_target_settings, sdf);
        })
    }
}
