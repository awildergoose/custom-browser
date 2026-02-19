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
pub mod renderer;

#[macroquad::main("Capsule Browser")]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let mut capsule = parse_capsule(&std::fs::read_to_string("test.capsule").unwrap());
    compute_layout(&mut capsule);

    log::info!("capsule: {capsule:?}");

    loop {
        clear_background(BLACK);

        render_capsule(&capsule);

        next_frame().await;
    }
}
