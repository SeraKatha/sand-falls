use macroquad::prelude::*;

pub struct View {
    center : Vec2,
    zoom : f32,
    world_size : Vec2,
}

impl View {
    pub fn new(world_size : Vec2) -> Self {
        return Self {
            center : world_size / 2.0,
            zoom : 1.0 / world_size.x,
            world_size : world_size,
        }
    }

    pub fn into_camera_2d(&self) -> Camera2D {
        return Camera2D {
            zoom: self.zoom * vec2(1., screen_width() / screen_height()),
            target: self.center,
            ..Default::default()
        };
    }

    pub fn update(&mut self) {
        const MOVE_SPEED : f32 = 1.0;
        let frame_time = macroquad::time::get_frame_time();
        if macroquad::input::is_key_down(KeyCode::W) {
            self.center.y -= MOVE_SPEED * frame_time / self.zoom;
        }
        if macroquad::input::is_key_down(KeyCode::A) {
            self.center.x -= MOVE_SPEED * frame_time / self.zoom;
        }
        if macroquad::input::is_key_down(KeyCode::S) {
            self.center.y += MOVE_SPEED * frame_time / self.zoom;
        }
        if macroquad::input::is_key_down(KeyCode::D) {
            self.center.x += MOVE_SPEED * frame_time / self.zoom;
        }
        self.center.x = self.center.x.clamp(0.0, self.world_size.x);
        self.center.y = self.center.y.clamp(0.0, self.world_size.y);
        if macroquad::input::is_key_down(KeyCode::Q) {
            self.zoom -= 1.0 * frame_time * self.zoom;
        }
        if macroquad::input::is_key_down(KeyCode::E) {
            self.zoom += 1.0 * frame_time * self.zoom;
        }
    }
}