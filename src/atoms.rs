use sycamore::prelude::*;

#[component(inline_props)]
pub fn Collapse(
    #[prop(setter(into))] open: MaybeDyn<bool>,
    #[prop(setter(into))] children: Children,
) -> View {
    view! {
        div(
            data-state=move || if open.get() {"open"} else {"closed"},
            class="overflow-hidden transition-all duration-300 ease-in-out data-[state=closed]:max-h-0 data-[state=open]:max-h-screen"
        ) {
            (children)
        }
    }
}
