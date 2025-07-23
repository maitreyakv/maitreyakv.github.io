use sycamore::prelude::*;

use site::{
    molecules::{Carousel, CarouselItem, CarouselItemPosition},
    organisms::{
        AboutCard, Footer, Header, JobCard, MetaCard, PythonCard, RustCard, SchoolCard, SkillsCard,
        YarnHoardCard,
    },
};

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|| {
        view! {
            div(class="h-screen w-screen flex justify-center items-center") {
                Header()
                Carousel() {
                    h1(class="text-center") { "Howdy!" }
                    CarouselItem() {
                        Focus() {
                            AboutCard()
                        }
                    }
                    h1(class="text-center") { "Skills" }
                    CarouselItem() {
                        Focus() {
                            PythonCard()
                        }
                    }
                    CarouselItem() {
                        Focus() {
                            RustCard()
                        }
                    }
                    CarouselItem() {
                        Focus() {
                            SkillsCard()
                        }
                    }
                    h1(class="text-center") { "Experience" }
                    CarouselItem() {
                        Focus() {
                            JobCard()
                        }
                    }
                    CarouselItem() {
                        Focus() {
                            SchoolCard()
                        }
                    }
                    h1(class="text-center") { "Personal Projects" }
                    CarouselItem() {
                        Focus() {
                            MetaCard()
                        }
                    }
                    CarouselItem() {
                        Focus() {
                            YarnHoardCard()
                        }
                    }
                }
                Footer()
            }
        }
    });
}

#[component(inline_props)]
fn Focus(#[prop(setter(into))] children: Children) -> View {
    let CarouselItemPosition(position) = use_context();

    view! {
        div(class="flex justify-center px-6 py-2") {
            div(
                data-focus=move || (position.get() == 0).to_string(),
                class="w-110"
            ) {
                (children)
            }
        }
    }
}
