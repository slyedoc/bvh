use glam::Vec3;
use crate::{ray::Ray, tri::Tri};

#[derive(Default, Debug, Copy, Clone)]
pub struct BvhNode {
    pub aabb_min: Vec3,
    aabb_max: Vec3,
    left_first: u32,
    tri_count: u32,
}

impl BvhNode {
    pub fn is_leaf(&self) -> bool {
        self.tri_count > 0
    }
}

pub struct Bvh {
    pub nodes: Vec<BvhNode>,
    pub open_node: usize,
    pub triangle_indexs: Vec<usize>,
}

impl Bvh {
    pub fn new(size: usize) -> Self {
        Self {
            nodes: vec![BvhNode::default(); size * 2],
            open_node: 2,
            triangle_indexs: (0..size).collect::<Vec<_>>(),
        }
    }

    /// Builds a BVH from a list of triangles added
    pub fn setup(&mut self, triangles: &Vec<Tri>) {
        let root = &mut self.nodes[0];
        root.left_first = 0;
        root.tri_count = triangles.len() as u32;

        self.update_node_bounds(0, triangles);
        self.subdivide_node(0, triangles);
    }

    fn update_node_bounds(&mut self, node_idx: usize, triangles: &Vec<Tri>) {
        let node = &mut self.nodes[node_idx];
        node.aabb_min = Vec3::splat(1e30f32);
        node.aabb_max = Vec3::splat(-1e30f32);
        for i in 0..node.tri_count {
            let leaf_tri_index = self.triangle_indexs[(node.left_first + i) as usize];
            let leaf_tri = triangles[leaf_tri_index];
            node.aabb_min = node.aabb_min.min(leaf_tri.vertex0);
            node.aabb_min = node.aabb_min.min(leaf_tri.vertex1);
            node.aabb_min = node.aabb_min.min(leaf_tri.vertex2);
            node.aabb_max = node.aabb_max.max(leaf_tri.vertex0);
            node.aabb_max = node.aabb_max.max(leaf_tri.vertex1);
            node.aabb_max = node.aabb_max.max(leaf_tri.vertex2);
        }
    }

    fn subdivide_node(&mut self, node_idx: usize, triangles: &Vec<Tri>) {
        
        let node = &self.nodes[node_idx];
        // terminate recursion
        if node.tri_count <= 2 {
            return;
        }

        // determine split axis and position
        let extent = node.aabb_max - node.aabb_min;
        let mut axis = 0;
        if extent.y > extent.x {
            axis = 1;
        }
        if extent.z > extent[axis] {
            axis = 2;
        }
        let split_pos = node.aabb_min[axis] + extent[axis] * 0.5f32;
        // in-place partition
        let mut i = node.left_first;
        let mut j = i + node.tri_count - 1;
        while i <= j {
            if triangles[self.triangle_indexs[i as usize]].centroid[axis] < split_pos {
                i += 1;
            } else {
                self.triangle_indexs.swap(i as usize, j as usize);
                j -= 1;
            }
        }
        // abort split if one of the sides is empty
        let left_count = i - node.left_first;
        if left_count == 0 || left_count == node.tri_count {
            return;
        }

        // create child nodes
        let left_child_idx = self.open_node as u32;
        self.open_node += 1;
        let right_child_idx = self.open_node as u32;
        self.open_node += 1;

        self.nodes[left_child_idx as usize].left_first = self.nodes[node_idx].left_first;
        self.nodes[left_child_idx as usize].tri_count = left_count;
        self.nodes[right_child_idx as usize].left_first = i;
        self.nodes[right_child_idx as usize].tri_count = self.nodes[node_idx].tri_count - left_count;

        self.nodes[node_idx].left_first = left_child_idx;
        self.nodes[node_idx].tri_count = 0;

        self.update_node_bounds(left_child_idx as usize, triangles);
        self.update_node_bounds(right_child_idx as usize, triangles);
        // recurse
        self.subdivide_node(left_child_idx as usize, triangles);
        self.subdivide_node(right_child_idx as usize, triangles);
    }

    pub fn intersect(&self, ray: &mut Ray, node_idx: u32, triangles: &Vec<Tri>) {

        let node = &self.nodes[node_idx as usize];
        if !ray.intersect_aabb(node.aabb_min, node.aabb_max) {
            return;
        }
        if node.is_leaf() {
            for i in 0..node.tri_count {
                ray.intersect_triangle(&triangles[self.triangle_indexs[(node.left_first + i) as usize]]);                    
            }
        } else {
            self.intersect(ray, node.left_first, triangles);
            self.intersect(ray, node.left_first + 1, triangles);
        }
    }
}
