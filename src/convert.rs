use std::io::prelude::*;

pub use crate::utils::collada;
pub use crate::utils::material;
pub use crate::utils::model;

pub fn convert(in_file: &str, out_file: &str) -> Result<(), std::io::Error> {
    let mut output = std::path::PathBuf::new();
    output.push(std::path::Path::new(out_file));

    let filename = std::path::Path::new(in_file);

    if let Some(extension) = filename.extension() {
        match extension.to_str().unwrap() {
            "dae" => {
                let file = std::fs::File::open(filename)?;
                let mut file = std::io::BufReader::new(file);
                let mut input = String::new();
                file.read_to_string(&mut input)?;
                let assets = collada::parse_collada_file(input);

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
            }

            _ => {}
        }
    } else {
        panic!("unknown file type");
    }

    Ok(())
}
