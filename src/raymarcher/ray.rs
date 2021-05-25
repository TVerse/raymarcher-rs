use crate::scene::scenemap::material::MaterialIndex;
use crate::scene::scenemap::sdf::Sdf;
use crate::{Point3, Vec3};
use num_traits::Float;

pub struct Ray<F> {
    pub origin: Point3<F>,
    pub direction: Vec3<F>,
}

impl<F: Float> Ray<F> {
    pub fn find_target(
        &self,
        find_target_settings: &FindTargetSettings<F>,
        sdf: &dyn Sdf<F>,
    ) -> Option<(Point3<F>, Option<MaterialIndex>)> {
        DepthIterator {
            sdf,
            r: &self,
            cur_depth: find_target_settings.t_min,
        }
        .into_iter()
        .take(find_target_settings.max_marching_steps)
        .filter(|DepthResult { total_depth, .. }| total_depth < &find_target_settings.t_max)
        .find(|DepthResult { dist, .. }| dist.abs() < find_target_settings.epsilon)
        .map(|dr| (dr.point, dr.mat_idx))
    }
}

pub struct FindTargetSettings<F> {
    t_min: F,
    t_max: F,
    epsilon: F,
    max_marching_steps: usize,
}

impl<F> FindTargetSettings<F> {
    pub fn new(t_min: F, t_max: F, epsilon: F, max_marching_steps: usize) -> Self {
        Self {
            t_min,
            t_max,
            epsilon,
            max_marching_steps,
        }
    }
}

struct DepthIterator<'a, F> {
    sdf: &'a dyn Sdf<F>,
    r: &'a Ray<F>,
    cur_depth: F,
}

struct DepthResult<F> {
    point: Point3<F>,
    dist: F,
    mat_idx: Option<MaterialIndex>,
    total_depth: F,
}

impl<'a, F: Float> Iterator for DepthIterator<'a, F> {
    type Item = DepthResult<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let point = Point3(self.r.origin.as_ref() + &self.r.direction * self.cur_depth);
        let (dist, mat_idx) = self.sdf.value_at(&point);
        let total_depth = dist + self.cur_depth;

        self.cur_depth = total_depth;

        Some(Self::Item {
            point,
            dist,
            mat_idx,
            total_depth,
        })
    }
}
