// TODO
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

use std::sync::Arc;

use macroquad::prelude::*;
use parking_lot::RwLock;

use crate::{
    capsule::{Capsule, obj::iter_all_objects, parser::parse_capsule},
    event::update::update_events,
    layout::computer::compute_layout,
    renderer::full::render_capsule,
};

pub mod capsule;
pub mod event;
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
    struct DebugView {
        pub show_mouse_hit: bool,
    }

    fn render_debug_view(debug_view: &DebugView, capsule: &Capsule) {
        if debug_view.show_mouse_hit {
            let mouse_position = Vec2::from(mouse_position());

            iter_all_objects(capsule, |e| {
                let bb = e.map(|o| o.bounding_box());

                if bb.contains(mouse_position) {
                    draw_rectangle(bb.x, bb.y, bb.w, bb.h, Color::from_rgba(255, 255, 0, 128));
                }
            });
        }
    }

    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    {
        use parking_lot::deadlock;
        use std::{thread, time::Duration};

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(10));
                let deadlocks = deadlock::check_deadlock();
                if deadlocks.is_empty() {
                    continue;
                }

                println!("{} deadlocks detected", deadlocks.len());
                for (i, threads) in deadlocks.iter().enumerate() {
                    println!("Deadlock #{i}");
                    for t in threads {
                        println!("Thread Id {:#?}", t.thread_id());
                        println!("{:#?}", t.backtrace());
                    }
                }
            }
        });
    }

    let mut capsule = parse_capsule(&std::fs::read_to_string("test.capsule").unwrap())
        .expect("failed to parse capsule");
    compute_layout(&mut capsule);

    let mut capsule_arc = Arc::new(RwLock::new(capsule));
    Capsule::run_scripts(&capsule_arc.clone());

    let mut debug_view = DebugView {
        show_mouse_hit: false,
    };

    loop {
        if is_key_pressed(KeyCode::F5) {
            match parse_capsule(&std::fs::read_to_string("test.capsule").unwrap()) {
                Ok(mut cap) => {
                    compute_layout(&mut cap);
                    let cap = Arc::new(RwLock::new(cap));
                    capsule_arc = cap;
                    Capsule::run_scripts(&capsule_arc.clone());
                    log::info!("Reloaded!");
                }
                Err(e) => {
                    log::error!("failed to parse capsule: {e:#?}");
                }
            }
        }

        if is_key_pressed(KeyCode::F1) {
            debug_view.show_mouse_hit = !debug_view.show_mouse_hit;
        }

        {
            let mut cap = capsule_arc.write();
            update_events(&mut cap);
        }

        {
            let cap = capsule_arc.read();
            clear_background(BLACK);
            render_capsule(&cap);
            render_debug_view(&debug_view, &cap);
        }

        next_frame().await;
    }
}
