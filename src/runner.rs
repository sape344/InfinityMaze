use crate::draw::draw_circle;
use piston_window::types::Color;
use piston_window::*;

pub struct Runner{
    x:i32,
    y:i32,
    path_width:i32,
}

impl Runner{
    pub fn new(x:i32, y:i32,path_width:i32) -> Runner{

        Runner
        {
            x:x,
            y:y,
            path_width:path_width,
        }

    }
    fn update_location(&mut self, a:i32, b:i32)
    {
        self.x += a;
        self.y += b;
    }
    pub fn get_x(&self) -> i32{
        return self.x ;
    }

    pub fn get_y(&self) -> i32{
        return self.y;
    }

    pub fn key_pressed(&mut self, key:Key)
    {
        let _dir = match key {
            Key::Up => self.update_location(0,-1),
            Key::Down => self.update_location(0,1),
            Key::Left => self.update_location(-1,0),
            Key::Right => self.update_location(1,0),
            _ => return,
        };
    }


    pub fn draw(&self,color:Color ,con: &Context, g: &mut G2d){
        draw_circle(color,(self.x * (self.path_width + 1) + self.path_width /2) as f64,(self.y * (self.path_width + 1) + self.path_width /2) as f64,(self.path_width/4) as f64,con,g);
    }


}