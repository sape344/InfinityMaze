use rand::Rng;

use crate::draw::draw_rectangle;
use piston_window::color::RED;
use piston_window::color::WHITE;
use piston_window::color::GREEN;

use crate::Context;
use piston_window::G2d;

#[derive(Debug, Clone, Copy)]
pub enum CellFlags
{
    NonePath = 0x0,
    PathN = 0x01,
    PathE = 0x02,
    PathS = 0x04,
    PathW = 0x08,
    Visited = 0x10,
}

pub struct Maze{
    maze: Vec<i32>,
    maze_width:i32,
    maze_height:i32,
    stack: Vec<(i32,i32)>,
    path_width:i32,
    start_x:i32,
    start_y:i32,
}

impl Maze{
    pub fn offset_according_to_stack_top(&self, x:i32, y:i32) ->usize{
        ((self.stack.last().unwrap().1 + y)*self.maze_width + self.stack.last().unwrap().0 + x) as usize
    }


    pub fn new(width: i32, height:i32, path_width: i32) -> Maze{
        let maze_width = width;
        let maze_height = height;
        let path_width = path_width;

        let maze = vec![0; (maze_width * maze_height) as usize];

        let stack = Vec::new();

        Maze{
            maze,
            maze_width,
            maze_height,
            stack,
            path_width,
            start_x : -1,
            start_y: -1,
        }
    }

    pub fn create_new_maze(&mut self)
    {
        let mut rng = rand::thread_rng();

        let mut visited_cells = 1;
        self.stack.clear();
        self.start_x = rng.gen_range(1..self.maze_width-1);
        self.start_y = rng.gen_range(1..self.maze_height-1);
        self.stack.push((self.start_x, self.start_y));
        for i in 0..self.maze_height * self.maze_width
        {
            self.maze[i as usize] = 0;
        }

        self.maze[(self.start_y * self.maze_width + self.start_x) as usize] |= CellFlags::Visited as i32;



        while visited_cells < self.maze_width * self.maze_height
        {
            let mut neighbours: Vec<i32> =Vec::new();

            // North neighbour
            if self.stack.last().unwrap().1 > 0 && self.maze[self.offset_according_to_stack_top(0,-1)] & CellFlags::Visited as i32 == 0
            {
                neighbours.push(0);
            }
            // East neighbour
            if self.stack.last().unwrap().0 < self.maze_width - 1 && self.maze[self.offset_according_to_stack_top(1, 0)] & CellFlags::Visited as i32 == 0
            {
                neighbours.push(1);
            }
            // East neighbour
            if self.stack.last().unwrap().1 < self.maze_height -1 && self.maze[self.offset_according_to_stack_top(0, 1)] & CellFlags::Visited as i32 == 0
            {
                neighbours.push(2);
            }
            // West neighbour
            if self.stack.last().unwrap().0 > 0 && self.maze[self.offset_according_to_stack_top(-1 ,0)] & CellFlags::Visited as i32 == 0
            {
                neighbours.push(3);
            } 

            if !neighbours.is_empty()
            {
                let next_cell = rng.gen_range(0..neighbours.len());


                match neighbours[next_cell] {
                    0 => {
                        let offset_top = self.offset_according_to_stack_top(0, -1);
                        self.maze[offset_top] |= CellFlags::Visited as i32 | CellFlags::PathS as i32;
                        let offset_current = self.offset_according_to_stack_top(0, 0);
                        self.maze[offset_current] |= CellFlags::PathN as i32;
                        let new_x = self.stack.last().unwrap().0;
                        let new_y = self.stack.last().unwrap().1 - 1;
                        self.stack.push((new_x, new_y));
                    }
                    1 => {
                        let offset_right = self.offset_according_to_stack_top(1, 0);
                        self.maze[offset_right] |= CellFlags::Visited as i32 | CellFlags::PathW as i32;
                        let offset_current = self.offset_according_to_stack_top(0, 0);
                        self.maze[offset_current] |= CellFlags::PathE as i32;
                        let new_x = self.stack.last().unwrap().0 + 1;
                        let new_y = self.stack.last().unwrap().1;
                        self.stack.push((new_x, new_y));
                    }
                    2 => {
                        let offset_bottom = self.offset_according_to_stack_top(0, 1);
                        self.maze[offset_bottom] |= CellFlags::Visited as i32 | CellFlags::PathN as i32;
                        let offset_current = self.offset_according_to_stack_top(0, 0);
                        self.maze[offset_current] |= CellFlags::PathS as i32;
                        let new_x = self.stack.last().unwrap().0;
                        let new_y = self.stack.last().unwrap().1 + 1;
                        self.stack.push((new_x, new_y));
                    }
                    3 => {
                        let offset_left = self.offset_according_to_stack_top(-1, 0);
                        self.maze[offset_left] |= CellFlags::Visited as i32 | CellFlags::PathE as i32;
                        let offset_current = self.offset_according_to_stack_top(0, 0);
                        self.maze[offset_current] |= CellFlags::PathW as i32;
                        let new_x = self.stack.last().unwrap().0 - 1;
                        let new_y = self.stack.last().unwrap().1;
                        self.stack.push((new_x, new_y));
                    }

                    _ =>
                    {

                    }
                }
                visited_cells += 1;
            }
            else
            {
                self.stack.pop();
            }                             
        }
    }

    pub fn can_go(&self, x:i32, y:i32, way: CellFlags) -> bool
    {
        if  self.maze[(y* self.maze_width + x) as usize] & way as i32 != 0 {
            return true;
        }
        return  false;
    }
    pub fn get_start_x(&self) ->i32
    {
        return self.start_x;
    }
    pub fn get_start_y(&self) ->i32
    {
        return self.start_y;
    }

    pub fn get_stop_x(&self) ->i32
    {
        return self.stack.last().unwrap().0;
    }
    pub fn get_stop_y(&self) ->i32
    {
        return self.stack.last().unwrap().1;
    }

    
    pub fn paint_maze(&mut self,con: &Context, g: &mut G2d){

        for x in 0..self.maze_width
        {
            for y in 0..self.maze_height
            {
                if self.maze[(y * self.maze_width + x) as usize] & CellFlags::Visited as i32 != 0
                {
                    draw_rectangle(WHITE, (x * (self.path_width + 1)) as f64, (y * (self.path_width + 1)) as f64, self.path_width as f64, self.path_width as f64, con, g);
                }

                for p in 0..self.path_width{

                    if self.maze[(y * self.maze_width + x) as usize] & CellFlags::PathS as i32 != 0
                    {
                        draw_rectangle(WHITE, (x * (self.path_width + 1) + p) as f64, (y * (self.path_width + 1) + self.path_width) as f64, 1.0, 1.0,con, g);
                    }

                    if self.maze[(y * self.maze_width + x) as usize] & CellFlags::PathE as i32 != 0 
                    {
                        draw_rectangle(WHITE, (x * (self.path_width + 1) + self.path_width) as f64, (y * (self.path_width + 1) + p) as f64, 1.0, 1.0,con, g);
                    }                
                }

            }
        }

        draw_rectangle(GREEN, (self.stack.last().unwrap().0 * (self.path_width + 1)) as f64, (self.stack.last().unwrap().1 * (self.path_width + 1)) as f64, self.path_width as f64, self.path_width as f64,con, g);
        draw_rectangle(RED, (self.start_x * (self.path_width + 1)) as f64, (self.start_y * (self.path_width + 1)) as f64, self.path_width as f64, self.path_width as f64,con, g);


    }

}