use macroquad::prelude::*;

pub const NAME: &'static str = "Wordle - Francis Lee";
pub const WIDTH: f32 = 500.0;
pub const HEIGHT: f32 = 600.0;

pub fn config() -> Conf {
    // initialize random seed
    rand::srand(macroquad::miniquad::date::now() as _);
    Conf {
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_title: NAME.to_string(),
        ..Default::default()
    }
}
