use std::time::Duration;

use gloo::timers::callback::{Interval, Timeout};
use sycamore::prelude::*;

use crate::{
    atoms::{ExtrudedText, Glass, SlideInOut, SlideInOutState},
    molecules::Header,
    pages::Page,
    starscape::State,
};

const PERSONAL_INFO_TEXT: &'static str = "let maitreya = Developer::new()
\t.first_name(\"Maitreya\")
\t.last_name(\"Venkataswamy\")
\t.pronouns(\"he\", \"him\", \"his\")
\t.location(\"Boston\")
\t.hobbies(vec![
\t\t\"Hiking\",
\t\t\"Videogames\",
\t\t\"Reading SciFi/Fantasy\",
\t\t\"Playing w/my dog\",
\t])
\t.build();\n";

#[component(inline_props)]
pub fn About(state: Signal<State>) -> View {
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
                        }
                    )
                }
            }
            div(class="grow flex flex-col gap-y-8 items-center") {
                div()
                SlideInOut(state=*slide, delay=Duration::from_millis(50)) {
                    Terminal()
                }
                SlideInOut(state=*slide, delay=Duration::from_millis(100)) {
                    AboutText()
                }
                div()
            }
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
                p() { "I have experience in data engineering, backend development, "
                      "and frontend development." }
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
fn Terminal() -> View {
    view! {
        div(class="w-100 md:w-130 h-112 md:h-132 flex flex-col") {
            div(class="w-full bg-gray-500 rounded-t-xl border-t-1 border-x-1 border-gray-600 p-2") {
                div(class="w-full flex justify-right gap-x-2 ") {
                    div(class="w-4 h-4 rounded-full bg-red-400")
                    div(class="w-4 h-4 rounded-full bg-yellow-400")
                    div(class="w-4 h-4 rounded-full bg-green-400")
                }
            }
            div(class="grow backdrop-blur-xs bg-[rgba(255,255,255,0.1)] rounded-b-xl border-b-1 border-x-1 border-gray-600 p-4") {
                TypingCodeBlock(text=PERSONAL_INFO_TEXT)
            }
        }
    }
}

#[component(inline_props)]
fn TypingCodeBlock(text: &'static str) -> View {
    let displayed_text = create_signal("");
    let index = create_signal(0);
    let is_cursor_visible = create_signal(true);

    let interval = Interval::new(50, move || {
        index.update(|i| {
            if *i < text.len() {
                *i += 1;
            }
        });
        displayed_text.set(&text[0..index.get()]);
    });
    on_cleanup(|| drop(interval));

    let interval2 = Interval::new(530, move || {
        if index.get() == text.len() {
            is_cursor_visible.update(|v| *v = !*v)
        }
    });
    on_cleanup(|| drop(interval2));

    view! {
        pre(class="font-roboto font-bold text-lg md:text-2xl") {
            code() {
                (displayed_text)
                (if is_cursor_visible.get() {"\u{2588}"} else {" "})
            }
        }
    }
}
