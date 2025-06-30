use sycamore::prelude::*;

use site::{
    molecules::{Carousel, CarouselItem, CarouselItemList},
    organisms::{AboutCard, MetaCard, PythonCard, RustCard, YarnHoardCard},
};

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|| {
        view! {
            Site()
        }
    });
}

#[component]
fn Site() -> View {
    view! {
        Carousel() {
            div(class="h-[1100px]") {
                CarouselItemList() {
                    CarouselItem() {
                        div(class="p-6") {
                            AboutCard()
                        }
                    }
                    CarouselItem() {
                        div(class="p-6") {
                            PythonCard()
                        }
                    }
                    CarouselItem() {
                        div(class="p-6") {
                            RustCard()
                        }
                    }
                    CarouselItem() {
                        div(class="p-6") {
                            YarnHoardCard()
                        }
                    }
                    CarouselItem() {
                        div(class="p-6") {
                            MetaCard()
                        }
                    }
                }
            }
        }
    }
}
