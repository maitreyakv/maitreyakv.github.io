use sycamore::prelude::*;
use web_sys::{HtmlElement, wasm_bindgen::JsCast};

#[component(inline_props)]
pub fn Collapse(
    #[prop(setter(into))] open: MaybeDyn<bool>,
    #[prop(setter(into))] children: Children,
) -> View {
    let node_ref = create_node_ref();

    on_mount(move || {
        create_effect(move || {
            let element = node_ref.get().dyn_into::<HtmlElement>().unwrap();
            let scroll_height_with_px_suffix = format!("{}px", element.scroll_height());
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
