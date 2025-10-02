use std::sync::{Arc, Mutex};

use derive_builder::Builder;
use gloo::{events::EventListener, render::request_animation_frame};
use rand::{Rng, SeedableRng, seq::IndexedRandom};
use sycamore::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, wasm_bindgen::JsCast};

use crate::utils::{WindowDims, request_animation_frame_loop};

const TWO_PI: f64 = 2. * std::f64::consts::PI;

const STAR_COLORS: [&str; 5] = ["#a4c2ff", "#cadaff", "#fff6ed", "#ffcea6", "#ffb16e"];
const STAR_DENSITY: f64 = 0.00075; // stars per square pixel
const HORIZONTAL_SPEED_MULT: f64 = 15.;

#[component(inline_props)]
pub fn Starscape(state: ReadSignal<State>) -> View {
    let window_dims = create_signal(WindowDims::now());
    let _resize_listener_handle =
        create_signal(EventListener::new(&window(), "resize", move |_event| {
            window_dims.set(WindowDims::now());
        }));

    let star_controller =
        create_memo(move || Arc::new(Mutex::new(StarController::new(window_dims.get()))));

    let node_ref = create_node_ref();
    let handle = create_signal(request_animation_frame(|_| {}));
    let t_last = create_signal(0.);

    on_mount(move || {
        let canvas_ctx = node_ref
            .get()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap()
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        request_animation_frame_loop(
            move |t| {
                let star_controller = star_controller.get_clone();
                let mut star_controller = star_controller.lock().unwrap();
                let window_dims = window_dims.get();
                let state = state.get();
                canvas_ctx.clear_rect(0., 0., window_dims.width, window_dims.height);
                star_controller.step(t - t_last.get(), &canvas_ctx, &state);
                t_last.set(t);
            },
            handle,
        );
    });

    view! {
        canvas(
            r#ref=node_ref,
            class="fixed bg-black",
            height=move || window_dims.get().height.to_string(),
            width=move || window_dims.get().width.to_string()
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub enum State {
    Down,
    Right,
    Left,
}

#[derive(Clone, Debug)]
struct StarController {
    stars: Vec<Star>,
    window_dims: WindowDims,
}
impl StarController {
    fn new(window_dims: WindowDims) -> Self {
        let mut rng = rand::rngs::StdRng::from_seed([0; 32]);
        let n_stars = (window_dims.area() * STAR_DENSITY) as i64;
        let stars = (0..n_stars)
            .map(move |_| {
                StarBuilder::default()
                    .x(rng.random_range(0.0..=window_dims.width))
                    .y(rng.random_range(0.0..=window_dims.height))
                    .depth(rng.random_range(0.0..=1.0))
                    .color(STAR_COLORS.choose(&mut rng).unwrap())
                    .build()
                    .unwrap()
            })
            .collect::<Vec<Star>>();
        Self { stars, window_dims }
    }

    fn step(&mut self, dt: f64, ctx: &CanvasRenderingContext2d, state: &State) {
        match state {
            State::Down => {
                for star in self.stars.iter_mut() {
                    star.y += star.speed() * dt;
                    star.y = star.y.rem_euclid(self.window_dims.height);
                    star.draw_as_circle(ctx);
                }
            }
            State::Left => {
                for star in self.stars.iter_mut() {
                    star.x -= HORIZONTAL_SPEED_MULT * star.speed() * dt;
                    star.x = star.x.rem_euclid(self.window_dims.width);
                    star.draw_as_ellipse(ctx);
                }
            }
            State::Right => {
                for star in self.stars.iter_mut() {
                    star.x += HORIZONTAL_SPEED_MULT * star.speed() * dt;
                    star.x = star.x.rem_euclid(self.window_dims.width);
                    star.draw_as_ellipse(ctx);
                }
            }
        }
    }
}

#[derive(Builder, Clone, Debug)]
struct Star {
    x: f64,
    y: f64,
    depth: f64,
    color: &'static str,
}
impl Star {
    fn draw_as_circle(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.arc(self.x, self.y, self.radius(), 0., TWO_PI).unwrap();
        ctx.set_fill_style_str(self.color);
        ctx.fill();
        ctx.close_path();
    }

    fn draw_as_ellipse(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.ellipse(
            self.x,
            self.y,
            15. * self.radius(),
            0.2 * self.radius(),
            0.,
            0.,
            TWO_PI,
        )
        .unwrap();
        ctx.set_fill_style_str(self.color);
        ctx.fill();
        ctx.close_path();
    }

    fn radius(&self) -> f64 {
        4.0 - self.depth * 2.5
    }

    fn speed(&self) -> f64 {
        0.08 - self.depth * 0.04
    }
}
