use derive_builder::Builder;
use gloo::events::EventListener;
use rand::{Rng, seq::IndexedRandom};
use sycamore::prelude::*;
use web_sys::{SvgCircleElement, wasm_bindgen::JsCast};

use crate::utils::{WindowDims, request_animation_frame_loop};

const STAR_COLORS: [&str; 5] = ["#a4c2ff", "#cadaff", "#fff6ed", "#ffcea6", "#ffb16e"];
const STAR_DENSITY: f64 = 0.00075; // stars per square pixel

#[component(inline_props)]
pub fn Starscape() -> View {
    let window_dims = create_signal(WindowDims::now());
    let _resize_listener_handle =
        create_signal(EventListener::new(&window(), "resize", move |_event| {
            window_dims.set(WindowDims::now());
        }));

    view! {
        div(class="fixed bg-black -z-999") {
            svg(height="100vh", width="100vw", xmlns="http://www.w3.org/2000/svg") {
                (move || StarController::new(window_dims.get()).view())
            }
        }
    }
}

#[derive(Clone)]
struct StarController {
    stars: Vec<Star>,
}
impl StarController {
    fn new(window_dims: WindowDims) -> Self {
        let mut rng = rand::rng();
        let n_stars = (window_dims.area() * STAR_DENSITY) as i64;
        let stars = (0..n_stars)
            .map(move |_| {
                StarBuilder::default()
                    .x_initial(rng.random_range(0.0..=window_dims.width))
                    .y_initial(rng.random_range(0.0..=window_dims.height))
                    .depth(rng.random_range(0.0..=1.0))
                    .color(STAR_COLORS.choose(&mut rng).unwrap())
                    .node_ref(create_node_ref())
                    .window_dims(window_dims)
                    .build()
                    .unwrap()
            })
            .collect::<Vec<Star>>();
        Self { stars }
    }

    fn animate(&self, t: f64) {
        for star in &self.stars {
            star.animate(t);
        }
    }

    fn view(self) -> View {
        let views = self
            .stars
            .iter()
            .map(|star| star.view())
            .collect::<Vec<View>>();

        // TEMP: Using SVG animations instead of manual animation
        //request_animation_frame_loop(move |t| {
        //    self.animate(t);
        //});

        view! {
            (views)
        }
    }
}

#[derive(Builder, Clone)]
struct Star {
    x_initial: f64,
    y_initial: f64,
    depth: f64,
    color: &'static str,
    node_ref: NodeRef,
    window_dims: WindowDims,
}
impl Star {
    fn view(&self) -> View {
        let radius = self.radius().to_string();
        let x = self.x_initial.to_string();
        let y = self.y_initial.to_string();
        let window_height = self.window_dims.height;
        let speed = self.speed();
        let duration = (0.001 * window_height / speed).to_string();
        let start = (-0.001 * (self.y_initial / speed)).to_string();
        view! {
            circle(r#ref=self.node_ref, r=radius, fill=self.color, cx=x, cy=y) {
                // TEMP: Using SVG animations instead of manual animation
                animate(
                    attributeName="cy",
                    from="0",
                    to=window_height.to_string(),
                    dur=duration,
                    repeatCount="indefinite",
                    begin=format!("{start}s"),
                )
            }
        }
    }

    fn animate(&self, t: f64) {
        let element = self.node_ref.get().dyn_into::<SvgCircleElement>().unwrap();
        let y = (self.y_initial + self.speed() * t) % self.window_dims.height;
        element.set_attribute("cy", &y.to_string()).unwrap();
    }

    fn radius(&self) -> f64 {
        4.0 - self.depth * 2.5
    }

    fn speed(&self) -> f64 {
        0.08 - self.depth * 0.04
    }
}
