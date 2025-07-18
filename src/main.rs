use sycamore::prelude::*;

use site::{
    molecules::{Carousel, CarouselItem, CarouselItemPosition},
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
                Focus() {
                    AboutCard()
                }
            }
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
                    YarnHoardCard()
                }
            }
            CarouselItem() {
                Focus() {
                    MetaCard()
                }
            }
        }
        Footer()
    }
}

#[component(inline_props)]
fn Focus(#[prop(setter(into))] children: Children) -> View {
    let CarouselItemPosition(position) = use_context();

    let data_position = move || match position.get() {
        (i32::MIN..=-2) => "far-previous",
        -1 => "previous",
        0 => "center",
        1 => "next",
        (2..=i32::MAX) => "far-next",
    };

    view! {
        div(
            data-position=data_position,
            class="flex justify-center px-6 py-2"
        ) {
            div(
                data-position=data_position,
                //class="data-[position=center]:scale-100 scale-90 duration-500"
                class="w-95"
            ) {
                (children)
            }
        }
    }
}
