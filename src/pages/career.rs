use std::time::Duration;

use gloo::timers::callback::Timeout;
use sycamore::prelude::*;

use crate::{
    atoms::{ExtrudedText, Glass, SlideInOut, SlideInOutState},
    molecules::{Footer, Header},
    starscape::State,
};

#[component(inline_props)]
pub fn Career(state: Signal<State>) -> View {
    let slide = create_signal(SlideInOutState::Right);
    on_mount(move || {
        Timeout::new(10, move || slide.set(SlideInOutState::OnScreen)).forget();
        Timeout::new(300, move || state.set(State::Down)).forget();
    });

    view! {
        div(class="w-screen h-screen flex flex-col items-center") {
            SlideInOut(state=*slide) {
                Header(
                    return_delay_ms=400,
                    return_callback=move || {
                        slide.set(SlideInOutState::Right);
                        state.set(State::Right);
                    }
                )
            }
            div(class="grow flex flex-col gap-y-8 items-center") {
                SlideInOut(state=*slide, delay=Duration::from_millis(50)) {
                    div(class="text-3xl md:text-5xl mb-4 md:mb-6") {
                        ExtrudedText(color="#d08700") { "Path to Software Engineer" }
                    }
                    PathText()
                }
                SlideInOut(state=*slide, delay=Duration::from_millis(100)) {
                    div(class="text-3xl md:text-5xl mb-4 md:mb-6") {
                        ExtrudedText(color="#ec003f") { "What I studied in school" }
                    }
                    SchoolText()
                }
            }
            Footer()
        }
    }
}

#[component]
fn PathText() -> View {
    view! {
        Glass() {
            div(class=r#"p-6 flex flex-col gap-y-4 max-w-100 md:max-w-180 text-md md:text-xl"#) {
                p() { "I've made the transition from a data scientist, to a data engineer, and finally now to a software engineer." }
                h2() {
                    a(href="https://www.titanaes.com") { "Titan Advanced Energy Solutions" }
                }
                p() { "I first worked as a Data Scientist at Titan, where I developed machine learning models to "
                      "inspect and monitor lithium-ion batteries using ultrasound technology." }
                p() { "I engineered a lot of the software (Python) and cloud infrastructure (AWS) to support the required data "
                      "collection experiments and signal processing techniques." }
                h2() {
                    a(href="https://www.dayzerodiagnostics.com") { "Day Zero Diagnostics" }
                    " (now "
                    a(href="https://www.biomerieux.com/corp/en/journalists/press-releases/NGS-sequencing-acquisition-day-zero-diagnostics.html") {
                        "bioMérieux"
                    }
                    ")"
                }
                p() { "I currently work for Day Zero as a Data/Software Engineer where I began with engineering the curation "
                      "of MicrohmDB, our in-house database of 90k+ bacterial genomes with corresponding AMR tests." }
                p() { "More recently, I've contributed to development of Keynome, our cloud platform (Python on GCP) for bacterial species ID "
                      "and AMR profiling from genomic data." }
                p() { "I also lead development of our platforms's CLI client (Rust), and  participate in development "
                      "of the web client (React)." }
            }
        }
    }
}

#[component]
fn SchoolText() -> View {
    view! {
        Glass() {
            div(class=r#"p-6 flex flex-col gap-y-4 max-w-100 md:max-w-180 text-md md:text-xl"#) {
                p() { "My engineering and data science backgrounds have been helpful for building software "
                      "in various domains."}
                p() { "I have a B.S. in Aerospace Engineering and a minor in computer science from the Georgia Institute of Technology." }
                p() { "While I was there I also did four years of undergraduate research part-time at the Computational Combustion Laboratory." }
                p() { "I have a M.S. in Data Science from Brown University as well." }
            }
        }
    }
}
