use std::{env, fs};

use raylib::prelude::*;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct Presentation {
    header: PresentationHeader,
    slide: Option<Vec<Slide>>,
}

#[derive(Deserialize, Debug)]
struct PresentationHeader {
    title: Option<String>,
    author: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Slide {
    title: Option<String>,
    contents: Option<String>,
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let pres_string = fs::read_to_string(args[1].clone()).unwrap(); // FIXME: Handle the exception

    let pres: Presentation = toml::from_str(pres_string.as_str()).unwrap();

    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title(
            pres.header
                .title
                .unwrap_or("The Presenter".to_string())
                .as_str(),
        )
        .resizable()
        .build();

    let mut slide: usize = 0;

    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(consts::MouseButton::MOUSE_LEFT_BUTTON) {
            if let Some(slides) = &pres.slide {
                if slide + 1 < slides.len() {
                    slide += 1;
                }
            }
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        if let Some(slides) = &pres.slide {
            if let Some(title) = &slides[slide].title {
                d.draw_text(title.as_str(), 5, 5, 36, Color::BLACK);
            }

            if let Some(contents) = &slides[slide].contents {
                d.draw_text(contents.as_str(), 5, 35, 24, Color::BLACK);
            }
        }
    }
}
