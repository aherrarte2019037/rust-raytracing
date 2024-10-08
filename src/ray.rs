use crate::materials::Material;
use crate::point3d::Point3D;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3D,
    pub direction: Point3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Point3D) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3D {
        self.origin + self.direction * t
    }
}

pub struct HitRecord<'material> {
    pub t: f64,
    pub point: Point3D,
    pub normal: Point3D,
    pub front_face: bool,
    pub material: &'material Material,
    pub u: f64,
    pub v: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
