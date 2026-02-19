// TODO
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

use macroquad::prelude::*;

use crate::{
    capsule::parser::parse_capsule, layout::computer::compute_layout,
    renderer::full::render_capsule,
};

pub mod capsule;
pub mod layout;
pub mod lua;
pub mod renderer;

pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 600;

fn window_conf() -> Conf {
    Conf {
        window_title: "Capsule Browser".into(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let mut capsule = parse_capsule(&std::fs::read_to_string("test.capsule").unwrap())
        .expect("failed to parse capsule");
    compute_layout(&mut capsule);
    capsule.run_scripts();

    loop {
        if is_key_pressed(KeyCode::F5) {
            capsule = parse_capsule(&std::fs::read_to_string("test.capsule").unwrap())
                .expect("failed to parse capsule");
            compute_layout(&mut capsule);
            capsule.run_scripts();
            log::info!("Reloaded!");
        }

        clear_background(BLACK);
        render_capsule(&capsule);
        next_frame().await;
    }
}
