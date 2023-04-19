extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use std::ops::Add;

const MAP_WIDTH: usize = 24;
const MAP_HEIGHT: usize = 24;
const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 480;

const WORLD_MAP: [&[i32; MAP_WIDTH]; MAP_HEIGHT] = [
    &[
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    &[
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    &[
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    &[
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    &[
        1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
    ],
    &[
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    &[
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 1,
    ],
    &[
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    &[
        1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
    ],
    &[
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    &[
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    &[
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    &[
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    &[
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    &[
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    &[
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    &[
        1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    &[
        1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    &[
        1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    &[
        1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    &[
        1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    &[
        1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    &[
        1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    &[
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
];

#[derive(Clone, Copy, PartialEq)]
pub struct Vec2D {
    x: f64,
    y: f64,
}

impl Add<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Vec2D) -> Self::Output {
        Vec2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Vec2D {
    pub fn new(x: f64, y: f64) -> Vec2D {
        Vec2D { x, y }
    }
}

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    //    fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
    //        let Point(x, y) = point;
    //        self.canvas.fill_rect(Rect::new(
    //            x * DOT_SIZE_IN_PXS as i32,
    //            y * DOT_SIZE_IN_PXS as i32,
    //            DOT_SIZE_IN_PXS.try_into().unwrap(),
    //            DOT_SIZE_IN_PXS.try_into().unwrap(),
    //        ))?;
    //
    //        Ok(())
    //    }

    pub fn draw(&mut self, context: &GameContext) -> Result<(), String> {
        self.draw_background(context);
        self.canvas.present();

        Ok(())
    }

    fn draw_background(&mut self, context: &GameContext) {
        let color = Color::RED;
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }
}

pub struct GameContext {}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {}
    }
    pub fn next_tick(&mut self) {}
}

fn main() -> Result<(), String> {
    let mut player_position = Vec2D::new(22.0, 12.0);
    let mut player_direction = Vec2D::new(-1.0, 0.0);
    let mut plane_position = Vec2D::new(0.0, 0.66);

    let mut time = 0.0; //time of current frame
    let mut old_time = 0.0; //time of previous frame

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Snake Game",
            (SCREEN_WIDTH).try_into().unwrap(),
            (SCREEN_HEIGHT).try_into().unwrap(),
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut context = GameContext::new();
    let mut renderer = Renderer::new(window)?;
    renderer.draw(&context)?;
    let mut frame_counter = 0;

    let done = false;

    while !done {
        for x in 0..(SCREEN_WIDTH - 1) {
            let mut camera_x: f64 = 2.0 * (x as f64) / (SCREEN_WIDTH as f64) - 1.0;
            let mut raydir_x: f64 = player_direction.x + plane_position.x * camera_x;
            let mut raydir_y: f64 = player_direction.y + plane_position.y * camera_x;

            let mut map_x: i32 = player_position.x as i32;
            let mut map_y: i32 = player_position.y as i32;

            let mut side_dist_x: f64 = 0.0;
            let mut side_dist_y: f64 = 0.0;
            let mut perpwalldist: f64 = 0.0;

            let delta_dist_x: f64 = if raydir_x == 0.0 {
                1e30
            } else {
                (1.0 / raydir_x).abs()
            };
            let delta_dist_y: f64 = if raydir_y == 0.0 {
                1e30
            } else {
                (1.0 / raydir_y).abs()
            };

            let mut step_x: i32 = 0;
            let mut step_y: i32 = 0;
            let mut hit: i32 = 0;
            let mut side: i32 = 0;

            if (raydir_x < 0.0) {
                step_x = -1;
                side_dist_x = (player_position.x - map_x as f64) * delta_dist_x;
            } else {
                step_x = 1;
                side_dist_x = (map_x as f64 + 1.0 - player_position.x) * delta_dist_x;
            }
            if (raydir_y < 0.0) {
                step_y = -1;
                side_dist_y = (player_position.y - map_y as f64) * delta_dist_y;
            } else {
                step_y = 1;
                side_dist_y = (map_y as f64 + 1.0 - player_position.y) * delta_dist_y;
            }

            while hit == 0 {
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    side = 1;
                }
                if WORLD_MAP[map_x as usize][map_y as usize] > 0 {
                    hit = 1;
                }
            }
        }
    }

    Ok(())
}
