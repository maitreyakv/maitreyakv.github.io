mod about;
mod career;
mod home;
mod not_found;
mod projects;
mod skills;

pub use about::About;
pub use career::Career;
pub use home::Home;
pub use not_found::NotFound;
pub use projects::Projects;
pub use skills::Skills;

use sycamore::prelude::*;

use crate::molecules::Footer;

#[component(inline_props)]
fn Page(#[prop(setter(into))] children: Children) -> View {
    view! {
        div(class="w-screen h-screen flex flex-col items-center") {
            // TOOD: Lift Header up from pages (in children) to here
            (children)
            div(class="sticky bottom-0") {
                Footer()
            }
        }
    }
}
