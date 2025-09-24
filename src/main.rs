use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

use site::{
    pages::{About, Career, Home, NotFound, Projects, Skills},
    starscape::Starscape,
};

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|| App());
}

#[component]
fn App() -> View {
    view! {
        Starscape()
        Router(
            integration=HistoryIntegration::new(),
            view=|route| view! {
                (match route.get_clone() {
                    AppRoutes::Home => Home(),
                    AppRoutes::Resume => Home(),
                    AppRoutes::About => About(),
                    AppRoutes::Skills => Skills(),
                    AppRoutes::Career => Career(),
                    AppRoutes::Projects => Projects(),
                    AppRoutes::NotFound => NotFound(),
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
