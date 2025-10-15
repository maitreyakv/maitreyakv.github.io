use std::time::Duration;

use gloo::timers::callback::Timeout;
use sycamore::prelude::*;

use crate::{
    components::atoms::{ExtrudedText, SlideInOut, SlideInOutState},
    components::molecules::Header,
    components::starscape::State,
    pages::Page,
};

#[component(inline_props)]
pub fn Projects(state: Signal<State>) -> View {
    let slide = create_signal(SlideInOutState::Right);
    on_mount(move || {
        Timeout::new(10, move || slide.set(SlideInOutState::OnScreen)).forget();
        Timeout::new(300, move || state.set(State::Down)).forget();
    });

    view! {
        Page() {
            div(class="w-full z-1 sticky top-0") {
                SlideInOut(state=*slide) {
                    Header(
                        return_delay_ms=400,
                        return_callback=move || {
                            slide.set(SlideInOutState::Right);
                            state.set(State::Right);
                        }
                    )
                }
            }
            div(class="grow flex flex-col gap-y-8 items-center py-4") {
                SlideInOut(state=*slide, delay=Duration::from_millis(50)) {
                    div(class="text-3xl md:text-5xl mb-4 md:mb-6") {
                        ExtrudedText(color="#0092b8") { "Source code for this site" }
                    }
                    MetaText()
                }
                SlideInOut(state=*slide, delay=Duration::from_millis(100)) {
                    div(class="text-3xl md:text-5xl mb-4 md:mb-6") {
                        ExtrudedText(color="#7f22fe") { "YarnHoard, track your stash" }
                    }
                    YarnHoardText()
                }
            }
        }
    }
}

#[component]
fn MetaText() -> View {
    view! {
        div(class="glass glass-border p-6 flex flex-col gap-y-4 text-md md:text-xl") {
            p() {
                "Want to see how this site works? Check out "
                a(href="https://github.com/maitreyakv/maitreyakv") { "the code on GitHub" }
                ". Its made with Rust and Tailwind!"
            }
            p() {
                "The styling is done with "
                a(href="https://tailwindcss.com") { "Tailwind CSS" }
                " and the interactivity is implemented with the delightfully simple "
                a(href="https://sycamore.dev") { "Sycamore" }
                " framework in Rust, which is compiled to "
                a(href="https://webassembly.org") { "WebAssembly" }
                "."
            }
            p() {
                "If you find a bug with this site, please don't hesitate to "
                a(href="https://github.com/maitreyakv/maitreyakv.github.io/issues") { "open an issue" }
                " in the repository."
            }
        }
    }
}

#[component]
fn YarnHoardText() -> View {
    view! {
        div(class="glass glass-border p-6 flex flex-col gap-y-4 text-md md:text-xl") {
            p() { "A full stack app for tracking your yarn inventory, for crafty people. The entire app "
                  "is written in Rust!" }
            h2(class="text-center") { "Work in progress, coming soon!" }
            p() { "Its a pretty straightforward CRUD app with email and password login, its main functionality "
                  "being allowing the user to create and manage records for their yarn inventory and record "
                  "information about their collection." }
        }
    }
}
