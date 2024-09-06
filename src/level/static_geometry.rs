use bevy::{prelude::{Bundle, Transform}, scene::SceneBundle};

#[derive(Bundle, Default)]
pub struct StaticGeometryBundle {
    model: SceneBundle,
    //todo! add collider
}

impl StaticGeometryBundle {
    pub fn new(model: SceneBundle, transform: Transform) -> Self {
        let mut model = model;
        model.transform = transform;
        
        Self {
            model,
        }
    }
}