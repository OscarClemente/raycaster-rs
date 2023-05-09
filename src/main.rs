extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::{event::Event, EventPump};
use std::time::{Duration, SystemTime};

const MAP_WIDTH: usize = 24;
const MAP_HEIGHT: usize = 24;
const SCREEN_WIDTH: usize = 1280;
const SCREEN_HEIGHT: usize = 720;
const TEXTURE_WIDTH: usize = 64;
const TEXTURE_HEIGHT: usize = 64;

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
struct Vec2D<T> {
    x: T,
    y: T,
}

impl<T> Vec2D<T> {
    pub fn new(x: T, y: T) -> Vec2D<T> {
        Vec2D { x, y }
    }
}

pub struct Renderer {
    canvas: WindowCanvas,
    buffer: Vec<Vec<u32>>,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer {
            canvas,
            buffer: vec![vec![0u32]],
        })
    }

    fn calculate_line(&mut self, game_context: GameContext, x: f64) -> (i32, i32, Color) {
        let camera_x: f64 = 2.0 * x / (SCREEN_WIDTH as f64) - 1.0;
        let raydir = Vec2D::new(
            game_context.player_direction.x + game_context.plane_position.x * camera_x,
            game_context.player_direction.y + game_context.plane_position.y * camera_x,
        );
        let map = Vec2D::new(
            game_context.player_position.x as i32,
            game_context.player_position.y as i32,
        );

        let mut delta_dist = Vec2D::new(0.0, 0.0);

        delta_dist.x = if raydir.x == 0.0 {
            1e30
        } else {
            (1.0 / raydir.x).abs()
        };
        delta_dist.y = if raydir.y == 0.0 {
            1e30
        } else {
            (1.0 / raydir.y).abs()
        };

        let (step, side_dist) = self.calc_step_and_side_dist(game_context, raydir, map, delta_dist);
        let (side_dist, map, side) = self.calc_dda(map, step, side_dist, delta_dist);

        let perpwalldist = if side == 0 {
            side_dist.x - delta_dist.x
        } else {
            side_dist.y - delta_dist.y
        };

        let (line_height, draw_start, draw_end) = self.calculate_draw_start_and_end(perpwalldist);
        // for textures

        let color = self.get_color(map, side);

        // textures stuff
        let text_num = WORLD_MAP[map.x as usize][map.y as usize];
        let mut wall_x = if side == 0 {
            game_context.player_position.y + perpwalldist * raydir.y
        } else {
            game_context.player_position.x + perpwalldist * raydir.x
        };
        wall_x -= wall_x.floor();
        let mut tex_x = (wall_x * TEXTURE_WIDTH as f64) as i32;
        if side == 0 && raydir.x > 0.0 { tex_x = TEXTURE_WIDTH as i32 - tex_x - 1};
        if side == 1 && raydir.y < 0.0 { tex_x = TEXTURE_WIDTH as i32 - tex_x - 1};

        let tex_step = 1.0 * TEXTURE_HEIGHT as f64 / line_height as f64;
        let mut tex_pos = (draw_start - SCREEN_HEIGHT as i32 / 2 + line_height / 2) as f64 * tex_step;

        for y in draw_start..draw_end {
            let tex_y = tex_pos as i32 & (TEXTURE_HEIGHT as i32 - 1);
            tex_pos += tex_step;
        /*    //
        Uint32 color = texture[texNum][texHeight * texY + texX];
        //make color darker for y-sides: R, G and B byte each divided through two with a "shift" and an "and"
        if(side == 1) color = (color >> 1) & 8355711;
        buffer[y][x] = color;
        */}

        return (draw_start, draw_end, color);
    }

    fn calc_step_and_side_dist(
        &mut self,
        game_context: GameContext,
        raydir: Vec2D<f64>,
        map: Vec2D<i32>,
        delta_dist: Vec2D<f64>,
    ) -> (Vec2D<i32>, Vec2D<f64>) {
        let mut side_dist = Vec2D::new(0.0, 0.0);
        let mut step = Vec2D::new(0, 0);

        if raydir.x < 0.0 {
            step.x = -1;
            side_dist.x = (game_context.player_position.x - map.x as f64) * delta_dist.x;
        } else {
            step.x = 1;
            side_dist.x = (map.x as f64 + 1.0 - game_context.player_position.x) * delta_dist.x;
        }
        if raydir.y < 0.0 {
            step.y = -1;
            side_dist.y = (game_context.player_position.y - map.y as f64) * delta_dist.y;
        } else {
            step.y = 1;
            side_dist.y = (map.y as f64 + 1.0 - game_context.player_position.y) * delta_dist.y;
        }

        return (step, side_dist);
    }

    fn calc_dda(
        &mut self,
        mut map: Vec2D<i32>,
        step: Vec2D<i32>,
        mut side_dist: Vec2D<f64>,
        delta_dist: Vec2D<f64>,
    ) -> (Vec2D<f64>, Vec2D<i32>, i32) {
        let mut side: i32;

        loop {
            if side_dist.x < side_dist.y {
                side_dist.x += delta_dist.x;
                map.x += step.x;
                side = 0;
            } else {
                side_dist.y += delta_dist.y;
                map.y += step.y;
                side = 1;
            }
            if WORLD_MAP[map.x as usize][map.y as usize] > 0 {
                break;
            }
        }

        return (side_dist, map, side);
    }

    fn calculate_draw_start_and_end(&mut self, wall_distance: f64) -> (i32, i32, i32) {
        let line_height = (SCREEN_HEIGHT as f64 / wall_distance) as i32;
        let mut draw_start: i32 = -line_height / 2 + SCREEN_HEIGHT as i32 / 2;
        if draw_start < 0 {
            draw_start = 0
        };
        let mut draw_end: i32 = line_height / 2 + SCREEN_HEIGHT as i32 / 2;
        if draw_end >= SCREEN_HEIGHT as i32 {
            draw_end = SCREEN_HEIGHT as i32 - 1
        };

        return (line_height, draw_start, draw_end);
    }

    fn get_color(&mut self, map: Vec2D<i32>, side: i32) -> Color {
        let mut color = match WORLD_MAP[map.x as usize][map.y as usize] {
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

        return color;
    }

    fn draw_screen(&mut self, game_context: GameContext) -> Result<(), String> {
        for x in 0..(SCREEN_WIDTH - 1) {
            let (draw_start, draw_end, color) = self.calculate_line(game_context, x as f64);
            self.draw_vertical_line(x as i32, draw_start, draw_end, color)?;
        }

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

    pub fn draw(&mut self, game_context: GameContext) -> Result<(), String> {
        self.draw_screen(game_context)?;
        self.canvas.present();

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct GameContext {
    player_position: Vec2D<f64>,
    player_direction: Vec2D<f64>,
    plane_position: Vec2D<f64>,
    time: SystemTime,
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            player_position: Vec2D::new(22.0, 12.0),
            player_direction: Vec2D::new(-1.0, 0.0),
            plane_position: Vec2D::new(0.0, 0.66),
            time: SystemTime::now(), //time of current frame
        }
    }
}

pub struct KeyboardEventHandler {
    event_pump: EventPump,
}

impl KeyboardEventHandler {
    pub fn new(event_pump: EventPump) -> KeyboardEventHandler {
        KeyboardEventHandler { event_pump }
    }

    pub fn handle_events(&mut self, game_context: &mut GameContext) -> bool {
        let frame_time: f64 = 0.016;
        let move_speed: f64 = frame_time * 5.0;
        let rotation_speed: f64 = frame_time * 3.0;

        let movement = Vec2D::new(
            game_context.player_direction.x * move_speed,
            game_context.player_direction.y * move_speed,
        );

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    return true;
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Q => {
                        return true;
                    }
                    Keycode::W | Keycode::K => {
                        if WORLD_MAP[(game_context.player_position.x + movement.x) as usize]
                            [game_context.player_position.y as usize]
                            == 0
                        {
                            game_context.player_position.x += movement.x
                        };
                        if WORLD_MAP[game_context.player_position.x as usize]
                            [(game_context.player_position.y + movement.y) as usize]
                            == 0
                        {
                            game_context.player_position.y += movement.y
                        };
                    }
                    Keycode::S | Keycode::J => {
                        if WORLD_MAP[(game_context.player_position.x - movement.x) as usize]
                            [game_context.player_position.y as usize]
                            == 0
                        {
                            game_context.player_position.x -= movement.x
                        };
                        if WORLD_MAP[game_context.player_position.x as usize]
                            [(game_context.player_position.y - movement.y) as usize]
                            == 0
                        {
                            game_context.player_position.y -= movement.y
                        };
                    }
                    Keycode::A | Keycode::H => {
                        let old_dir_x = game_context.player_direction.x;
                        game_context.player_direction.x = game_context.player_direction.x
                            * rotation_speed.cos()
                            - game_context.player_direction.y * rotation_speed.sin();
                        game_context.player_direction.y = old_dir_x * rotation_speed.sin()
                            + game_context.player_direction.y * rotation_speed.cos();
                        let old_plane_x = game_context.plane_position.x;
                        game_context.plane_position.x = game_context.plane_position.x
                            * rotation_speed.cos()
                            - game_context.plane_position.y * rotation_speed.sin();
                        game_context.plane_position.y = old_plane_x * rotation_speed.sin()
                            + game_context.plane_position.y * rotation_speed.cos();
                    }
                    Keycode::D | Keycode::L => {
                        let negative_rotation_speed = -rotation_speed;
                        let old_dir_x = game_context.player_direction.x;
                        game_context.player_direction.x = game_context.player_direction.x
                            * negative_rotation_speed.cos()
                            - game_context.player_direction.y * negative_rotation_speed.sin();
                        game_context.player_direction.y = old_dir_x * negative_rotation_speed.sin()
                            + game_context.player_direction.y * negative_rotation_speed.cos();
                        let old_plane_x = game_context.plane_position.x;
                        game_context.plane_position.x = game_context.plane_position.x
                            * negative_rotation_speed.cos()
                            - game_context.plane_position.y * negative_rotation_speed.sin();
                        game_context.plane_position.y = old_plane_x * negative_rotation_speed.sin()
                            + game_context.plane_position.y * negative_rotation_speed.cos();
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        return false;
    }
}

fn main() -> Result<(), String> {
    let mut game_context = GameContext::new();
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

    let event_pump = sdl_context.event_pump()?;
    let mut keyboard_event_handler = KeyboardEventHandler::new(event_pump);

    let mut renderer = Renderer::new(window)?;
    let mut textures: [Vec<u32>; 8] = Default::default();

    // generate textures
    for x in 0..TEXTURE_WIDTH {
        for y in 0..TEXTURE_HEIGHT {
            let xorcolor = ((x * 256 / TEXTURE_WIDTH) ^ (y * 256 / TEXTURE_HEIGHT)) as u32;
            //int xcolor = x * 256 / TEXTURE_WIDTH;
            let ycolor = (y * 256 / TEXTURE_HEIGHT) as u32;
            let xycolor = (y * 128 / TEXTURE_HEIGHT + x * 128 / TEXTURE_WIDTH) as u32;
            let cross_filter = (x != y && x != TEXTURE_WIDTH - y) as u32; //flat red texture with black cross
            let brick_padding = ((x % 16) | (y % 16)) as u32;
            textures[0].push(65536 * 254 * cross_filter); //flat red texture with black cross
            textures[1].push(xycolor + 256 * xycolor + 65536 * xycolor); //sloped greyscale
            textures[2].push(256 * xycolor + 65536 * xycolor); //sloped yellow gradient
            textures[3].push(xorcolor + 256 * xorcolor + 65536 * xorcolor); //xor greyscale
            textures[4].push(256 * xorcolor); //xor green
            textures[5].push(65536 * 192 * brick_padding);
            textures[6].push(65536 * ycolor); //red gradient
            textures[7].push(128 + 256 * 128 + 65536 * 128); //flat grey texture
        }
    }

    loop {
        renderer.draw(game_context)?;
        let elapsed = game_context.time.elapsed().unwrap();
        let sleep_time = Duration::new(0, 1_000_000_000u32 / 30);
        let time_diff = if elapsed > sleep_time {
            Duration::new(0, 1u32)
        } else {
            sleep_time - elapsed
        };
        game_context.time = SystemTime::now();
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

        let quit = keyboard_event_handler.handle_events(&mut game_context);
        if quit {
            break;
        }
    }

    Ok(())
}
