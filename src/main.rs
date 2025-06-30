use sycamore::prelude::*;

use site::organisms::{AboutCard, MetaCard, PythonCard, RustCard, YarnHoardCard};
use web_sys::{Element, wasm_bindgen::JsCast};

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|| {
        view! {
            Site()
        }
    });
}

#[component]
fn Site() -> View {
    view! {
        Carousel() {
            AboutCard()
            PythonCard()
            RustCard()
            YarnHoardCard()
            MetaCard()
        }
    }
}

#[component(inline_props)]
pub fn Carousel(#[prop(setter(into))] children: Children) -> View {
    let node_ref = create_node_ref();

    let scroll_progress = create_signal(0.);
    let update_scroll_progress = move |_| {
        scroll_progress.set(compute_scroll_progress(
            node_ref.get().dyn_into::<Element>().unwrap(),
        ));
    };

    create_effect(move || {
        console_log!("{}", scroll_progress.get());
    });

    view! {
        div(
            r#ref=node_ref,
            on:scroll=update_scroll_progress,
            class="max-h-screen overflow-scroll"
        ) {
            div(class="flex flex-col justify-center" ) {
                (children)
            }
        }
    }
}

fn compute_scroll_progress(element: Element) -> f64 {
    let scroll_top: f64 = element.scroll_top().into();
    let scroll_height: f64 = element.scroll_height().into();
    let client_height: f64 = element.client_height().into();
    scroll_top / (scroll_height - client_height)
}
