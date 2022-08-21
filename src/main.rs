use macroquad::prelude::*;
use wordle::{GameSettings, Game, Finished};
use settings::*;

pub mod settings;
pub mod message;
pub mod wordle;

pub const GREEN: Color = color_u8!(83, 141, 78, 255);
pub const ORANGE: Color = color_u8!(181, 159, 59, 255);
pub const GREY: Color = color_u8!(58, 58, 60, 255);

fn reset() -> Game {
  GameSettings::blueprint()
        .width((WIDTH - 100.) as usize)
        .height((HEIGHT - 100.) as usize)
        .pos(50.0, 50.0)
        .rows(6)
        .build()
}


#[macroquad::main(config)]
async fn main() {
    let mut game = reset();
    let mut bg = wordle::GradientEffect::constant_random(120);

    loop {
        clear_background(bg.current);
        game.draw();
        game.update();

        bg.constant_update();

        if is_key_pressed(KeyCode::Space) {
            match game.finished() {
                Finished::Won => game = reset(),
                Finished::Lost => game = reset(),
                _ => (),
            }
        }

        next_frame().await;
    }
}

