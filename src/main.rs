use piston_window::*;
use piston_window::types::Color;

mod draw;
mod maze;
mod runner;

use runner::Runner;
use crate::maze::Maze;
use crate::maze::CellFlags;

const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const RUNNER_COLOR: Color = [0.00, 0.20, 1.00, 1.0];

const WIDTH: i32 = 900;
const HEIGHT: i32 = 500;
const PATH_WIDTH: i32 = 40;




fn main() {

    let mut maze = Maze::new(WIDTH / PATH_WIDTH, HEIGHT / PATH_WIDTH, PATH_WIDTH);
    maze.create_new_maze();
    
    let mut runner: Runner = Runner::new(maze.get_start_x(), maze.get_start_y(), PATH_WIDTH);

    let mut window: PistonWindow = WindowSettings::new("Infinity Maze", [WIDTH as u32, HEIGHT as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut key: Key = Key::Unknown;

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            maze.paint_maze(&c, g);
            runner.draw(RUNNER_COLOR, &c, g);
        });

        if let Some(Button::Keyboard(key_)) = event.press_args() {
            if key_ == Key::Escape {
                window.set_should_close(true);
            }
            key = key_;
        }

        if key == Key::Unknown {
            continue;
        }

        let way: CellFlags;
        match key {
            Key::Up => way = CellFlags::PathN,
            Key::Down => way = CellFlags::PathS,
            Key::Left => way = CellFlags::PathW,
            Key::Right => way = CellFlags::PathE,
            _ => way = CellFlags::NonePath,
        }

        if maze.can_go(runner.get_x(), runner.get_y(), way) {
            runner.key_pressed(key);
            if maze.get_stop_x() == runner.get_x() && maze.get_stop_y() == runner.get_y() {
                maze.create_new_maze();
                runner = Runner::new(maze.get_start_x(), maze.get_start_y(), PATH_WIDTH);
            }
        }

        key = Key::Unknown;

        event.update(|_| {});
    }
}