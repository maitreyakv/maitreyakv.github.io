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
    view! {
        div(class=r#"max-h-full h-full overflow-y-scroll flex flex-col
                     scroll-py-8 snap-y snap-mandatory before:basis-1/2 
                     before:shrink-0 after:basis-1/2 after:shrink-0"#) {
            (children)
        }
    }
}

#[component(inline_props)]
pub fn CarouselItem(#[prop(setter(into))] children: Children) -> View {
    let position = create_signal(-1);
    provide_context_in_new_scope(CarouselItemPosition(position), || {
        view! {
            div(class="snap-center") {
                (children)
            }
        }
    })
}

#[derive(Clone, Copy)]
pub struct CarouselItemPosition(pub Signal<i32>);
