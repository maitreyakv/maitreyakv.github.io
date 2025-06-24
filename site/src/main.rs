use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::window;

fn main() {
    console_error_panic_hook::set_once();

    let window = window().unwrap();

    let closure = Closure::wrap(Box::new(update_page) as Box<dyn Fn()>);
    let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
    let _ = window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
    closure.forget();

    // Trigger an update immediately to stage the element positions initially
    update_page()
}

fn update_page() {
    update_sliding_section("section__about", SliderDirection::Left);
    update_sliding_section("section__python", SliderDirection::Right);
    update_sliding_section("section__rust", SliderDirection::Left);
    update_sliding_section("section__yarn-hoard", SliderDirection::Right);
}

fn update_sliding_section(id: &str, direction: SliderDirection) {
    let window = window().unwrap();
    let viewport_height = f64::try_from(window.inner_height().unwrap()).unwrap();
    let element = window.document().unwrap().get_element_by_id(id).unwrap();
    let rect = element.get_bounding_client_rect();
    let class_to_hide = match direction {
        SliderDirection::Left => "-translate-x-2/3",
        SliderDirection::Right => "translate-x-2/3",
    };
    let hidden = rect.bottom() > viewport_height;
    element
        .class_list()
        .toggle_with_force(class_to_hide, hidden)
        .unwrap();
}

#[derive(Clone)]
enum SliderDirection {
    Left,
    Right,
}
