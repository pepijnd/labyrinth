use std::collections::HashMap;
use std::fs::read_dir;

use crate::utils::{model::ModelRepr, shader::ProgramRepr};
use labyrinth_assets::assets::{
    Animation, Assets, Effect, Material, Object, Program, Shader, Skeleton, Vertices,
};

use labyrinth_engine::labyrinth_error;

#[derive(Debug)]
pub enum PackError {
    IO(std::io::Error),
    Serde(serde_json::error::Error)
}

labyrinth_error!(PackError, |e| match e {
    PackError::IO(e) => {
        format!("[IO] {}", e)
    },
    PackError::Serde(e) => {
        format!("[Serde] {}", e)
    }
});

impl From<std::io::Error> for PackError {
    fn from(e: std::io::Error) -> Self {
        PackError::IO(e.into())
    }
} 

impl From<serde_json::error::Error> for PackError {
    fn from(e: serde_json::error::Error) -> Self {
        PackError::Serde(e.into())
    }
}

pub fn pack(in_file: &str) -> Result<Assets, PackError> {
    let base_dir = std::path::PathBuf::from(in_file);

    let mut meshes: HashMap<String, Vertices> = HashMap::new();
    let mut meshes_path = base_dir.clone();
    meshes_path.push("meshes");
    for vertice in read_dir(meshes_path)? {
        let path = vertice?.path();
        let input = std::fs::read_to_string(&path)?;
        let input: Vertices = serde_json::from_str(&input)?;
        meshes.insert(
            path.file_stem().unwrap().to_str().unwrap().to_owned(),
            input,
        );
    }

    let mut models = Vec::new();
    let mut model_path = base_dir.clone();
    model_path.push("models");
    for model in read_dir(model_path)? {
        let path = model?.path();
        let input = std::fs::read_to_string(&path)?;
        let mut input: ModelRepr = serde_json::from_str(&input)?;

        for meshrepr in input.meshes.iter_mut() {
            let mesh_name = if let Some(mesh_name) = meshrepr.mesh_source.clone() {
                mesh_name
            } else {
                format!("{}_{}", &input.name, &meshrepr.name)
            };
            let vertices = meshes.get(&mesh_name).unwrap();
            meshrepr.vertices = vertices.clone();
        }
        models.push(input.into());
    }

    let mut skeletons = Vec::new();
    let mut skeleton_path = base_dir.clone();
    skeleton_path.push("skeletons");
    for skeleton in read_dir(skeleton_path)? {
        let path = skeleton?.path();
        let input = std::fs::read_to_string(&path)?;
        let input: Skeleton = serde_json::from_str(&input)?;
        skeletons.push(input.into());
    }

    let mut animations = Vec::new();
    let mut animation_path = base_dir.clone();
    animation_path.push("animations");
    for animation in read_dir(animation_path)? {
        let path = animation?.path();
        let input = std::fs::read_to_string(&path)?;
        let input: Animation = serde_json::from_str(&input)?;
        animations.push(input.into());
    }

    let mut shaders: HashMap<String, Shader> = HashMap::new();
    let mut shaders_path = base_dir.clone();
    shaders_path.push("shaders");
    for shader in read_dir(shaders_path)? {
        let path = shader?.path();
        let input = std::fs::read_to_string(&path)?;
        shaders.insert(
            path.file_stem().unwrap().to_str().unwrap().to_owned(),
            Shader { code: input },
        );
    }

    let mut programs = Vec::new();
    let mut programs_path = base_dir.clone();
    programs_path.push("programs");
    for program in read_dir(programs_path)? {
        let path = program?.path();
        let input = std::fs::read_to_string(&path)?;
        let input: ProgramRepr = serde_json::from_str(&input)?;

        programs.push(Program {
            name: input.name.clone(),
            vertex: shaders.get(&input.vertex).unwrap().clone(),
            fragment: shaders.get(&input.fragment).unwrap().clone(),
        });
    }

    let mut effects = Vec::new();
    let mut effect_path = base_dir.clone();
    effect_path.push("effects");
    for effect in read_dir(effect_path)? {
        let path = effect?.path();
        let input = std::fs::read_to_string(&path)?;
        let input: Effect = serde_json::from_str(&input)?;
        effects.push(input.into());
    }

    let mut materials = Vec::new();
    let mut material_path = base_dir.clone();
    material_path.push("materials");
    for material in read_dir(material_path)? {
        let path = material?.path();
        let input = std::fs::read_to_string(&path)?;
        let input: Material = serde_json::from_str(&input)?;
        materials.push(input.into());
    }

    let mut objects = Vec::new();
    let mut object_path = base_dir.clone();
    object_path.push("objects");
    for object in read_dir(object_path)? {
        let path = object?.path();
        let input = std::fs::read_to_string(&path)?;
        let input: Object = serde_json::from_str(&input)?;
        objects.push(input);
    }

    Ok(Assets {
        models,
        skeletons,
        animations,
        programs,
        objects,
        effects,
        materials,
    })
}
