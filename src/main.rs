use sycamore::prelude::*;

use site::{
    molecules::{Carousel, CarouselItem},
    organisms::{AboutCard, Footer, Header, MetaCard, PythonCard, RustCard, YarnHoardCard},
};

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|| {
        view! {
            div(class="h-screen w-screen flex justify-center items-center") {
                Site()
            }
        }
    });
}

#[component]
fn Site() -> View {
    view! {
        Header()
        Carousel() {
            CarouselItem() {
                div(class="px-6 py-2") {
                    AboutCard()
                }
            }
            CarouselItem() {
                div(class="px-6 py-2") {
                    PythonCard()
                }
            }
            CarouselItem() {
                div(class="px-6 py-2") {
                    RustCard()
                }
            }
            CarouselItem() {
                div(class="px-6 py-2") {
                    YarnHoardCard()
                }
            }
            CarouselItem() {
                div(class="px-6 py-2") {
                    MetaCard()
                }
            }
        }
        Footer()
    }
}
