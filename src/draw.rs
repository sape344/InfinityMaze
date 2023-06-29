use piston_window::{Context, G2d, rectangle, ellipse};
use piston_window::types::Color;

pub fn draw_rectangle(color: Color, x: f64, y: f64, width: f64, height: f64, con: &Context, g: &mut G2d) {
    rectangle(color, [x, y, width, height], con.transform, g);
}

pub fn draw_circle(color: Color, x: f64, y: f64, radius: f64, con: &Context, g: &mut G2d) {
    ellipse(color, [x - radius, y - radius, radius * 2.0, radius * 2.0], con.transform, g);
}