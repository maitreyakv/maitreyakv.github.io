use std::sync::{Arc, Mutex, OnceLock};

use derive_builder::Builder;
use gloo::{events::EventListener, render::request_animation_frame};
use rand::{Rng, SeedableRng, seq::IndexedRandom};
use sycamore::prelude::*;
use web_sys::{Element, wasm_bindgen::JsCast};

use crate::utils::WindowDims;

const STAR_COLORS: [&str; 5] = ["#a4c2ff", "#cadaff", "#fff6ed", "#ffcea6", "#ffb16e"];
const STAR_DENSITY: f64 = 0.00075; // stars per square pixel
const HORIZONTAL_SPEED_MULT: f64 = 7.5;
const TRANSITION_HEAD_START: f64 = 100.; // milliseconds

#[component(inline_props)]
pub fn Starscape(state: ReadSignal<State>) -> View {
    let window_dims = create_signal(WindowDims::now());
    let _resize_listener_handle =
        create_signal(EventListener::new(&window(), "resize", move |_event| {
            window_dims.set(WindowDims::now());
        }));

    view! {
        div(class="fixed bg-black") {
            svg(height="100vh", width="100vw", xmlns="http://www.w3.org/2000/svg") {
                (move || StarController::new(window_dims.get()).view(state))
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum State {
    Down,
    Right,
    Left,
}

static SVG_ANIMATION_START_TIME: OnceLock<f64> = OnceLock::new();

#[derive(Clone)]
struct StarController {
    stars: Arc<Mutex<Vec<Star>>>,
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
                    .node_ref(create_node_ref())
                    .window_dims(window_dims)
                    .build()
                    .unwrap()
            })
            .collect::<Vec<Star>>();
        Self {
            stars: Arc::new(Mutex::new(stars)),
        }
    }

    fn view(self, state: ReadSignal<State>) -> View {
        let views = self
            .stars
            .lock()
            .unwrap()
            .iter()
            .map(|star| star.view())
            .collect::<Vec<View>>();

        let frame = create_signal(request_animation_frame(|_| {}));
        on_mount(move || {
            create_effect(move || {
                let state = state.get();
                let stars = self.stars.clone();
                frame.set(request_animation_frame({
                    move |t| {
                        for star in stars.lock().unwrap().iter_mut() {
                            star.update_position();
                        }
                        for star in stars.lock().unwrap().iter() {
                            let t_corrected = t - SVG_ANIMATION_START_TIME.get_or_init(|| t);
                            star.set(t_corrected, &state);
                        }
                    }
                }))
            })
        });

        view! {
            (views)
        }
    }
}

#[derive(Builder, Clone)]
struct Star {
    x: f64,
    y: f64,
    depth: f64,
    color: &'static str,
    node_ref: NodeRef,
    window_dims: WindowDims,
}
impl Star {
    fn view(&self) -> View {
        let radius = self.radius().to_string();
        let x = self.x.to_string();
        let y = self.y.to_string();
        view! {
            circle(
                r#ref=self.node_ref,
                r=radius,
                fill=self.color,
                cx=x,
                cy=y,
                transformOrigin="center"
            ) {
                animate(repeatCount="indefinite")
            }
        }
    }

    fn update_position(&mut self) {
        let element = self.node_ref.get().dyn_into::<Element>().unwrap();
        let rect = element.get_bounding_client_rect();
        self.x = 0.5 * (rect.left() + rect.right());
        self.y = 0.5 * (rect.top() + rect.bottom());
    }

    fn set(&self, t: f64, state: &State) {
        let circle = self.node_ref.get().dyn_into::<Element>().unwrap();
        let animate = circle.first_child().unwrap();
        let animate_new = animate
            .clone_node_with_deep(false)
            .unwrap()
            .dyn_into::<Element>()
            .unwrap();
        let base_speed = self.speed();
        match state {
            State::Left => {
                animate_new.set_attribute("from", "100%").unwrap();
                animate_new.set_attribute("to", "0%").unwrap();
            }
            State::Right | State::Down => {
                animate_new.set_attribute("from", "0%").unwrap();
                animate_new.set_attribute("to", "100%").unwrap();
            }
        };
        match state {
            State::Right | State::Left => {
                circle.set_attribute("cy", &self.y.to_string()).unwrap();
                animate_new.set_attribute("attributeName", "cx").unwrap();
            }
            State::Down => {
                circle.set_attribute("cx", &self.x.to_string()).unwrap();
                animate_new.set_attribute("attributeName", "cy").unwrap();
            }
        }
        match state {
            State::Down => {
                let dur = 0.001 * self.window_dims.height / base_speed;
                animate_new.set_attribute("dur", &dur.to_string()).unwrap();
                let mut start = -1.0 * self.y / base_speed;
                start = 0.001 * (start + t - TRANSITION_HEAD_START);
                animate_new
                    .set_attribute("begin", &start.to_string())
                    .unwrap();
            }
            State::Right => {
                let speed = HORIZONTAL_SPEED_MULT * base_speed;
                let dur = 0.001 * self.window_dims.width / speed;
                animate_new.set_attribute("dur", &dur.to_string()).unwrap();
                let mut start = -1.0 * self.x / speed;
                start = 0.001 * (start + t - TRANSITION_HEAD_START);
                animate_new
                    .set_attribute("begin", &start.to_string())
                    .unwrap();
            }
            State::Left => {
                let speed = HORIZONTAL_SPEED_MULT * base_speed;
                let dur = 0.001 * self.window_dims.width / speed;
                animate_new.set_attribute("dur", &dur.to_string()).unwrap();
                let mut start = -1.0 * (self.window_dims.width - self.x) / speed;
                start = 0.001 * (start + t - TRANSITION_HEAD_START);
                animate_new
                    .set_attribute("begin", &start.to_string())
                    .unwrap();
            }
        }
        circle.replace_child(&animate_new, &animate).unwrap();
    }

    fn radius(&self) -> f64 {
        4.0 - self.depth * 2.5
    }

    fn speed(&self) -> f64 {
        0.08 - self.depth * 0.04
    }
}
