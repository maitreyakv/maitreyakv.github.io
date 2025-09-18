use sycamore::prelude::*;

#[component(inline_props)]
pub fn Card(#[prop(setter(into))] children: Children) -> View {
    view! {
        div(class="flex flex-col justify-center gap-y-2 backdrop-blur-[2px]") {
            (children)
        }
    }
}

#[component(inline_props)]
pub fn CardTitle(#[prop(setter(into))] children: Children) -> View {
    view! {
        div(class="flex justify-center") {
            (children)
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
