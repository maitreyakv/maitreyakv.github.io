use std::time::Duration;

use gloo::timers::callback::Timeout;
use sycamore::prelude::*;

use crate::atoms::{SlideInOut, SlideInOutState};
use crate::molecules::{FancyHandleText, Header};
use crate::pages::Page;
use crate::starscape::State;

#[component(inline_props)]
pub fn NotFound(state: Signal<State>) -> View {
    let slide = create_signal(SlideInOutState::Right);
    on_mount(move || {
        Timeout::new(10, move || slide.set(SlideInOutState::OnScreen)).forget();
        Timeout::new(300, move || state.set(State::Down)).forget();
    });

    view! {
        Page() {
            div(class="z-1 sticky top-0") {
                SlideInOut(state=*slide) {
                    Header(
                        return_delay_ms=400,
                        return_callback=move || {
                            slide.set(SlideInOutState::Right);
                            state.set(State::Right);
                        },
                    )
                }
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
        }
    }
}
