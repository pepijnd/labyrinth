use labyrinth_assets::assets::{
    Animation, AnimationTarget, Effect, Joint, Mesh, Model, Skeleton, Vertex,
};

use labyrinth_cgmath::FloatMat4;

use itertools::izip;

pub struct ColladaAssets {
    pub models: Vec<Model>,
    pub skeletons: Vec<Skeleton>,
    pub animations: Vec<Animation>,
    pub effects: Vec<Effect>,
}

pub fn parse_collada_file(input: String) -> ColladaAssets {
    let file = labyrinth_collada::document::ColladaDocument::from_str(&input).unwrap();
    let objs = file.get_obj_set().unwrap();

    let mut models = Vec::new();
    for obj in objs.objects.iter() {
        let mut meshes = Vec::new();
        for (i, mesh) in obj.geometry.iter().enumerate() {
            let mut vertices = Vec::new();
            let mut material = String::new();
            for shapes in mesh.mesh.iter() {
                if let labyrinth_collada::PrimitiveElement::Triangles(shapes) = shapes {
                    if let Some(shape_material) = &shapes.material {
                        material = shape_material.clone();
                    }
                    for (vindex, tindex, _nindex) in izip!(
                        shapes.vertices.iter().cloned(),
                        shapes.tex_vertices.as_ref().unwrap().iter().cloned(),
                        shapes.normals.as_ref().unwrap().iter().cloned()
                    ) {
                        let mut primitive = Vertex::from_vertices(vec![
                            (
                                obj.vertices[vindex.0].into(),
                                obj.tex_vertices[tindex.0].into(),
                                obj.joint_weights[vindex.0].joints,
                                obj.joint_weights[vindex.0].weights,
                            ),
                            (
                                obj.vertices[vindex.1].into(),
                                obj.tex_vertices[tindex.1].into(),
                                obj.joint_weights[vindex.1].joints,
                                obj.joint_weights[vindex.1].weights,
                            ),
                            (
                                obj.vertices[vindex.2].into(),
                                obj.tex_vertices[tindex.2].into(),
                                obj.joint_weights[vindex.2].joints,
                                obj.joint_weights[vindex.2].weights,
                            ),
                        ]);
                        vertices.append(&mut primitive);
                    }
                }
            }
            let mname = format!("{}{}", obj.name.clone(), i);
            let mesh = Mesh {
                name: mname.clone(),
                material: material,
                vertices,
            };
            meshes.push(mesh);
        }
        models.push(Model {
            name: obj.name.clone(),
            meshes,
        });
    }

    let mut skeletons = Vec::new();
    for skeleton in file.get_skeletons().unwrap() {
        let joints = skeleton
            .joints
            .iter()
            .map(|x| Joint {
                name: x.name.clone(),
                parent_index: if x.parent_index == 255 {
                    None
                } else {
                    Some(x.parent_index)
                },
                inverse_bind_pose: FloatMat4::from(x.inverse_bind_pose),
            })
            .collect();
        skeletons.push(Skeleton {
            name: skeleton.name.clone(),
            joints: joints,
            bind_poses: skeleton
                .bind_poses
                .iter()
                .cloned()
                .map(|x| FloatMat4::from(x))
                .collect(),
        });
    }

    let animations = file
        .get_animations()
        .iter()
        .map(|animation| Animation {
            name: animation.name.clone(),
            targets: animation
                .targets
                .iter()
                .map(|target| AnimationTarget {
                    target: target.target.clone(),
                    sample_poses: target
                        .sample_poses
                        .iter()
                        .map(|pose| FloatMat4::from(*pose))
                        .collect(),
                })
                .collect(),
        })
        .collect();

    let effects = file
        .get_effect_library()
        .iter()
        .map(|(name, effect)| Effect {
            name: name.clone(),
            emission: (effect.emission[0], effect.emission[1], effect.emission[2]),
            ambient: (effect.ambient[0], effect.ambient[1], effect.ambient[2]),
            diffuse: (effect.diffuse[0], effect.diffuse[1], effect.diffuse[2]),
            specular: (effect.specular[0], effect.specular[1], effect.specular[2]),
            shininess: effect.shininess,
            alpha: 1.0,
            refraction: effect.index_of_refraction,
        })
        .collect();

    dbg!(&effects);

    ColladaAssets {
        models,
        skeletons,
        animations,
        effects,
    }
}
