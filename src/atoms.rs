use sycamore::prelude::*;
use web_sys::{HtmlElement, wasm_bindgen::JsCast};

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
