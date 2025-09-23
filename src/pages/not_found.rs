use std::time::Duration;

use gloo::timers::callback::Timeout;
use sycamore::prelude::*;

use crate::atoms::{SlideInOut, SlideInOutState};
use crate::molecules::{FancyHandleText, Footer, Header};

#[component]
pub fn NotFound() -> View {
    let slide = create_signal(SlideInOutState::Right);
    on_mount(move || {
        Timeout::new(10, move || slide.set(SlideInOutState::OnScreen)).forget();
    });

    view! {
        div(class="w-screen h-screen flex flex-col") {
            SlideInOut(state=*slide) {
                Header(
                    return_callback=move || slide.set(SlideInOutState::Right),
                    return_delay_ms=400
                )
            }
            div(class="grow flex flex-col gap-y-8 justify-center items-center") {
                SlideInOut(state=*slide, delay=Duration::from_millis(50)) {
                    div(class="text-7xl md:text-9xl") {
                        FancyHandleText() { "404" }
                    }
                }
                SlideInOut(state=*slide, delay=Duration::from_millis(100)) {
                    div(class="text-4xl md:text-6xl") {
                        FancyHandleText() { "Page Not Found" }
                    }
                }
            }
            Footer()
        }
    }
}
