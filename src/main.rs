use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

use site::{
    components::starscape::Starscape,
    pages::{About, Career, Home, NotFound, Projects, Skills},
};

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|| App());
}

#[component]
fn App() -> View {
    let state = create_signal(site::components::starscape::State::Down);
    view! {
        div(class="-z-1") {
            Starscape(state=*state)
        }
        Router(
            integration=HistoryIntegration::new(),
            view=move |route| view! {
                (match route.get_clone() {
                    AppRoutes::Home => view! { Home(state=state) },
                    AppRoutes::Resume => view! { Home(state=state) },
                    AppRoutes::About => view! { About(state=state) },
                    AppRoutes::Skills => view! { Skills(state=state) },
                    AppRoutes::Career => view! { Career(state=state) },
                    AppRoutes::Projects => view! { Projects(state=state) } ,
                    AppRoutes::NotFound => view! { NotFound(state=state) },
                })
            }
        )
    }
}

#[derive(Route, Clone)]
enum AppRoutes {
    #[to("/")]
    Home,
    #[to("/maitreyakv-resume.pdf")]
    Resume,
    #[to("/about")]
    About,
    #[to("/skills")]
    Skills,
    #[to("/career")]
    Career,
    #[to("/projects")]
    Projects,
    #[not_found]
    NotFound,
}
