use std::{
    cell::OnceCell,
    path::Path,
    rc::Rc,
    sync::{Arc, Mutex},
};

use derive_builder::Builder;
use gloo::{
    events::EventListener,
    render::{AnimationFrame, request_animation_frame},
    timers::callback::Interval,
};
use rand::{Rng, SeedableRng, seq::IndexedRandom};
use sycamore::{prelude::*, web::tags::SvgAnimate};
use web_sys::{Element, HtmlElement, SvgAnimateElement, SvgCircleElement, wasm_bindgen::JsCast};

use crate::utils::WindowDims;

const STAR_COLORS: [&str; 5] = ["#a4c2ff", "#cadaff", "#fff6ed", "#ffcea6", "#ffb16e"];
const STAR_DENSITY: f64 = 0.00075; // stars per square pixel

#[component(inline_props)]
pub fn Starscape(state: ReadSignal<State>) -> View {
    let window_dims = create_signal(WindowDims::now());
    let _resize_listener_handle =
        create_signal(EventListener::new(&window(), "resize", move |_event| {
            window_dims.set(WindowDims::now());
        }));

    // TEMP: For testing
    //let state = create_signal(State::Down);
    //create_signal(Interval::new(2_000, move || {
    //    state.update(|val| {
    //        *val = match val {
    //            State::Down => State::Left,
    //            State::Right => State::Down,
    //            State::Left => State::Down,
    //        }
    //    });
    //}));

    view! {
        div(class="fixed bg-black -z-999") {
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
            let t_start = Rc::new(OnceCell::new());
            create_effect(move || {
                let state = state.get();
                let stars = self.stars.clone();
                let t_start = t_start.clone();
                frame.set(request_animation_frame(move |t| {
                    for star in stars.lock().unwrap().iter_mut() {
                        star.update_position();
                    }
                    let t_start = t_start.get_or_init(|| t);
                    for star in stars.lock().unwrap().iter() {
                        star.set(t - t_start, &state);
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
            circle(r#ref=self.node_ref, r=radius, fill=self.color, cx=x, cy=y) {
                animate(repeatCount="indefinite")
            }
        }
    }

    fn update_position(&mut self) {
        (self.x, self.y) = self.get_position();
    }

    fn set(&self, t: f64, state: &State) {
        let element = self.get_element();
        let element2_orig = element.first_child().unwrap();
        let element2 = element2_orig
            .clone_node_with_deep(false)
            .unwrap()
            .dyn_into::<Element>()
            .unwrap();
        let base_speed = self.speed();
        match state {
            State::Down => {
                element.set_attribute("cx", &self.x.to_string()).unwrap();
                element2.set_attribute("from", "0%").unwrap();
                element2.set_attribute("to", "100%").unwrap();
                element2.set_attribute("attributeName", "cy").unwrap();
                let dur = 0.001 * self.window_dims.height / base_speed;
                element2.set_attribute("dur", &dur.to_string()).unwrap();
                let mut start = -1.0 * self.y / base_speed;
                start = 0.001 * (start + t);
                element2.set_attribute("begin", &start.to_string()).unwrap();
            }
            State::Right => {
                element.set_attribute("cy", &self.y.to_string()).unwrap();
                element2.set_attribute("from", "0%").unwrap();
                element2.set_attribute("to", "100%").unwrap();
                element2.set_attribute("attributeName", "cx").unwrap();
                let speed = 10. * base_speed;
                let dur = 0.001 * self.window_dims.width / speed;
                element2.set_attribute("dur", &dur.to_string()).unwrap();
                let mut start = -1.0 * self.x / speed;
                start = 0.001 * (start + t);
                element2.set_attribute("begin", &start.to_string()).unwrap();
            }
            State::Left => {
                element.set_attribute("cy", &self.y.to_string()).unwrap();
                element2.set_attribute("from", "100%").unwrap();
                element2.set_attribute("to", "0%").unwrap();
                element2.set_attribute("attributeName", "cx").unwrap();
                let speed = 10. * base_speed;
                let dur = 0.001 * self.window_dims.width / speed;
                element2.set_attribute("dur", &dur.to_string()).unwrap();
                let mut start = -1.0 * (self.window_dims.width - self.x) / speed;
                start = 0.001 * (start + t);
                element2.set_attribute("begin", &start.to_string()).unwrap();
            }
        }
        element.replace_child(&element2, &element2_orig).unwrap();
    }

    fn get_element(&self) -> SvgCircleElement {
        self.node_ref.get().dyn_into().unwrap()
    }

    fn get_position(&self) -> (f64, f64) {
        let element = self.node_ref.get().dyn_into::<Element>().unwrap();
        let rect = element.get_bounding_client_rect();
        let x = 0.5 * (rect.left() + rect.right());
        let y = 0.5 * (rect.top() + rect.bottom());
        (x, y)
    }

    fn radius(&self) -> f64 {
        4.0 - self.depth * 2.5
    }

    fn speed(&self) -> f64 {
        0.08 - self.depth * 0.04
    }
}
