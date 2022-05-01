use crate::util::{Model, ModelComponent, Part, Primitive, Point};
use sdl2::{render::Canvas, video::Window, pixels::Color, rect::Rect, gfx::primitives::DrawRenderer};
use bevy::prelude::*;

pub fn draw_scene() {unimplemented!()}

pub fn draw_model_sys(query: Query<&ModelComponent>, mut canvas: NonSendMut<Canvas<Window>>) {
    for model in query.iter() {
        draw_model(&model.0, &mut canvas);
    }
}

pub fn draw_model(model: &Model, canvas: &mut Canvas<Window>) {
    for part in model.parts.iter() { draw_part(part, canvas); }
}

pub fn draw_part(part: &Part, canvas: &mut Canvas<Window>) {
    for pri in part.0.iter() {
        match pri {
            Primitive::Point {point, rgba} => {
                if let Some(rgba) = rgba {
                    let (r, g, b, a) = (rgba[0], rgba[1], rgba[2], rgba[3]);
                    canvas.set_draw_color(Color::RGBA(r, g, b, a));
                }

                canvas.draw_point(*point).unwrap();
            },
            Primitive::Line {points, rgba} => {
                if let Some(rgba) = rgba {
                    let (r, g, b, a) = (rgba[0], rgba[1], rgba[2], rgba[3]);
                    canvas.set_draw_color(Color::RGBA(r, g, b, a));
                }

                canvas.draw_line::<Point, Point>(points[0], points[1]).unwrap();
            },
            Primitive::Bezier {points, steps, rgba} => {
                let mut c = Color::RGBA(0, 0, 0, 0);

                if let Some(rgba) = rgba {
                    let (r, g, b, a) = (rgba[0], rgba[1], rgba[2], rgba[3]);
                    c = Color::RGBA(r, g, b, a);
                   // canvas.set_draw_color(c);
                }

                let mut vx: Vec<i16> = Vec::new();
                let mut vy: Vec<i16> = Vec::new();

                for p in points.iter() {
                    vx.push((*p).x as i16);
                    vy.push((*p).y as i16);
                }

                canvas.bezier(vx.as_slice(), vy.as_slice(), *steps, c).unwrap();
            },
            Primitive::Rect {x, y, width, height, rgba} => {
                if let Some(rgba) = rgba {
                    let (r, g, b, a) = (rgba[0], rgba[1], rgba[2], rgba[3]);
                    canvas.set_draw_color(Color::RGBA(r, g, b, a));
                }

                canvas.draw_rect(Rect::new(*x, *y, *width, *height)).unwrap();
            }
        }
    }
}
