use glam::Vec3;
use image::imageops::FilterType::Triangle;

use crate::{ray::Ray, tri::Tri};

#[derive(Default, Debug, Copy, Clone)]
pub struct BvhNode {
    pub aabb_min: Vec3,
    aabb_max: Vec3,
    left_child: u32,
    first_tri_index: u32,
    tri_count: u32,
}

impl BvhNode {
    pub fn is_leaf(&self) -> bool {
        self.tri_count != 0
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
        root.left_child = 0;
        root.first_tri_index = 0;
        root.tri_count = triangles.len() as u32;

        self.update_node_bounds(0, triangles);
        self.subdivide_node(0, triangles);
    }

    fn update_node_bounds(&mut self, nodeIdx: usize, triangles: &Vec<Tri>) {
        let node = &mut self.nodes[nodeIdx];
        node.aabb_min = Vec3::splat(1e30f32);
        node.aabb_max = Vec3::splat(-1e30f32);
        for i in 0..node.tri_count {
            let leaf_tri_index = self.triangle_indexs[(node.first_tri_index + i) as usize];
            let leafTri = triangles[leaf_tri_index];
            node.aabb_min = node.aabb_min.min(leafTri.vertex0);
            node.aabb_min = node.aabb_min.min(leafTri.vertex1);
            node.aabb_min = node.aabb_min.min(leafTri.vertex2);
            node.aabb_max = node.aabb_max.max(leafTri.vertex0);
            node.aabb_max = node.aabb_max.max(leafTri.vertex1);
            node.aabb_max = node.aabb_max.max(leafTri.vertex2);
        }
    }

    fn subdivide_node(&mut self, nodeIdx: usize, triangles: &Vec<Tri>) {
        
        let node = &mut self.nodes[nodeIdx];
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
        let splitPos = node.aabb_min[axis] + extent[axis] * 0.5f32;
        // in-place partition
        let mut i = node.first_tri_index;
        let mut j = i + node.tri_count - 1;
        while (i <= j) {
            if triangles[self.triangle_indexs[i as usize]].centroid[axis] < splitPos {
                i += 1;
            } else {
                self.triangle_indexs.swap(i as usize, j as usize);
                j -= 1;
            }
        }
        // abort split if one of the sides is empty
        let leftCount = i - node.first_tri_index;
        if (leftCount == 0 || leftCount == node.tri_count) {
            return;
        }
        // create child nodes
        let leftChildIdx = self.open_node as u32;
        self.open_node += 1;
        let rightChildIdx = self.open_node as u32;
        self.open_node += 1;

        node.left_child = leftChildIdx;

        self.nodes[leftChildIdx as usize].first_tri_index = self.nodes[nodeIdx].first_tri_index;
        self.nodes[leftChildIdx as usize].tri_count = leftCount;
        self.nodes[rightChildIdx as usize].first_tri_index = i;
        self.nodes[rightChildIdx as usize].tri_count = self.nodes[nodeIdx].tri_count - leftCount;

        self.nodes[nodeIdx].tri_count = 0;

        self.update_node_bounds(leftChildIdx as usize, triangles);
        self.update_node_bounds(rightChildIdx as usize, triangles);
        // recurse
        self.subdivide_node(leftChildIdx as usize, triangles);
        self.subdivide_node(rightChildIdx as usize, triangles);
    }

    pub fn intersect(&self, ray: &mut Ray, nodeIdx: u32, triangles: &Vec<Tri>) {

        let node = &self.nodes[nodeIdx as usize];
        if !ray.intersect_aabb(node.aabb_min, node.aabb_max) {
            return;
        }

        let mut list = Vec::<u32>::new();
        if node.is_leaf() {
            for i in 0..node.tri_count {
                ray.intersect_triangle(&triangles[self.triangle_indexs[(node.first_tri_index + i) as usize]]);                    
            }
        } else {

            self.intersect(ray, node.left_child, triangles);
            self.intersect(ray, node.left_child + 1, triangles);
        }
    }
}
