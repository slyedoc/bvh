use glam::Vec3;

use crate::{N, tri::Tri, ray::Ray};


#[derive(Default, Copy, Clone)]
pub struct BvhNode {
    aabb_min: Vec3, // 12
    aabb_max: Vec3, // 12
    left_child: u32,
    right_child: u32,
    //isLeaf: bool,
    first_triangle: u32,
    triangle_count: u32,
}

impl BvhNode {
    pub fn is_leaf(&self) -> bool {
        self.triangle_count > 0
    }
}


pub struct Bvh {
    pub nodesUsed: u32,
    pub nodes: [BvhNode; N * 2 - 1],
    pub triangle_count: usize,
    pub triangles: [Tri; N],
    pub triangle_indexs: [usize; N],
}

impl Bvh {
    pub fn new() -> Self {
        Bvh {
            
            triangles: [Tri::default(); N],
            triangle_indexs: [0; N],
            nodesUsed: 1,
            triangle_count: 0, // 0 will be root
            nodes: [BvhNode::default(); N * 2 - 1],
        }
    }

    /// Builds a BVH from a list of triangles added
    pub fn setup(&mut self) {
        let mut root = &mut self.nodes[0];
        root.left_child = 0;
        root.right_child = 0;
        root.first_triangle = 0;

        root.triangle_count = self.triangle_count as u32; // N

        self.update_node_bounds(0);
        // subdivide recursively
        self.subdivide_node(0);
    }

    pub fn add_triangle(&mut self, t: Tri) {
        assert!(self.triangle_count < N);

        self.triangles[self.triangle_count] = t;
        self.triangle_indexs[self.triangle_count] = self.triangle_count;
        self.triangle_count += 1;
    }

    fn update_node_bounds(&mut self, nodeIdx: usize) {
        let mut node = &mut self.nodes[nodeIdx];
        node.aabb_min = Vec3::splat(1e30f32);
        node.aabb_max = Vec3::splat(-1e30f32);
        for i in node.first_triangle..node.triangle_count {
            let leafTri = &self.triangles[(node.first_triangle + i) as usize];
            node.aabb_min = node.aabb_min.min(leafTri.vertex0);
            node.aabb_min = node.aabb_min.min(leafTri.vertex1);
            node.aabb_min = node.aabb_min.min(leafTri.vertex2);
            node.aabb_max = node.aabb_max.max(leafTri.vertex0);
            node.aabb_max = node.aabb_max.max(leafTri.vertex1);
            node.aabb_max = node.aabb_max.max(leafTri.vertex2);
        }
    }

    fn subdivide_node(&mut self, nodeIdx: usize) {
        // terminate recursion

        if self.nodes[nodeIdx].triangle_count <= 2 {
            return;
        }
        // determine split axis and position
        let extent = self.nodes[nodeIdx].aabb_max - self.nodes[nodeIdx].aabb_min;
        let mut axis = 0;
        if extent.y > extent.x {
            axis = 1;
        }
        if extent.z > extent[axis] {
            axis = 2;
        }
        let splitPos = self.nodes[nodeIdx].aabb_min[axis] + extent[axis] * 0.5f32;
        // in-place partition
        let mut i = self.nodes[nodeIdx].first_triangle;
        let mut j = i + self.nodes[nodeIdx].triangle_count - 1;
        while (i <= j) {
            if self.triangles[self.triangle_indexs[i as usize] as usize].centroid[axis] < splitPos {
                i += 1;
            } else {
                self.triangle_indexs.swap(i as usize, j as usize);
                j -= 1;
            }
        }
        // abort split if one of the sides is empty
        let leftCount = i - self.nodes[nodeIdx].first_triangle;
        if (leftCount == 0 || leftCount == self.nodes[nodeIdx].triangle_count) {
            return;
        }
        // create child nodes
        let leftChildIdx = self.nodesUsed;
        self.nodesUsed += 1;
        let rightChildIdx = self.nodesUsed;
        self.nodesUsed += 1;

        self.nodes[nodeIdx].left_child = leftChildIdx;

        self.nodes[leftChildIdx as usize].first_triangle = self.nodes[nodeIdx].first_triangle;
        self.nodes[leftChildIdx as usize].triangle_count = leftCount;
        self.nodes[rightChildIdx as usize].first_triangle = i;
        self.nodes[rightChildIdx as usize].triangle_count = self.nodes[nodeIdx].triangle_count - leftCount;

        self.nodes[nodeIdx].triangle_count = 0;

        self.update_node_bounds(leftChildIdx as usize);
        self.update_node_bounds(rightChildIdx as usize);
        // recurse
        self.subdivide_node(leftChildIdx as usize);
        self.subdivide_node(rightChildIdx as usize);
    }

    pub fn intersect(&self, ray: &mut Ray, nodeIdx: u32) {
        let node = &self.nodes[nodeIdx as usize];
        if !ray.intersect_aabb(node.aabb_min, node.aabb_max) {
            return;
        }
        if node.is_leaf() {
            for i in 0..node.triangle_count {
                ray.intersect_triangle(&self.triangles[node.first_triangle as usize + i as usize]);
            }
        } else {
            self.intersect(ray, node.left_child);
            self.intersect(ray, node.left_child + 1);
        }
    }
}