use std::time::Duration;

use sycamore::prelude::*;
use web_sys::{HtmlElement, wasm_bindgen::JsCast};

#[component(inline_props)]
pub fn SlideInOut(
    state: ReadSignal<SlideInOutState>,
    delay: Option<Duration>,
    #[prop(setter(into))] children: Children,
) -> View {
    let delay_ms = delay.unwrap_or(Duration::default()).as_millis();
    let node_ref = create_node_ref();
    on_mount(move || {
        let element = node_ref.get().dyn_into::<HtmlElement>().unwrap();
        create_effect(move || {
            let style = format!(
                "transition: transform 0.5s ease-in-out; transition-delay: {}ms; transform: {}",
                delay_ms,
                state.get().to_transform_value()
            );
            element.style().set_css_text(&style);
        });
    });
    view! {
        div(r#ref=node_ref) {
            (children)
        }
    }
}

#[derive(Copy, Clone)]
pub enum SlideInOutState {
    OnScreen,
    Up,
    Down,
    Left,
    Right,
}
impl SlideInOutState {
    fn to_transform_value(&self) -> String {
        match self {
            Self::OnScreen => "translateX(0)",
            Self::Up => "translateY(-100vh)",
            Self::Down => "translateY(100vh)",
            Self::Left => "translateX(-100vw)",
            Self::Right => "translateX(100vw)",
        }
        .to_string()
    }
}

#[component(inline_props)]
pub fn ExtrudedText(
    #[prop(setter(into))] children: Children,
    color: &'static str,
    depth: Option<f32>,
    resolution: Option<f32>,
) -> View {
    let depth = depth.unwrap_or(0.15);
    let resolution = resolution.unwrap_or(0.001);
    let n_layers = (depth / resolution) as i32;

    let text_shadow = (0..=n_layers)
        .into_iter()
        .map(|i| {
            format!(
                "{}em {}em {color}",
                resolution * i as f32,
                resolution * i as f32
            )
        })
        .reduce(|left, right| format!("{left}, {right}"))
        .unwrap();

    view! {
        div(style=format!("text-shadow: {text_shadow};")) {
            (children)
        }
    }
}

#[component(inline_props)]
pub fn Collapse(
    #[prop(setter(into))] open: MaybeDyn<bool>,
    #[prop(setter(into))] children: Children,
) -> View {
    let node_ref = create_node_ref();

    let mut height = 0i32;
    on_mount(move || {
        let element = node_ref.get().dyn_into::<HtmlElement>().unwrap();
        create_effect(move || {
            height = {
                let scroll_height = element.scroll_height();
                if (scroll_height - height).abs() > 1 {
                    scroll_height
                } else {
                    height
                }
            };

            let height_with_px_suffix = format!("{height}px");
            element
                .style()
                .set_property(
                    "height",
                    if open.get() {
                        &height_with_px_suffix
                    } else {
                        "0"
                    },
                )
                .unwrap();
        });
    });

    view! {
        div(
            r#ref=node_ref,
            class="overflow-hidden transition-all delay-600 duration-500 ease-in-out"
        ) {
            (children)
        }
    }
}
