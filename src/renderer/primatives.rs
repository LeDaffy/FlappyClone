use nalgebra::{Point2, Point3};
use std::cmp::PartialEq;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vert {
    pub pos: Point3<f32>,
    pub color: Point3<f32>,
    pub uv: Point2<f32>,
    pub normal: Point3<f32>,
}

impl Vert {
    pub fn new(pos: Point3<f32>, color: Point3<f32>, uv: Point2<f32>, normal: Point3<f32>) -> Vert {
        Vert {
            pos,
            color,
            uv,
            normal,
        }
    }
    pub fn from_position(pos: [f32; 3]) -> Vert {
        Vert {
            pos: Point3::new(pos[0], pos[1], pos[2]),
            color: Point3::new(0.0, 0.0, 0.0),
            uv: Point2::new(0.0, 0.0),
            normal: Point3::new(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Debug)]
pub struct Tri {
    pub verts: [Vert; 3],
}

impl Tri {
    pub fn new(verts: [Vert; 3]) -> Tri {
        Tri { verts }
    }
    pub fn from_positions(verts: [Point3<f32>; 3]) -> Tri {
        Tri {
            verts: [
                Vert::from_position([verts[0].x, verts[0].y, verts[0].z]),
                Vert::from_position([verts[1].x, verts[1].y, verts[1].z]),
                Vert::from_position([verts[2].x, verts[2].y, verts[2].z]),
            ],
        }
    }
    pub fn elements(&self) -> [u32; 3] {
        [0, 1, 2]
    }
}

#[derive(Debug)]
pub struct Quad {
    pub verts: [Vert; 4],
    pub elements: [u32; 6],
}

impl Quad {
    pub fn new(verts: [Vert; 4]) -> Quad {
        Quad {
            verts: verts,
            elements: [0, 1, 2, 0, 2, 3],
        }
    }
    pub fn new_square() -> Self {
        Self {
            verts: [
                Vert::from_position([-1.0, -1.0, 0.0]),
                Vert::from_position([1.0, -1.0, 0.0]),
                Vert::from_position([1.0, 1.0, 0.0]),
                Vert::from_position([-1.0, 1.0, 0.0]),
            ],
            elements: [0, 1, 2, 0, 2, 3],
        }
    }
    pub fn elements(&self) -> [u32; 6] {
        self.elements
    }
    pub fn verts(&self) -> [Vert; 4] {
        self.verts
    }
}

#[derive(Debug)]
pub struct Cube {
    pub quads: [Quad; 6],
}

impl Cube {
    pub fn verts(&self) -> Vec<Vert> {
        self.quads.iter().map(|q| q.verts).flatten().collect()
    }
    pub fn elements(&self) -> Vec<u32> {
        let mut offset = 0;
        self.quads
            .iter()
            .map(|q| {
                let a = q
                    .elements
                    .iter()
                    .map(|e| {
                        let x = e + offset;
                        x
                    })
                    .collect::<Vec<u32>>();
                offset += 4;
                a
            })
            .flatten()
            .collect()
    }
    pub fn new() -> Cube {
        let sides = [
            // bottom
            Quad::new([
                Vert::new(
                    Point3::new(-0.5, -0.5, -0.5),
                    Point3::new(0.0, 0.0, 0.0),
                    Point2::new(0.0, 0.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
                Vert::new(
                    Point3::new(0.5, -0.5, -0.5),
                    Point3::new(1.0, 0.0, 0.0),
                    Point2::new(1.0, 0.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
                Vert::new(
                    Point3::new(0.5, 0.5, -0.5),
                    Point3::new(1.0, 1.0, 0.0),
                    Point2::new(1.0, 1.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
                Vert::new(
                    Point3::new(-0.5, 0.5, -0.5),
                    Point3::new(0.0, 1.0, 0.0),
                    Point2::new(0.0, 1.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
            ]),
            // top
            Quad::new([
                Vert::new(
                    Point3::new(-0.5, -0.5, 0.5),
                    Point3::new(0.0, 0.0, 1.0),
                    Point2::new(0.0, 0.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
                Vert::new(
                    Point3::new(0.5, -0.5, 0.5),
                    Point3::new(1.0, 0.0, 1.0),
                    Point2::new(1.0, 0.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
                Vert::new(
                    Point3::new(0.5, 0.5, 0.5),
                    Point3::new(1.0, 1.0, 1.0),
                    Point2::new(1.0, 1.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
                Vert::new(
                    Point3::new(-0.5, 0.5, 0.5),
                    Point3::new(0.0, 1.0, 1.0),
                    Point2::new(0.0, 1.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
            ]),
            // side
            Quad::new([
                Vert::new(
                    Point3::new(-0.5, -0.5, -0.5),
                    Point3::new(0.0, 0.0, 0.0),
                    Point2::new(0.0, 0.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
                Vert::new(
                    Point3::new(0.5, -0.5, -0.5),
                    Point3::new(1.0, 0.0, 0.0),
                    Point2::new(1.0, 0.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
                Vert::new(
                    Point3::new(0.5, -0.5, 0.5),
                    Point3::new(1.0, 0.0, 1.0),
                    Point2::new(1.0, 1.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
                Vert::new(
                    Point3::new(-0.5, -0.5, 0.5),
                    Point3::new(0.0, 0.0, 1.0),
                    Point2::new(0.0, 1.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
            ]),
            // y side
            Quad::new([
                Vert::new(
                    Point3::new(-0.5, 0.5, -0.5),
                    Point3::new(0.0, 1.0, 0.0),
                    Point2::new(0.0, 0.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
                Vert::new(
                    Point3::new(0.5, 0.5, -0.5),
                    Point3::new(1.0, 1.0, 0.0),
                    Point2::new(1.0, 0.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
                Vert::new(
                    Point3::new(0.5, 0.5, 0.5),
                    Point3::new(1.0, 1.0, 1.0),
                    Point2::new(1.0, 1.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
                Vert::new(
                    Point3::new(-0.5, 0.5, 0.5),
                    Point3::new(0.0, 1.0, 1.0),
                    Point2::new(0.0, 1.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
            ]),
            Quad::new([
                Vert::new(
                    Point3::new(-0.5, -0.5, -0.5),
                    Point3::new(0.0, 0.0, 0.0),
                    Point2::new(0.0, 0.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
                Vert::new(
                    Point3::new(-0.5, 0.5, -0.5),
                    Point3::new(0.0, 1.0, 0.0),
                    Point2::new(1.0, 0.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
                Vert::new(
                    Point3::new(-0.5, 0.5, 0.5),
                    Point3::new(0.0, 1.0, 1.0),
                    Point2::new(1.0, 1.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
                Vert::new(
                    Point3::new(-0.5, -0.5, 0.5),
                    Point3::new(0.0, 0.0, 1.0),
                    Point2::new(0.0, 1.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
            ]),
            Quad::new([
                Vert::new(
                    Point3::new(0.5, -0.5, -0.5),
                    Point3::new(1.0, 0.0, 0.0),
                    Point2::new(0.0, 0.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
                Vert::new(
                    Point3::new(0.5, 0.5, -0.5),
                    Point3::new(1.0, 1.0, 0.0),
                    Point2::new(1.0, 0.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
                Vert::new(
                    Point3::new(0.5, 0.5, 0.5),
                    Point3::new(1.0, 1.0, 1.0),
                    Point2::new(1.0, 1.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
                Vert::new(
                    Point3::new(0.5, -0.5, 0.5),
                    Point3::new(1.0, 0.0, 1.0),
                    Point2::new(0.0, 1.0),
                    Point3::new(0.0, 0.0, 1.0),
                ),
            ]),
        ];
        let mut offset = 0;
        let _ = sides.iter().map(|q| {
            let _ = q.elements().into_iter().map(|mut e| {
                e += offset;
                e
            });
            offset += 4;
        });
        Cube { quads: sides }
    }
}
