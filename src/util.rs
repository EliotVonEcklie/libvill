use serde::{Deserialize};
use bevy::prelude::*;

#[inline]
#[allow(dead_code)]
fn load_model_def(path: &str) -> Result<ModelDef, Box<dyn std::error::Error>> {
    use std::fs::read_to_string;
    use toml::from_str;
    Ok(
        from_str(&read_to_string(path)?)?
    )
}
#[inline]
#[allow(dead_code)]
fn load_part_def(path: &str) -> Result<Part, Box<dyn std::error::Error>> {
    use std::fs::read_to_string;
    use deser_hjson::from_str;
    Ok(
        from_str(&read_to_string(path)?)?
    )
}

/// Model Definitions deserialize from TOML definition files.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct ModelDef {
    parts: Vec<String>
}

pub struct Model {
    pub parts: Vec<Part>
}

impl Model {
    pub fn new(path: &str) -> Self {
        let model_def = load_model_def(&format!("assets/{0}/{0}.model.toml", path)).unwrap();

        let mut parts = Vec::new();
        for part_path in model_def.parts.iter() {
            parts.push(load_part_def(&format!("assets/{}/{}", path, part_path)).unwrap());
        }

        Self { parts }
    }
}

#[derive(Component)]
pub struct ModelComponent(pub Model);

impl ModelComponent {
    pub fn new(path: &str) -> Self {
        Self { 0: Model::new(path) }
    }
}

/// Parts deserialize from HJSON definition files.
#[derive(Deserialize, Clone, Debug)]
pub struct Part(pub Vec<Primitive>);

#[derive(Deserialize, Clone, Debug)]
pub enum Primitive {
    Point {point: Point, rgba: Option<[u8; 4]>},
    Line {points: [Point; 2], rgba: Option<[u8; 4]>},
    Bezier {points: Vec<Point>, steps: i32, rgba: Option<[u8; 4]>},
    Rect {x: i32, y: i32, width: u32, height: u32, rgba: Option<[u8; 4]>}
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Point { pub x: i32, pub y: i32 }

impl Into<sdl2::rect::Point> for Point {
    fn into(self) -> sdl2::rect::Point {
        sdl2::rect::Point::new(self.x, self.y)
    }
}
