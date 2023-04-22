extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use std::ops::Add;
use std::time::{Duration, SystemTime};

const MAP_WIDTH: usize = 24;
const MAP_HEIGHT: usize = 24;
const SCREEN_WIDTH: usize = 1280;
const SCREEN_HEIGHT: usize = 720;

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

    pub fn draw_box(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: Color,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        self.canvas
            .fill_rect(Rect::new(x, y, width as u32, height as u32))?;

        Ok(())
    }

    pub fn draw_vertical_line(
        &mut self,
        x: i32,
        y_top: i32,
        y_bottom: i32,
        color: Color,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        self.canvas
            .fill_rect(Rect::new(x, y_top, 1, (y_bottom - y_top) as u32))?;

        Ok(())
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.present();

        Ok(())
    }

    /*fn draw_background(&mut self, context: &GameContext) {
        let color = Color::RED;
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }*/
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

    let mut time = SystemTime::now(); //time of current frame

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "raycaster-rs",
            (SCREEN_WIDTH).try_into().unwrap(),
            (SCREEN_HEIGHT).try_into().unwrap(),
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut renderer = Renderer::new(window)?;


    'running: loop {
        for x in 0..(SCREEN_WIDTH - 1) {
            let camera_x: f64 = 2.0 * (x as f64) / (SCREEN_WIDTH as f64) - 1.0;
            let raydir_x: f64 = player_direction.x + plane_position.x * camera_x;
            let raydir_y: f64 = player_direction.y + plane_position.y * camera_x;

            let mut map_x: i32 = player_position.x as i32;
            let mut map_y: i32 = player_position.y as i32;

            let mut side_dist_x: f64;
            let mut side_dist_y: f64;

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

            let step_x: i32;
            let step_y: i32;
            let mut hit: i32 = 0;
            let mut side: i32 = 0;

            if raydir_x < 0.0 {
                step_x = -1;
                side_dist_x = (player_position.x - map_x as f64) * delta_dist_x;
            } else {
                step_x = 1;
                side_dist_x = (map_x as f64 + 1.0 - player_position.x) * delta_dist_x;
            }
            if raydir_y < 0.0 {
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
            let perpwalldist = if side == 0 {
                side_dist_x - delta_dist_x
            } else {
                side_dist_y - delta_dist_y
            };

            let line_height = (SCREEN_HEIGHT as f64 / perpwalldist) as i32;
            let mut draw_start: i32 = -line_height / 2 + SCREEN_HEIGHT as i32 / 2;
            if draw_start < 0 {
                draw_start = 0
            };
            let mut draw_end: i32 = line_height / 2 + SCREEN_HEIGHT as i32 / 2;
            if draw_end >= SCREEN_HEIGHT as i32 {
                draw_end = SCREEN_HEIGHT as i32 - 1
            };

            let mut color = match WORLD_MAP[map_x as usize][map_y as usize] {
                1 => Color::RED,
                2 => Color::GREEN,
                3 => Color::BLUE,
                4 => Color::WHITE,
                _ => Color::YELLOW,
            };

            if side == 1 {
                color = Color {
                    r: color.r / 2,
                    g: color.g / 2,
                    b: color.b / 2,
                    a: color.a,
                }
            };
            renderer.draw_vertical_line(x as i32, draw_start, draw_end, color)?;
        }

        renderer.draw()?;
        let elapsed = time.elapsed().unwrap();
        let sleep_time = Duration::new(0, 1_000_000_000u32 / 30);
        let time_diff = if elapsed > sleep_time {Duration::new(0, 1u32)} else {sleep_time - elapsed};
        time = SystemTime::now();
        ::std::thread::sleep(time_diff);

        renderer.canvas.set_draw_color(Color {
            a: 100,
            r: 40,
            g: 40,
            b: 40,
        });
        renderer.canvas.clear();
        renderer.draw_box(
            0,
            0,
            SCREEN_WIDTH as i32,
            SCREEN_HEIGHT as i32 / 2,
            Color {
                a: 100,
                r: 135,
                g: 206,
                b: 235,
            },
        )?;

        let frame_time: f64 = 0.016;
        let move_speed: f64 = frame_time * 5.0;
        let rotation_speed: f64 = frame_time * 3.0;

        let move_x: f64 = player_direction.x * move_speed;
        let move_y: f64 = player_direction.y * move_speed;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W | Keycode::K => {
                        if WORLD_MAP[(player_position.x + move_x) as usize]
                            [player_position.y as usize]
                            == 0
                        {
                            player_position.x += move_x
                        };
                        if WORLD_MAP[player_position.x as usize]
                            [(player_position.y + move_y) as usize]
                            == 0
                        {
                            player_position.y += move_y
                        };
                    }
                    Keycode::S | Keycode::J => {
                        if WORLD_MAP[(player_position.x - move_x) as usize]
                            [player_position.y as usize]
                            == 0
                        {
                            player_position.x -= move_x
                        };
                        if WORLD_MAP[player_position.x as usize]
                            [(player_position.y - move_y) as usize]
                            == 0
                        {
                            player_position.y -= move_y
                        };
                    }
                    Keycode::A | Keycode::H => {
                        let old_dir_x = player_direction.x;
                        player_direction.x = player_direction.x * rotation_speed.cos()
                            - player_direction.y * rotation_speed.sin();
                        player_direction.y = old_dir_x * rotation_speed.sin()
                            + player_direction.y * rotation_speed.cos();
                        let old_plane_x = plane_position.x;
                        plane_position.x = plane_position.x * rotation_speed.cos()
                            - plane_position.y * rotation_speed.sin();
                        plane_position.y = old_plane_x * rotation_speed.sin()
                            + plane_position.y * rotation_speed.cos();
                    }
                    Keycode::D | Keycode::L => {
                        let negative_rotation_speed = -rotation_speed;
                        let old_dir_x = player_direction.x;
                        player_direction.x = player_direction.x * negative_rotation_speed.cos()
                            - player_direction.y * negative_rotation_speed.sin();
                        player_direction.y = old_dir_x * negative_rotation_speed.sin()
                            + player_direction.y * negative_rotation_speed.cos();
                        let old_plane_x = plane_position.x;
                        plane_position.x = plane_position.x * negative_rotation_speed.cos()
                            - plane_position.y * negative_rotation_speed.sin();
                        plane_position.y = old_plane_x * negative_rotation_speed.sin()
                            + plane_position.y * negative_rotation_speed.cos();
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    Ok(())
}
