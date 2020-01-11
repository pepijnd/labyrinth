use std::io::prelude::*;

pub use crate::utils::collada;
pub use crate::utils::material;
pub use crate::utils::model;

use labyrinth_engine::labyrinth_error;

#[derive(Debug)]
pub enum ConvertError {
    Convert(Box<dyn std::error::Error>),
    Collada(&'static str),
    Parse(String),
    Type(String)
}

labyrinth_error!(ConvertError, |e| match e {
    ConvertError::Convert(e) => {
        format!("[Convert] {}", e)
    },
    ConvertError::Collada(e) => {
        format!("[Collada] {}", e)
    },
    ConvertError::Parse(e) => {
        format!("[Parse] {}", e)
    },
    ConvertError::Type(ext) => {
        format!("[Type] Unknown file type \"{}\"", ext)
    }
});

impl From<std::io::Error> for ConvertError {
    fn from(e: std::io::Error) -> Self { 
        ConvertError::Convert(e.into())
     }
}

impl From<&'static str> for ConvertError {
    fn from(e: &'static str) -> Self {
        ConvertError::Collada(e)
    }
}

pub fn convert(in_file: &str, out_file: &str) -> Result<(), ConvertError> {
    let mut output = std::path::PathBuf::new();
    output.push(std::path::Path::new(out_file));

    let filename = std::path::Path::new(in_file);

    match filename.extension().ok_or_else(|| ConvertError::Type("None".to_string()))?.to_str().unwrap() {
        "dae" => {
            let file = std::fs::File::open(filename)?;
            let mut file = std::io::BufReader::new(file);
            let mut input = String::new();
            file.read_to_string(&mut input)?;
            let assets = collada::parse_collada_file(input)?;

            let path = std::path::PathBuf::from(output.as_path());
            std::fs::create_dir_all(path.as_path())?;

            for model in assets.models.iter() {
                for mesh in model.meshes.iter() {
                    let out_filename = format!("{}_{}", model.name, mesh.name);
                    let mut path = std::path::PathBuf::from(path.as_path());
                    path.push(std::path::Path::new("meshes"));
                    std::fs::create_dir_all(path.as_path())?;
                    path.push(std::path::Path::new(&(out_filename + ".json")));
                    let mut file = std::fs::File::create(path)?;
                    file.write_all(
                        &serde_json::to_vec(
                            &labyrinth_assets::assets::Vertices::from_vertices(&mesh.vertices),
                        )
                        .unwrap(),
                    )?;
                }
                let mut path = std::path::PathBuf::from(path.as_path());
                path.push(std::path::Path::new("models"));
                std::fs::create_dir_all(path.as_path())?;
                path.push(std::path::Path::new(&(model.name.clone() + ".json")));
                let mut file = std::fs::File::create(path)?;
                file.write_all(
                    &serde_json::to_vec(&model::ModelRepr::from(model.clone())).unwrap(),
                )?;
            }

            for skeleton in assets.skeletons.iter() {
                let mut path = std::path::PathBuf::from(path.as_path());
                path.push(std::path::Path::new("skeletons"));
                std::fs::create_dir_all(path.as_path())?;
                let name = skeleton.name.clone() + ".json";
                path.push(std::path::Path::new(&name));
                let mut file = std::fs::File::create(path)?;
                file.write_all(&serde_json::to_vec(skeleton).unwrap())?;
            }

            for animation in assets.animations.iter() {
                let mut path = std::path::PathBuf::from(path.as_path());
                path.push(std::path::Path::new("animations"));
                std::fs::create_dir_all(path.as_path())?;
                let name = animation.name.clone() + ".json";
                path.push(std::path::Path::new(&name));
                let mut file = std::fs::File::create(path)?;
                file.write_all(&serde_json::to_vec(animation).unwrap())?;
            }

            for effect in assets.effects.iter() {
                let mut path = std::path::PathBuf::from(path.as_path());
                path.push(std::path::Path::new("effects"));
                std::fs::create_dir_all(path.as_path())?;
                let name = effect.name.clone() + ".json";
                path.push(std::path::Path::new(&name));
                let mut file = std::fs::File::create(path)?;
                file.write_all(&serde_json::to_vec(effect).unwrap())?;
            }

            Ok(())
        }

        ext => {Err(ConvertError::Type(ext.to_string()))}
    }
}
