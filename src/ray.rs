use glam::Vec3;

use crate::tri::Tri;

pub struct Ray {
    pub orgin: Vec3,
    // Should be normalized
    pub distance: Vec3,
    pub t: f32,
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            orgin: Vec3::ZERO,
            distance: Vec3::ZERO,
            t: 0.0,
        }
    }
}

impl Ray {
    // Moller Trumbore
    // https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    pub fn intersect_triangle(&mut self, tri: &Tri) {
        let edge1 = tri.vertex1 - tri.vertex0;
        let edge2 = tri.vertex2 - tri.vertex0;
        let h = self.distance.cross(edge2);
        let a = edge1.dot(h);
        if a > -0.0001 && a < 0.0001 {
            return;
        };
        // ray parallel to triangle
        let f = 1.0 / a;
        let s = self.orgin - tri.vertex0;
        let u = f * s.dot(h);
        if u < 0.0 || u > 1.0 {
            return;
        }
        let q = s.cross(edge1);
        let v = f * self.distance.dot(q);
        if v < 0.0 || u + v > 1.0 {
            return;
        }
        let t = f * edge2.dot(q);
        if t > 0.0001 {
            self.t = self.t.min(t)
        }
    }

    pub fn intersect_aabb(&self, bmin: Vec3, bmax: Vec3) -> bool {
        let tx1 = (bmin.x - self.orgin.x) / self.distance.x;
        let tx2 = (bmax.x - self.orgin.x) / self.distance.x;
        let tmin = tx1.min(tx2);
        let tmax = tx1.max(tx2);
        let ty1 = (bmin.y - self.orgin.y) / self.distance.y;
        let ty2 = (bmax.y - self.orgin.y) / self.distance.y;
        let tmin = tmin.max(ty1.min(ty2));
        let tmax = tmax.min(ty1.max(ty2));
        let tz1 = (bmin.z - self.orgin.z) / self.distance.z;
        let tz2 = (bmax.z - self.orgin.z) / self.distance.z;
        let tmin = tmin.max(tz1.min(tz2));
        let tmax = tmax.min(tz1.max(tz2));
        return tmax >= tmin && tmin < self.t && tmax > 0.0;
    }
}
