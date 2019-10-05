use serde::Deserialize;
use serde::Serialize;

use labyrinth_assets::assets::{Mesh, Model, Vertices};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeshRepr {
    pub name: String,
    pub material: String,
    #[serde(skip)]
    pub vertices: Vertices,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh_source: Option<String>,
}

impl Into<Mesh> for MeshRepr {
    fn into(self) -> Mesh {
        Mesh {
            name: self.name,
            material: self.material,
            vertices: self.vertices.into(),
        }
    }
}

impl From<Mesh> for MeshRepr {
    fn from(mesh: Mesh) -> Self {
        MeshRepr {
            name: mesh.name,
            material: mesh.material,
            vertices: mesh.vertices.as_slice().into(),
            mesh_source: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelRepr {
    pub name: String,
    pub meshes: Vec<MeshRepr>,
}

impl Into<Model> for ModelRepr {
    fn into(self) -> Model {
        Model {
            name: self.name,
            meshes: self.meshes.iter().cloned().map(|x| x.into()).collect(),
        }
    }
}

impl From<Model> for ModelRepr {
    fn from(model: Model) -> Self {
        ModelRepr {
            name: model.name,
            meshes: model.meshes.iter().cloned().map(|x| x.into()).collect(),
        }
    }
}
