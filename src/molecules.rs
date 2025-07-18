use sycamore::prelude::*;
use web_sys::{Element, Node, wasm_bindgen::JsCast};

#[component(inline_props)]
pub fn Card(id: &'static str, #[prop(setter(into))] children: Children) -> View {
    view! {
        div(id=id, class="border-2 border-black rounded-2xl shadow-[8px_8px_0px_rgba(0,0,0,1)]") {
            div(class="p-4 flex flex-col justify-center gap-y-2") {
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

    let nodes_and_relative_positions = create_signal(Vec::new());

    let callback = move |_| {
        let node_ref = node_ref.get();
        let carousel = node_ref.dyn_ref::<Element>().unwrap();
        let carousel_rect = carousel.get_bounding_client_rect();
        let y_mid = (carousel_rect.top() + carousel_rect.bottom()) / 2.;

        let items: Vec<Node> = node_ref
            .child_nodes()
            .values()
            .into_iter()
            .map(|v| v.unwrap().into())
            .collect();

        let center_item_idx = items
            .iter()
            .enumerate()
            .min_by_key(|&(_, element)| {
                let element_rect = element
                    .dyn_ref::<Element>()
                    .unwrap()
                    .get_bounding_client_rect();
                let y_mid_element = (element_rect.top() + element_rect.bottom()) / 2.;

                (y_mid - y_mid_element).abs() as u32
            })
            .map(|(idx, _)| idx)
            .unwrap_or(0) as i32;

        nodes_and_relative_positions.set(
            items
                .into_iter()
                .enumerate()
                .map(|(idx, node)| (node, idx as i32 - center_item_idx))
                .collect(),
        )
    };

    provide_context_in_new_scope(
        CarouselState {
            nodes_and_relative_positions,
        },
        || {
            view! {
                div(
                    r#ref=node_ref,
                    on:scroll=callback,
                    class=r#"max-h-full h-full overflow-y-scroll flex flex-col
                             scroll-py-8 snap-y snap-mandatory before:basis-[calc(50vh)]
                             before:shrink-0 after:basis-[calc(50vh)] after:shrink-0"#
                ) {
                    (children)
                }
            }
        },
    )
}

#[derive(Clone)]
pub struct CarouselState {
    nodes_and_relative_positions: Signal<Vec<(Node, i32)>>,
}

#[component(inline_props)]
pub fn CarouselItem(#[prop(setter(into))] children: Children) -> View {
    let node_ref = create_node_ref();

    let position = create_signal(-1);

    let CarouselState {
        nodes_and_relative_positions,
    } = use_context();

    create_effect(move || {
        position.set(
            nodes_and_relative_positions
                .get_clone()
                .iter()
                .find(|(node, _)| *node == node_ref.get())
                .map(|(_, relative_index)| relative_index.to_owned())
                .unwrap_or(-1),
        );
    });

    provide_context_in_new_scope(CarouselItemPosition(position), || {
        view! {
            div(r#ref=node_ref, class="snap-center") {
                (children)
            }
        }
    })
}

#[derive(Clone, Copy)]
pub struct CarouselItemPosition(pub Signal<i32>);
