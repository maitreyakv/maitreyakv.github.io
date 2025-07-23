use sycamore::prelude::*;
use web_sys::{HtmlElement, wasm_bindgen::JsCast};

#[component(inline_props)]
pub fn Collapse(
    #[prop(setter(into))] open: MaybeDyn<bool>,
    #[prop(setter(into))] children: Children,
) -> View {
    let node_ref = create_node_ref();

    on_mount(move || {
        let element = node_ref.get().dyn_into::<HtmlElement>().unwrap();
        // TODO: This may not work with dynamically sized children, since we're computing this once
        let scroll_height_with_px_suffix = format!("{}px", element.scroll_height());

        create_effect(move || {
            element
                .style()
                .set_property(
                    "height",
                    if open.get() {
                        &scroll_height_with_px_suffix
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
