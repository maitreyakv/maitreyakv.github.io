use std::fmt::Display;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::Event;
use web_sys::{Element, window};

fn main() {
    console_error_panic_hook::set_once();

    let window = window().unwrap();

    create_slider_callback(
        window
            .document()
            .unwrap()
            .get_element_by_id("slider__about")
            .unwrap(),
        SliderDirection::Left,
    );
    create_slider_callback(
        window
            .document()
            .unwrap()
            .get_element_by_id("slider__python")
            .unwrap(),
        SliderDirection::Right,
    );
    create_slider_callback(
        window
            .document()
            .unwrap()
            .get_element_by_id("slider__rust")
            .unwrap(),
        SliderDirection::Left,
    );
    create_slider_callback(
        window
            .document()
            .unwrap()
            .get_element_by_id("slider__yarn-hoard")
            .unwrap(),
        SliderDirection::Right,
    );

    // Trigger a scroll event manually at the start to activate the first few sliders if they could
    // trigger already.
    window
        .dispatch_event(&Event::new("scroll").unwrap())
        .unwrap();
}

fn create_slider_callback(element: Element, direction: SliderDirection) {
    let window = window().unwrap();
    let closure = {
        let window = window.clone();
        Closure::wrap(Box::new(move || {
            let half_viewport_height = f64::try_from(window.inner_height().unwrap()).unwrap() / 2.;
            let rect = element.get_bounding_client_rect();
            let out_class = format!("slider slider-out-{}", direction);
            element.set_class_name(if rect.y() < half_viewport_height {
                "slider slider-in"
            } else {
                &out_class
            })
        }) as Box<dyn Fn()>)
    };
    let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
    closure.forget();
}

#[derive(Clone)]
enum SliderDirection {
    Left,
    Right,
}

impl Display for SliderDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Left => "left",
                Self::Right => "right",
            }
        )
    }
}
