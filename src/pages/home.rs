use std::time::Duration;

use gloo::timers::callback::Timeout;
use sycamore::prelude::*;
use sycamore_router::navigate;

use crate::{
    atoms::{ExtrudedText, SlideInOut, SlideInOutState},
    molecules::{FancyHandleText, Footer},
    starscape::State,
};

#[component(inline_props)]
pub fn Home(state: Signal<State>) -> View {
    let slide = create_signal(SlideInOutState::Left);
    on_mount(move || {
        Timeout::new(10, move || slide.set(SlideInOutState::OnScreen)).forget();
        Timeout::new(300, move || state.set(State::Down)).forget();
    });

    view! {
        div(class="w-screen h-screen flex flex-col items-center") {
            div(class="grow flex flex-col justify-left items-center") {
                SlideInOut(state=*slide) {
                    div(class="mt-16 mb-16 md:mb-20 text-7xl md:text-9xl ") {
                        FancyHandleText() { "@maitreyakv" }
                    }
                }
                SlideInOut(state=*slide, delay=Duration::from_millis(50)) {
                    ProfilePhoto()
                }
                div(class="grow flex flex-col justify-center items-center gap-y-6 md:gap-y-8 text-5xl md:text-7xl") {
                    PageLink(
                        slide=slide,
                        state=state,
                        delay_ms=100,
                        url="/about",
                        color="var(--color-1)"
                    ) { "About" }
                    PageLink(
                        slide=slide,
                        state=state,
                        delay_ms=150,
                        url="/skills",
                        color="var(--color-2)"
                    ) { "Skills" }
                    PageLink(
                        slide=slide,
                        state=state,
                        delay_ms=200,
                        url="/career",
                        color="var(--color-4)"
                    ) { "Career" }
                    PageLink(
                        slide=slide,
                        state=state,
                        delay_ms=250,
                        url="/projects",
                        color="var(--color-5)"
                    ) { "Projects" }
                }
            }
            Footer()
        }
    }
}

#[component(inline_props)]
fn PageLink(
    slide: Signal<SlideInOutState>,
    state: Signal<State>,
    delay_ms: u64,
    url: &'static str,
    color: &'static str,
    #[prop(setter(into))] children: Children,
) -> View {
    let navigate_after_delay = move |_event| {
        slide.set(SlideInOutState::Left);
        state.set(State::Left);
        Timeout::new(550, || navigate(url)).forget();
    };
    view! {
        SlideInOut(state=*slide, delay=Duration::from_millis(delay_ms)) {
            div(class="hover:cursor-pointer", on:click=navigate_after_delay) {
                ExtrudedText(color=color) {
                    (children)
                }
            }
        }

    }
}

#[component]
fn ProfilePhoto() -> View {
    view! {
        div(class="flex justify-center items-center") {
            img(
                class="z-10 absolute w-48 md:w-64 rounded-full",
                src="assets/face.jpeg",
                alt="A picture of my face",
            )
            div(
                class="h-52 md:h-68 w-52 md:w-68 rounded-full animate-[spin_10s_linear_infinite]",
                style=r#"background: conic-gradient(
                    var(--color-1),
                    var(--color-2),
                    var(--color-3),
                    var(--color-4),
                    var(--color-5),
                    var(--color-4),
                    var(--color-3),
                    var(--color-2),
                    var(--color-1)
                );"#
            )
        }
    }
}
