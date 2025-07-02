use sycamore::prelude::*;

#[component(inline_props)]
pub fn Collapse(
    #[prop(setter(into))] open: MaybeDyn<bool>,
    #[prop(setter(into))] children: Children,
) -> View {
    view! {
        div(
            data-state=move || if open.get() {"open"} else {"closed"},
            //class="overflow-hidden _transition-all duration-100 data-[state=closed]:max-h-0 data-[state=open]:max-h-screen"
        ) {
            (children)
        }
    }
}
