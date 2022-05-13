use glam::Vec3;

use crate::tri::Tri;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    // Should be normalized
    pub direction: Vec3,
    pub t: f32,
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            origin: Vec3::ZERO,
            direction: Vec3::ONE,
            t: 0.0,
        }
    }
}

impl Ray {
    // Moller Trumbore
    // https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    pub fn intersect_triangle(&mut self, tri: &Tri) -> bool {
        let edge1 = tri.vertex1 - tri.vertex0;
        let edge2 = tri.vertex2 - tri.vertex0;
        let h = self.direction.cross(edge2);
        let a = edge1.dot(h);
        if a > -0.0001 && a < 0.0001 {
            return false;
        };
        // ray parallel to triangle
        let f = 1.0 / a;
        let s = self.origin - tri.vertex0;
        let u = f * s.dot(h);
        if u < 0.0 || u > 1.0 {
            return false;
        }
        let q = s.cross(edge1);
        let v = f * self.direction.dot(q);
        if v < 0.0 || u + v > 1.0 {
            return false;
        }
        let t = f * edge2.dot(q);
        if t > 0.0001 {
            self.t = self.t.min(t);
            return true;
        }
        false
    }

    pub fn intersect_aabb(&self, bmin: Vec3, bmax: Vec3) -> bool {
        let tx1 = (bmin.x - self.origin.x) / self.direction.x;
        let tx2 = (bmax.x - self.origin.x) / self.direction.x;
        let tmin = tx1.min(tx2);
        let tmax = tx1.max(tx2);
        let ty1 = (bmin.y - self.origin.y) / self.direction.y;
        let ty2 = (bmax.y - self.origin.y) / self.direction.y;
        let tmin = tmin.max(ty1.min(ty2));
        let tmax = tmax.min(ty1.max(ty2));
        let tz1 = (bmin.z - self.origin.z) / self.direction.z;
        let tz2 = (bmax.z - self.origin.z) / self.direction.z;
        let tmin = tmin.max(tz1.min(tz2));
        let tmax = tmax.min(tz1.max(tz2));
        return tmax >= tmin && tmin < self.t && tmax > 0.0;
    }
}
