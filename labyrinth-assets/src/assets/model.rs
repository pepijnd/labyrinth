use serde::Deserialize;
use serde::Serialize;

use serde::de::{Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};

use itertools::izip;
use itertools::Itertools;

use glium::implement_vertex;

use labyrinth_cgmath::{prelude::*, FloatVec2, FloatVec3};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    pub name: String,
    pub meshes: Vec<Mesh>,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub name: String,
    pub material: String,
    pub vertices: Vec<Vertex>,
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: (f32, f32, f32),
    pub tex_coords: (f32, f32),
    pub normal: (f32, f32, f32),
    pub tangent: (f32, f32, f32),
    pub bitangent: (f32, f32, f32),
    pub b_index: [i32; 4],
    pub b_weight: [f32; 4],
}

implement_vertex!(
    Vertex,
    position,
    tex_coords,
    normal,
    tangent,
    bitangent,
    b_index,
    b_weight
);

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Vertices {
    position: Vec<(f32, f32, f32)>,
    tex_coords: Vec<(f32, f32)>,
    joints: Vec<[i32; 4]>,
    joint_weights: Vec<[f32; 4]>,
}

impl Vertex {
    pub fn from_vertices(
        input: Vec<((f32, f32, f32), (f32, f32), [i32; 4], [f32; 4])>,
    ) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        let v0: FloatVec3 = input[0].0.into();
        let v1: FloatVec3 = input[1].0.into();
        let v2: FloatVec3 = input[2].0.into();

        let uv0: FloatVec2 = input[0].1.into();
        let uv1: FloatVec2 = input[1].1.into();
        let uv2: FloatVec2 = input[2].1.into();

        let dp1: FloatVec3 = v1 - v0;
        let dp2: FloatVec3 = v2 - v0;

        let duv1: FloatVec2 = uv1 - uv0;
        let duv2: FloatVec2 = uv2 - uv0;

        let r = 1f32 / (duv1.x * duv2.y - duv1.y * duv2.x);
        let tangent = ((dp1 * duv2.y - dp2 * duv1.y) * r).normalize();
        let bitangent = ((dp2 * duv1.x - dp1 * duv2.x) * r).normalize();
        let normal = dp1.cross(dp2).normalize();
        for v in input.iter() {
            vertices.push(Vertex {
                position: v.0,
                tex_coords: v.1,
                normal: normal.into(),
                tangent: tangent.into(),
                bitangent: bitangent.into(),
                b_index: v.2,
                b_weight: v.3,
            })
        }
        vertices
    }
}

impl Vertices {
    pub fn to_vertices(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        let verts: Vec<((f32, f32, f32), (f32, f32), [i32; 4], [f32; 4])> = izip!(
            self.position.iter().cloned(),
            self.tex_coords.iter().cloned(),
            self.joints.iter().cloned(),
            self.joint_weights.iter().cloned()
        )
        .collect();
        for n in &verts.iter().chunks(3) {
            vertices.append(&mut Vertex::from_vertices(n.cloned().collect()));
        }
        vertices
    }

    pub fn from_vertices(vertices: &[Vertex]) -> Self {
        Vertices {
            position: vertices
                .iter()
                .map(|x| x.position)
                .collect::<Vec<(f32, f32, f32)>>(),
            tex_coords: vertices
                .iter()
                .map(|x| x.tex_coords)
                .collect::<Vec<(f32, f32)>>(),
            joints: vertices.iter().map(|x| x.b_index).collect::<Vec<[i32; 4]>>(),
            joint_weights: vertices
                .iter()
                .map(|x| x.b_weight)
                .collect::<Vec<[f32; 4]>>(),
        }
    }
}

impl Into<Vec<Vertex>> for Vertices {
    fn into(self) -> Vec<Vertex> {
        self.to_vertices()
    }
}

impl From<&[Vertex]> for Vertices {
    fn from(input: &[Vertex]) -> Self {
        Self::from_vertices(input)
    }
}

impl Serialize for Mesh {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Mesh", 3)?;
        let vertices: Vertices = self.vertices.as_slice().into();

        s.serialize_field("name", &self.name)?;
        s.serialize_field("material", &self.material)?;
        s.serialize_field("vertices", &vertices)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for Mesh {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Name,
            Material,
            Vertices,
        };

        struct MeshVisitor;

        impl<'de> Visitor<'de> for MeshVisitor {
            type Value = Mesh;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Mesh")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Mesh, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let name = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let material = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let vertices: Vertices = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;

                Ok(Mesh {
                    name,
                    material,
                    vertices: vertices.into(),
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Mesh, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name = None;
                let mut material = None;
                let mut vertices: Option<Vertices> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Material => {
                            if material.is_some() {
                                return Err(serde::de::Error::duplicate_field("material"));
                            }
                            material = Some(map.next_value()?);
                        }
                        Field::Vertices => {
                            if vertices.is_some() {
                                return Err(serde::de::Error::duplicate_field("vertices"));
                            }
                            vertices = Some(map.next_value()?);
                        }
                    }
                }
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let material =
                    material.ok_or_else(|| serde::de::Error::missing_field("material"))?;
                let vertices =
                    vertices.ok_or_else(|| serde::de::Error::missing_field("vertices"))?;
                Ok(Mesh {
                    name,
                    material,
                    vertices: vertices.to_vertices(),
                })
            }
        }
        deserializer.deserialize_struct("Mesh", &["name", "material", "vertices"], MeshVisitor)
    }
}
