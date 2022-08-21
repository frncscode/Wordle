use macroquad::prelude::*;
use std::io::prelude::*;
use std::time::Duration;
use std::time::Instant;
use crate::settings::*;
use std::fs::File;
use crate::message::{
    MessageSettings,
    MessageHandler,
    Style,
};
use crate::{
    GREY,
    ORANGE,
    GREEN,
};

pub fn random_colour() -> Color {
    color_u8!(rand::gen_range(0, 255), rand::gen_range(0, 255), rand::gen_range(0, 255), 255)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientEffect {
    start: Color,
    end: Color,
    pub current: Color,
    length: usize,
    ticks: usize,
}

impl GradientEffect {
    pub fn new(start: Color, end: Color, length: usize) -> Self {
        Self {
            start,
            current: start,
            end,
            length,
            ticks: 0,
        }
    }

    pub fn constant_random(interval: usize) -> Self {
        let start = random_colour();
        Self {
            start: start,
            end: random_colour(),
            current: start,
            length: interval,
            ticks: 0,
        }
    }

    pub fn constant_update(&mut self) {
        if self.ticks >= self.length {
            self.start = self.current;
            self.end = random_colour();
            self.ticks = 0;
        }
        self.current = lerpColour(self.start, self.end, self.ticks as f32 / self.length as f32);
        self.ticks += 1;
    }

    pub fn update(&mut self) {
        if self.ticks <= self.length {
            self.current = lerpColour(self.start, self.end, self.ticks as f32 / self.length as f32);
            self.ticks += 1;
        }
    }
}

pub fn lerpColour(c1: Color, c2: Color, progress: f32) -> Color {
    let progress = progress.max(0.).min(1.);

    let r1 = c1.r;
    let r2 = c2.r;
    let result_red = r1 + progress * (r2 - r1);

    let g1 = c1.g;
    let g2 = c2.g;
    let result_green = g1 + progress * (g2 - g1);

    let b1 = c1.b;
    let b2 = c2.b;
    let result_blue = b1 + progress * (b2 - b1);

    Color::new(result_red as f32, result_green as f32, result_blue as f32, 255.)
}

fn load_words() -> Vec<String> {
    let mut file = File::open("C:/Users/fr3nc/dev/rust/wordle/src/words.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut words = vec![];
    for word in contents.split('\n') {
        let word = word.to_string();
        if word.len() == 5 {
            words.push(word);
        }
    }
    words
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Finished {
    Won,
    Lost,
    No,
}

#[derive(Debug)]
struct Effect {
    offset: (f32, f32),
    elapsed: usize,
    length: usize,
    running: bool,
    intensity: f32,
}

impl Effect {
    fn new() -> Self {
        Self {
            offset: (0.0, 0.0),
            elapsed: 0,
            length: 0,
            running: false,
            intensity: 0.0,
        } 
    }

    fn start_shake(&mut self, length: usize, intensity: f32) {
        let intensity = intensity.min(1.).max(0.);

        self.elapsed = 0;
        self.length = length;
        self.running = true;
        self.intensity = intensity;
    }

    fn reset(&mut self) {
        self.elapsed = 0;
        self.length = 0;
        self.running = false;
        self.offset = (0.0, 0.0);
        self.intensity = 0.0;
    }

    fn run(&mut self) {
        if self.running {
            self.offset.0 = rand::gen_range(-40, 40) as f32 * self.intensity;
            self.offset.1 = rand::gen_range(-40, 40) as f32 * self.intensity;
            
            if self.elapsed >= self.length {
                self.reset();
            } else {
                self.elapsed += 1;
            }
        }
    }
}

#[derive(Debug)]
struct Timer {
    secs: usize,
    start: Option<Instant>,
    elapsed: Duration,
}

impl Timer {
    fn new(secs: usize) -> Self {
        Self {
            secs,
            start: None,
            elapsed: Duration::ZERO,
        }
    }

    fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    fn ticktock(&mut self) {
        self.elapsed = self.start.unwrap().elapsed();
    }

    fn finished(&self) -> bool {
        self.elapsed.as_secs() >= self.secs as u64
    }
}



#[derive(Debug, Copy, Clone)]
enum Hint {
    Grey(GradientEffect),
    Orange(GradientEffect),
    Green(GradientEffect),
}

#[derive(Debug)]
pub struct Game {
    messages: MessageHandler,
    rows: usize,
    columns: usize,
    board: Vec<Vec<Option<char>>>,
    commited: Vec<Vec<Option<Hint>>>,
    effector: Effect,
    width: f32,
    height: f32,
    word: String,
    timer: Option<Timer>,
    pos: (f32, f32),
    control: bool,
    finished: Finished,
    selected: (usize, usize),
}

impl Game {
    pub fn draw(&self) {
        let size = (self.width / self.columns as f32, self.height / self.rows as f32);
        let start = self.pos;

        for (r, row) in self.board.iter().enumerate() {
            for (c, chr) in row.iter().enumerate() {
                let (mut x, mut y) = (c as f32 * size.0, r as f32 * size.1);
                x += start.0;
                y += start.1;

                if self.effector.running {
                    x += self.effector.offset.0;
                    y += self.effector.offset.1;
                }

                if self.is_commited(r) {
                    match self.commited[r][c].unwrap() {
                        Hint::Grey(colour) => draw_rectangle(x, y, size.0, size.1, colour.current),
                        Hint::Orange(colour) => draw_rectangle(x, y, size.0, size.1, colour.current),
                        Hint::Green(colour) => draw_rectangle(x, y, size.0, size.1, colour.current),
                    }
                } else {
                    draw_rectangle(x, y, size.0, size.1, BLACK);
                    draw_rectangle_lines(x, y, size.0, size.1, 4.0, DARKGRAY);
                }

                if let Some(chr) = chr {
                    draw_text(&chr.to_string().to_uppercase()[..], x + size.0 * 0.5 - 10.0, y + size.1 * 0.5 + 10.0, 50.0, WHITE);
                }

                self.messages.draw();
            }
        }
    }

    pub fn is_commited(&self, row: usize) -> bool {
        if self.commited[row][0].is_none() {
            return false;
        }
        true
    }

    pub fn finished(&self) -> Finished {
        self.finished
    }

    pub fn control(&mut self) {
        if !(self.finished == Finished::No) {
            // game finished
            return;
        }

        // handling typing
        let key = get_char_pressed();
        // return if no key is pressed
        if key.is_none() { return; }
        let key = key.unwrap();
        
        // typing letters if they are alphabetic
        if key.is_alphabetic() {
            if self.selected.1 < self.columns {
                self.set(self.selected.1, self.selected.0, Some(key));
                // move selected one along
                self.selected.1 += 1;
            }
        } else if is_key_pressed(KeyCode::Enter) && self.selected.1 >= self.columns {
            // commit word and move on
            let res = self.commit_row();

            match res {
                Ok(won) => {
                    if won {
                        self.finished = Finished::Won;

                        let message = MessageSettings::blueprint()
                            .pos(self.width * 0.5 + self.pos.0, self.height * 0.5 + self.pos.0)
                            .message("You Won!")
                            .style(Style::Rectangle)
                            .bg(GREEN)
                            .build();
                        self.messages.add(message);
                        return;
                    } else if self.selected.0 == self.rows - 1 { // the end
                        // lost
                        self.finished = Finished::Lost;
                        let message = MessageSettings::blueprint()
                            .pos(self.width * 0.5 + self.pos.0, self.height * 0.5 + self.pos.0)
                            .message("You Lost!")
                            .style(Style::Rectangle)
                            .bg(RED)
                            .build();
                        self.messages.add(message);
                    }
                    self.selected.1 = 0; // back to beginnning
                    self.selected.0 += 1; // down a row
                },
                Err(word) => {
                    self.effector.start_shake(15, 0.3);
                    let message = MessageSettings::blueprint()
                        .pos(self.width * 0.5 + self.pos.0, self.height * 0.5 + self.pos.0)
                        .message(&format!("{} not in list!", word)[..])
                        .style(Style::Rectangle)
                        .lifetime(30)
                        .font_size(30)
                        .bg(BLUE)
                        .build();
                    self.messages.add(message);
                }
            }
        } else if is_key_pressed(KeyCode::Backspace) && self.selected.1 > 0 {
            // remove last letter
            self.set(self.selected.1 - 1, self.selected.0, None);
            // make last letter space now the selected square
            self.selected.1 -= 1;
        }
    }

    pub fn commit_row(&mut self) -> Result<bool, String> { // return is whether it was a win
        let mut word = String::new();
        for chr in &self.board[self.selected.0][..] {
            word.push(chr.unwrap()); // we know that it will never be NONE
        }

        if !load_words().contains(&word) {
            return Err(word);
        }

        let mut greens = 0;

        for (i, chr) in word.chars().enumerate() {
            // green
            if self.word.chars().nth(i).unwrap() == chr {
                self.commited[self.selected.0][i] = Some(Hint::Green(GradientEffect::new(BLACK, GREEN, 30)));
                greens += 1;
            } else if self.word.contains(chr) { 
                self.commited[self.selected.0][i] = Some(Hint::Orange(GradientEffect::new(BLACK, ORANGE, 30)));
            } else {
                self.commited[self.selected.0][i] = Some(Hint::Grey(GradientEffect::new(BLACK, GREY, 30)));
            }
        }

        if greens == self.columns {
            return Ok(true);
        }
        return Ok(false);
    }

    pub fn update(&mut self) {
        if self.timer.is_some() {
            if self.timer.as_ref().unwrap().finished() {
                if self.finished == Finished::No {
                    self.finished = Finished::Lost;
                    let message = MessageSettings::blueprint()
                        .pos(self.width * 0.5 + self.pos.0, self.height * 0.5 + self.pos.0)
                        .message("You Ran Out of Time!")
                        .style(Style::Rectangle)
                        .font_size(30)
                        .bg(RED)
                        .build();
                    self.messages.add(message);
                }
            }
        }

        // starting timer if not already done
        self.effector.run();
        if self.control {
            self.control();
        }

        // update graidents
        for row in self.commited.iter_mut() {
            for col in row.iter_mut() {
                if let Some(col) = col {
                    match col {
                        Hint::Grey(grad) => grad.update(),
                        Hint::Orange(grad) => grad.update(),
                        Hint::Green(grad) => grad.update(),
                    }
                }
            }
        }

        if self.timer.is_some() {
            let timer = self.timer.as_mut().unwrap();
            timer.ticktock();
        }

        // update all messages
        self.messages.update();
    }

    pub fn at(&self, x: usize, y: usize) -> Option<char> {
        self.board[y][x].clone()
    }

    pub fn set(&mut self, x: usize, y: usize, val: Option<char>) {
        self.board[y][x] = val;
    }

    pub fn board(&self) -> &Vec<Vec<Option<char>>> {
        &self.board
    }
}

#[derive(Debug)]
pub struct GameSettings {
    width: f32,
    columns: usize,
    rows: usize,
    height: f32,
    word: String,
    timer: Option<Timer>,
    pos: (f32, f32),
    control: bool,
}

impl GameSettings {
    pub fn blueprint() -> Self {
        // basically default
        Self {
            width: WIDTH,
            columns: 5,
            rows: 6,
            height: HEIGHT,
            word: Self::_random(), 
            timer: None,
            pos: (0.0, 0.0),
            control: true,
        }
    }

    pub fn pos(mut self, x: f32, y: f32) -> Self {
        self.pos = (x, y);
        self
    }

    pub fn control(mut self, control: bool) -> Self {
        self.control = control;
        self
    }

    pub fn word(mut self, word: String) -> Self {
        self.word = word;
        self
    }

    pub fn width(mut self, width: usize) -> Self {
        self.width = width.max(100).min(WIDTH as usize) as f32;
        self
    }

    pub fn height(mut self, height: usize) -> Self {
        self.height = height.max(100).min(HEIGHT as usize) as f32;
        self
    }

    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows;
        self
    }

    pub fn timer(mut self, secs: usize) -> Self {
        self.timer = Some(Timer::new(secs));
        self
    }

    fn _random() -> String {
        let words = load_words();
        words[rand::gen_range(0, words.len())].to_string()
    }

    pub fn build(mut self) -> Game {
        if self.timer.is_some() {
            self.timer.as_mut().unwrap().start();
        }
        let rows: usize = self.rows;
        let columns: usize = self.columns;
        Game {
            board: vec![vec![None; columns]; rows],
            // board: [[None; columns]; rows],
            // commited: [[None; columns]; rows],
            commited: vec![vec![None; columns]; rows],
            width: self.width,
            height: self.height,
            word: self.word,
            timer: self.timer,
            pos: self.pos,
            control: self.control,
            selected: (0, 0),
            effector: Effect::new(),
            finished: Finished::No,
            rows,
            columns,
            messages: MessageHandler::new(),
        }
    }
}
