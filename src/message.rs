use macroquad::prelude::*;

#[derive(Debug)]
pub enum Style {
    Curved(f32, f32), // width, height
    Rectangle, // width, height
    Clear,
}

#[derive(Debug)]
pub struct Message {
    message: String,
    font_size: u16,
    pos: Vec2,
    lifetime: Option<usize>, // ticks
    current: usize, // ticks
    style: Style,
    fg: Color,
    bg: Color,
}

impl Message {
    pub fn draw(&self) {
        match self.style {
            Style::Rectangle => {
                // mesaure the text
                let dims = measure_text(&self.message[..], None, self.font_size, 1.0);
                let rect_scale = 1.4;
                // drawing rect around the text
                // test: drawing it in center of screen

                draw_rectangle(
                    self.pos.x - dims.width * 0.5 - dims.width * ((rect_scale - 1.0) * 0.5),
                    self.pos.y - dims.height * 0.5 - dims.height * ((rect_scale - 1.0) * 0.5),
                    dims.width * rect_scale,
                    dims.height * rect_scale,
                    self.bg
                    );
                draw_text(&self.message[..], self.pos.x - dims.width * 0.5, self.pos.y + dims.height * 0.5, self.font_size as f32,  self.fg);
            },
            Style::Curved(w, h) => {
                unimplemented!();
            },
            Style::Clear => {
                // drawing text in center
                let dims = measure_text(&self.message[..], None, self.font_size, 1.0);
                draw_text(&self.message[..], self.pos.x - dims.width * 0.5, self.pos.y + dims.height * 0.5, self.font_size as f32, self.fg);
            }
        }
    }

    pub fn update(&mut self) {
        self.current += 1;
    }
}

#[derive(Debug)]
pub struct MessageHandler {
    messages: Vec<Message>,
}

impl MessageHandler {
    pub fn new() -> Self {
        Self { messages: Vec::new() }
    }

    pub fn draw(&self) {
        for message in self.messages.iter() {
            message.draw();
        }
    }

    pub fn add(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn update(&mut self) {
        let mut to_del = vec![];
        for (idx, message) in self.messages.iter_mut().enumerate() {
            if let Some(lifetime) = message.lifetime {
                if message.current >= lifetime {
                    to_del.push(idx);
                }
            }
            message.update();
        }
        for idx in to_del {
            self.messages.remove(idx);
        }
    }
}

pub struct MessageSettings {
    message: String,
    lifetime: Option<usize>, // ticks
    current: usize, // ticks
    style: Style,
    fg: Color,
    bg: Color,
    pos: Vec2,
    font_size: u16,
}

impl MessageSettings {
    pub fn blueprint() -> Self {
        Self {
            message: "".to_string(),
            lifetime: None, // infinite
            current: 0,
            pos: Vec2::new(0., 0.),
            style: Style::Clear,
            fg: WHITE,
            bg: BLACK,
            font_size: 50,
        }
    }

    pub fn font_size(mut self, size: u16) -> Self {
        self.font_size = size;
        self
    }

    pub fn message(mut self, msg: &str) -> Self {
        self.message = msg.to_string();
        self
    }

    pub fn lifetime(mut self, lifetime: usize) -> Self {
        self.lifetime = Some(lifetime);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn pos(mut self, x: f32, y: f32) -> Self {
        self.pos = Vec2::new(x, y);
        self
    }

    pub fn fg(mut self, colour: Color) -> Self {
        self.fg = colour;
        self
    }

    pub fn bg(mut self, colour: Color) -> Self {
        self.bg = colour;
        self
    }

    pub fn build(self) -> Message {
        Message {
            message: self.message,
            lifetime: self.lifetime,
            current: self.current,
            style: self.style,
            fg: self.fg,
            bg: self.bg,
            pos: self.pos,
            font_size: self.font_size,
        }
    }
}
