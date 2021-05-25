use crate::scene::scenemap::material::MaterialIndex;
use crate::scene::scenemap::sdf::Sdf;
use crate::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn find_target(
        &self,
        find_target_settings: &FindTargetSettings,
        sdf: &dyn Sdf,
    ) -> Option<(Point3, Option<MaterialIndex>)> {
        DepthIterator {
            sdf,
            r: &self,
            cur_depth: find_target_settings.t_min,
        }
        .into_iter()
        .take_while(|DepthResult { total_depth, .. }| total_depth < &find_target_settings.t_max)
        .find(|DepthResult { dist, .. }| *dist < find_target_settings.epsilon)
        .map(|dr| (dr.point, dr.mat_idx))
    }

    pub fn is_shadow(&self, find_target_settings: &FindTargetSettings, sdf: &dyn Sdf) -> bool {
        DepthIterator{
            sdf,
            r: &self,
            cur_depth: find_target_settings.t_min
        }
            .into_iter()
            .take_while(|DepthResult { total_depth, .. }| total_depth < &find_target_settings.t_max)
            .find(|DepthResult{dist, ..}| *dist < find_target_settings.epsilon)
            .is_some()
    }
}

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

struct DepthIterator<'a> {
    sdf: &'a dyn Sdf,
    r: &'a Ray,
    cur_depth: f64,
}

struct DepthResult {
    point: Point3,
    dist: f64,
    mat_idx: Option<MaterialIndex>,
    total_depth: f64,
}

impl<'a> Iterator for DepthIterator<'a> {
    type Item = DepthResult;

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
