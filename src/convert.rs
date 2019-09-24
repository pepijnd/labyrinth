use labyrinth_engine::game;
use labyrinth_engine::window;
use labyrinth_engine::runner;
use labyrinth_engine::resources::loader::ResourceLoader;

use serde::Serialize;
use serde::Deserialize;

use serde::ser::{Serializer, SerializeStruct, SerializeSeq};
use serde::de::{Deserializer, Visitor, SeqAccess, MapAccess};

use std::io::prelude::*;

use labyrinth_cgmath::prelude::*;
use labyrinth_cgmath::FloatVec3;
use labyrinth_cgmath::FloatVec2;

use itertools::izip;

#[derive(Debug)]
struct Vertex {
    pub position: (f32, f32, f32),
    pub tex_coords: (f32, f32),
    pub normal: (f32, f32, f32),
    pub tangent: (f32, f32, f32),
    pub bitangent: (f32, f32, f32)
}

#[derive(Debug)]
struct Mesh {
    name: String,
    material: String,
    vertices: Vec<Vertex>
}

#[derive(Serialize, Deserialize)]
struct Vertices {
    position: Vec<(f32, f32, f32)>,
    tex_coords: Vec<(f32, f32)>,
    normal: Vec<(f32, f32, f32)>,
    tangent: Vec<(f32, f32, f32)>,
    bitangent: Vec<(f32, f32, f32)>
}

impl Vertices {
    fn to_vertices(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        for (position, tex_coords, normal, tangent, bitangent) in 
            izip!(
                self.position.iter(), 
                self.tex_coords.iter(),
                self.normal.iter(), 
                self.tangent.iter(), 
                self.bitangent.iter()) {
            vertices.push(
                Vertex {
                    position: *position, 
                    tex_coords: *tex_coords, 
                    normal: *normal, 
                    tangent: *tangent, 
                    bitangent: *bitangent
                }
            );
        }
        vertices
    }
}

impl Serialize for Mesh {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {

        let mut s = serializer.serialize_struct("Mesh", 3)?;
        let vertices = Vertices {
            position: self.vertices.iter().map(|x| x.position).collect::<Vec<(f32, f32, f32)>>(),
            tex_coords: self.vertices.iter().map(|x| x.tex_coords).collect::<Vec<(f32, f32)>>(),
            normal: self.vertices.iter().map(|x| x.normal).collect::<Vec<(f32, f32, f32)>>(),
            tangent: self.vertices.iter().map(|x| x.tangent).collect::<Vec<(f32, f32, f32)>>(),
            bitangent: self.vertices.iter().map(|x| x.bitangent).collect::<Vec<(f32, f32, f32)>>()
        };

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
        enum Field { Name, Material, Vertices };

        struct MeshVisitor;

        impl<'de> Visitor<'de> for MeshVisitor {
            type Value = Mesh;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Mesh")
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
                let material = material.ok_or_else(|| serde::de::Error::missing_field("material"))?;
                let vertices = vertices.ok_or_else(|| serde::de::Error::missing_field("vertices"))?;
                Ok(Mesh {
                    name,
                    material,
                    vertices: vertices.to_vertices()
                })
            }
        }
        deserializer.deserialize_struct("Mesh", &["name", "material", "vertices"], MeshVisitor)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Model {
    name: String,
    meshes: Vec<Mesh>
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args.get(1).unwrap();


    let file = std::fs::File::open(std::path::Path::new(filename)).unwrap();
    let mut file = std::io::BufReader::new(file);
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let objs = labyrinth_obj::obj::parse(input).unwrap();
    let mut models = Vec::new();
    for obj in objs.objects.iter() {
        let mut meshes = Vec::new();
        for (i, mesh) in obj.geometry.iter().enumerate() {
            let mut vertices = Vec::new();
            for shape in mesh.shapes.iter() {
                if let labyrinth_obj::obj::Primitive::Triangle(a, b, c) = shape.primitive {
                    let v0: FloatVec3 = obj.vertices[a.0].into();
                    let v1: FloatVec3 = obj.vertices[b.0].into();
                    let v2: FloatVec3 = obj.vertices[c.0].into();

                    let uv0: FloatVec2 = obj.tex_vertices[a.1.unwrap()].into();
                    let uv1: FloatVec2 = obj.tex_vertices[b.1.unwrap()].into();
                    let uv2: FloatVec2 = obj.tex_vertices[c.1.unwrap()].into();

                    let dp1 = v1 - v0;
                    let dp2 = v2 - v0;

                    let duv1 = uv1 - uv0;
                    let duv2 = uv2 - uv0;

                    let r = 1f32 / (duv1.x * duv2.y - duv1.y * duv2.x);
                    let tangent = ((dp1 * duv2.y - dp2 * duv1.y)*r).normalize();
                    let bitangent = ((dp2 * duv1.x - dp1 * duv2.x)*r).normalize();
                    let normal = (v1-v0).cross(v2-v0).normalize();

                    for (vindex, tindex, _nindex) in [a, b, c].iter() {
                        vertices.push(Vertex {
                            position: obj.vertices[*vindex].into(),
                            tex_coords: if let Some(index) = tindex {
                                obj.tex_vertices[*index].into()
                            } else {
                                (0.0, 0.0)
                            },
                            normal: normal.into(),
                            tangent: tangent.into(),
                            bitangent: bitangent.into()
                        })
                    }
                }
            }
            let mname = format!("{}{}", obj.name.clone(), i);
            let mesh = Mesh {
                name: mname.clone(),
                material: mesh.material_name.clone().unwrap_or_default(),
                vertices
            };
            meshes.push(mesh);
        }
        models.push(Model {
            name: obj.name.clone(),
            meshes,
        });
    }
    let serialized = serde_json::to_string(&models).unwrap();
    //println!("{}", serialized);
    // let enc: Vec<u8> = bincode::serialize(&models).unwrap();
    // let mut file = std::fs::File::create("output.bin").unwrap();
    // file.write_all(&enc).unwrap();
    let deserialized: Vec<Model> = serde_json::from_str(&serialized).unwrap();
    println!("{:?}", deserialized);

}
