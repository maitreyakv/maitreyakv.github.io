use sycamore::prelude::*;
use web_sys::{Element, Node, wasm_bindgen::JsCast};

#[component(inline_props)]
pub fn Card(id: &'static str, #[prop(setter(into))] children: Children) -> View {
    view! {
        div(id=id, class="border-2 border-black rounded-2xl shadow-[8px_8px_0px_rgba(0,0,0,1)]") {
            div(class="p-4 flex flex-col gap-y-2") {
                (children)
            }
        }
    }
}

#[component(inline_props)]
pub fn CardHeader(#[prop(setter(into))] children: Children) -> View {
    view! {
        div(class="grid grid-cols-3 gap-x-4") {
            (children)
        }

    }
}

#[component(inline_props)]
pub fn CardSummary(#[prop(setter(into))] children: Children) -> View {
    view! {
        div(class="col-span-2 flex flex-col justify-around gap-y-1") {
            (children)
        }
    }
}

#[component(inline_props)]
pub fn CardImage(#[prop(setter(into))] children: Children) -> View {
    view! {
        div(class="col-span-1 flex justify-center items-center") {
            (children)
        }
    }
}

#[component(inline_props)]
pub fn CardContent(#[prop(setter(into))] children: Children) -> View {
    view! {
        div(class="flex flex-col gap-y-2") {
            (children)
        }
    }
}

#[component(inline_props)]
pub fn Carousel(#[prop(setter(into))] children: Children) -> View {
    let node_ref = create_node_ref();

    let scroll_progress = create_signal(0.);

    let scroll_callback = move |_| {
        scroll_progress.set(compute_scroll_progress(
            node_ref.get().dyn_ref::<Element>().unwrap(),
        ));
    };

    provide_context_in_new_scope(CarouselScrollProgress(scroll_progress), || {
        view! {
            div(r#ref=node_ref, on:scroll=scroll_callback, class="max-h-screen overflow-scroll") {
                (children)
            }
        }
    })
}

#[component(inline_props)]
pub fn CarouselItemList(#[prop(setter(into))] children: Children) -> View {
    let node_ref = create_node_ref();

    let CarouselScrollProgress(scroll_progress) = use_context();

    let focused_node = create_signal(None);

    on_mount(move || {
        create_effect(move || {
            let child_nodes = node_ref
                .get()
                .child_nodes()
                .values()
                .into_iter()
                .map(|v| Node::from(v.unwrap()))
                .collect::<Vec<_>>();
            let idx = (scroll_progress.get() * child_nodes.len() as f64) as usize;
            let idx = std::cmp::min(idx, child_nodes.len() - 1);
            focused_node.set(Some(child_nodes[idx].to_owned()));
        });
    });

    provide_context_in_new_scope(CarouselFocusedNode(focused_node), || {
        view! {
            div(r#ref=node_ref, class="flex flex-col justify-center") {
                (children)
            }
        }
    })
}

#[component(inline_props)]
pub fn CarouselItem(#[prop(setter(into))] children: Children) -> View {
    let node_ref = create_node_ref();

    let CarouselFocusedNode(focused_node) = use_context();

    let item_is_focused = create_signal(false);
    on_mount(move || {
        create_effect(move || {
            item_is_focused.set(focused_node.get_clone() == Some(node_ref.get()));
        });
    });

    provide_context_in_new_scope(CarouselItemIsFocused(item_is_focused), || {
        view! {
            div(r#ref=node_ref) {
                (children)
            }
        }
    })
}

#[derive(Clone, Copy)]
struct CarouselScrollProgress(Signal<f64>);

#[derive(Clone, Copy)]
struct CarouselFocusedNode(Signal<Option<Node>>);

#[derive(Clone, Copy)]
pub struct CarouselItemIsFocused(pub Signal<bool>);

fn compute_scroll_progress(element: &Element) -> f64 {
    let scroll_top: f64 = element.scroll_top().into();
    let scroll_height: f64 = element.scroll_height().into();
    let client_height: f64 = element.client_height().into();
    scroll_top / (scroll_height - client_height)
}
