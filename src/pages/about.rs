use std::time::Duration;

use gloo::timers::callback::Timeout;
use sycamore::prelude::*;

use crate::{
    atoms::{ExtrudedText, Glass, SlideInOut, SlideInOutState},
    molecules::{Footer, Header},
};

#[component]
pub fn About() -> View {
    let slide = create_signal(SlideInOutState::Right);
    on_mount(move || {
        Timeout::new(10, move || slide.set(SlideInOutState::OnScreen)).forget();
    });

    view! {
        div(class="w-screen h-screen flex flex-col items-center") {
            SlideInOut(state=*slide) {
                Header(
                    return_delay_ms=400,
                    return_callback=move || slide.set(SlideInOutState::Right)
                )
            }
            div(class="grow flex flex-col justify-evenly items-center") {
                SlideInOut(state=*slide, delay=Duration::from_millis(50)) {
                    Glass() {
                        div(class="p-6") {
                            PersonalInfo()
                        }
                    }
                }
                SlideInOut(state=*slide, delay=Duration::from_millis(100)) {
                    AboutText()
                }
            }
            Footer()
        }
    }
}

#[component]
fn AboutText() -> View {
    view! {
        Glass() {
            div(class=r#"p-6 flex flex-col gap-y-4 max-w-100 md:max-w-180 text-md md:text-xl"#) {
                p() { "I'm pursuing a career in software development, but I come from a "
                      "scientific/engineering background and I love working on problems in "
                      "those domains."}
                p() { "I'm more experienced in data engineering and backend development, "
                      "but I also dabble in frontend and am looking for opportunities to "
                      "grow my skills there." }
                p() {
                    "In my free time I enjoy hiking, playing videogames, reading science "
                    "fiction and fantasy, and playing with my dog "
                    a(href="https://www.instagram.com/bumblebee.the.bully") { "Bumblebee" }
                    "."
                }
            }
        }

    }
}

#[component]
fn PersonalInfo() -> View {
    view! {
        pre(class="font-roboto text-xl md:text-3xl") {
            code() {
                ExtrudedText(color="var(--color-1)") {
r#"let maitreya = Developer::new()
    .first_name("Maitreya")
    .last_name("Venkataswamy")
    .pronouns("he", "him", "his")
    .location("Boston")
    .hobbies(vec![
        "Hiking",
        "Videogames",
        "Reading SciFi/Fantasy",
        "Playing w/my dog",
    ])
    .build();"#
                }
            }
        }
    }
}
