use rand::Rng;
use sycamore::prelude::*;

use site::{
    molecules::{Carousel, CarouselItem, CarouselItemPosition},
    organisms::{
        AboutCard, Header, JobCard, MetaCard, PythonCard, RustCard, SchoolCard, SkillsCard,
        YarnHoardCard,
    },
};

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|| {
        view! {
            div(class="h-screen w-screen flex justify-center items-center") {
                div(class="fixed") {
                    Stars()
                }
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
            }
        }
    });
}

#[component(inline_props)]
fn Stars() -> View {
    let mut rng = rand::rng();

    let height = create_signal(0);
    let width = create_signal(0);

    let update_window_size_callback = move || {
        let window = web_sys::window().unwrap();
        width.set(window.inner_width().unwrap().as_f64().unwrap() as i32);
        height.set(window.inner_height().unwrap().as_f64().unwrap() as i32);
    };
    let _listener = create_signal(gloo::events::EventListener::new(
        &web_sys::window().unwrap(),
        "resize",
        move |_event| update_window_size_callback(),
    ));

    let stars = create_signal(Vec::new());
    create_effect(move || {
        stars.set(
            (0..=400)
                .into_iter()
                .map(|_| {
                    let x = rng.random_range(0..=width.get());
                    let duration = rng.random_range(20.0..=40.0);
                    let delay = rng.random_range(0.0..=duration);
                    Star { x, duration, delay }
                })
                .collect::<Vec<Star>>(),
        );
    });

    on_mount(update_window_size_callback);

    view! {
        svg(height="100vh", width="100vw", xmlns="http://www.w3.org/2000/svg") {
            Indexed(
                list=stars,
                view=move |star| view! {
                    circle(r="2", cx=star.x.to_string(), cy="0", fill="white") {
                        animate(
                            attributeName="cy",
                            dur=star.duration.to_string(),
                            begin=format!("-{}s", star.delay),
                            from="0",
                            to=height.get().to_string(),
                            repeatCount="indefinite"
                        )
                    }
                }
            )
        }
    }
}

#[derive(PartialEq, Clone)]
struct Star {
    x: i32,
    duration: f32,
    delay: f32,
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
