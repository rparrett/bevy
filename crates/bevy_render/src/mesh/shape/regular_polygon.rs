use crate::mesh::{Indices, Mesh};
use wgpu::PrimitiveTopology;

/// A regular polygon in the xy plane
#[derive(Debug, Copy, Clone)]
pub struct RegularPolygon {
    /// Inscribed radius in the xy plane.
    pub radius: f32,
    /// Number of sides.
    pub sides: usize,
}
impl Default for RegularPolygon {
    fn default() -> Self {
        Self {
            radius: 0.5,
            sides: 6,
        }
    }
}

impl From<RegularPolygon> for Mesh {
    fn from(polygon: RegularPolygon) -> Self {
        let RegularPolygon { radius, sides } = polygon;

        let mut positions = Vec::with_capacity(sides);
        let mut normals = Vec::with_capacity(sides);
        let mut uvs = Vec::with_capacity(sides);

        for i in 0..sides {
            let a = std::f32::consts::FRAC_PI_2 - i as f32 * std::f32::consts::TAU / (sides as f32);

            positions.push([a.cos() * radius, a.sin() * radius, 0.0]);
            normals.push([0.0, 0.0, 1.0]);
            uvs.push([(a.cos() + 1.0) / 2.0, 1.0 - (a.sin() + 1.0) / 2.0]);
        }

        let mut indices = Vec::with_capacity((sides - 1) * 3);
        for i in 1..sides as u32 {
            indices.extend_from_slice(&[0, i + 1, i]);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}

/// A circle on the xy plane
pub struct Circle {
    /// Inscribed radius on the xy plane.
    pub radius: f32,
    /// The number of subdivisions applied.
    pub subdivisions: usize,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            radius: 0.5,
            subdivisions: 64,
        }
    }
}

impl From<Circle> for RegularPolygon {
    fn from(circle: Circle) -> Self {
        Self {
            radius: circle.radius,
            sides: circle.subdivisions,
        }
    }
}

impl From<Circle> for Mesh {
    fn from(circle: Circle) -> Self {
        Mesh::from(RegularPolygon::from(circle))
    }
}
