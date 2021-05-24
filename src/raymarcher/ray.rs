use crate::scene::scenemap::sdf::Sdf;
use crate::{Point3, RenderSettings, Vec3};
use num_traits::Float;

pub struct Ray<F> {
    pub origin: Point3<F>,
    pub direction: Vec3<F>,
}

impl<F: Float> Ray<F> {
    pub fn find_target(
        &self,
        render_settings: &RenderSettings<F>,
        sdf: &dyn Sdf<F>,
    ) -> Option<Point3<F>> {
        DepthIterator {
            sdf,
            r: &self,
            cur_depth: render_settings.t_min,
        }
        .into_iter()
        .take(render_settings.max_marching_steps)
        .filter(|DepthResult { total_depth, .. }| total_depth < &render_settings.t_max)
        .find(|DepthResult { dist, .. }| dist.abs() < render_settings.epsilon)
        .map(|dr| dr.point)
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
    total_depth: F,
}

impl<'a, F: Float> Iterator for DepthIterator<'a, F> {
    type Item = DepthResult<F>;

    fn next(&mut self) -> Option<Self::Item> {
        let point = Point3(self.r.origin.as_ref() + &self.r.direction * self.cur_depth);
        let dist = self.sdf.value_at(&point);
        let total_depth = dist + self.cur_depth;

        self.cur_depth = total_depth;

        Some(Self::Item {
            point,
            dist,
            total_depth,
        })
    }
}
