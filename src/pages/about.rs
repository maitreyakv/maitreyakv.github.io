use std::{rc::Rc, time::Duration};

use gloo::timers::callback::{Interval, Timeout};
use sycamore::prelude::*;

use crate::{
    atoms::{ExtrudedText, SlideInOut, SlideInOutState},
    include_html,
    molecules::Header,
    pages::Page,
    starscape::State,
    typer::{Typer, TyperStep},
};

const PERSONAL_INFO_TEXT: &'static str = "$ developer new \\
\t--first-name \"Maitreya\" \\
\t--last-name \"Venkataswamy\" \\
\t--pronouns \"he/him/his\" \\
\t--location \"Boston\" \\
\t--hobby \"Hiking\" \\
\t--hobby \"Videogames\" \\
\t--hobby \"Reading SciFi/Fantasy\" \\
\t--hobby \"Playing w/my dog\"
$ ";

#[component(inline_props)]
pub fn About(state: Signal<State>) -> View {
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
            div(class="w-full grow flex flex-col gap-y-8 items-center") {
                div()
                SlideInOut(state=*slide, delay=Duration::from_millis(50)) {
                    Rotator()
                }
                SlideInOut(state=*slide, delay=Duration::from_millis(100)) {
                    div(class="flex flex-col md:flex-row md:flex-row-reverse gap-y-8 gap-x-8") {
                        div(class="shrink-0") {
                            Terminal()
                        }
                        (include_html!("about_text.html"))
                    }
                }
                div()
            }
        }
    }
}

const TEXTS: [&'static str; 4] = [
    "Backend Development",
    "Data Engineering",
    "Frontend Development",
    "Data Science",
];

const COLORS: [&'static str; 4] = [
    "var(--color-1)",
    "var(--color-2)",
    "var(--color-4)",
    "var(--color-5)",
];

#[component]
fn Rotator() -> View {
    let idx = create_signal(0);
    let interval = Interval::new(2000, move || {
        idx.update(|i| *i = (*i + 1) % TEXTS.len());
    });
    on_cleanup(|| drop(interval));

    let views = (0..TEXTS.len())
        .into_iter()
        .map(|i| {
            let pos = create_memo(move || {
                if idx.get() == i {
                    "current"
                } else if (idx.get() + 1).rem_euclid(TEXTS.len()) == i {
                    "next"
                } else {
                    "previous"
                }
            });

            view! {
                div(
                    data-pos=pos,
                    class=r#"absolute transition duration-500 opacity-0
                             data-[pos=next]:translate-y-[100%]
                             data-[pos=previous]:-translate-y-[100%]
                             data-[pos=current]:opacity-100"#
                ) {
                    ExtrudedText(color=COLORS[i]) { (TEXTS[i]) }
                }
            }
        })
        .collect::<Vec<View>>();

    view! {
        div(class="flex flex-wrap gap-x-2 text-4xl md:text-5xl font-bold") {
            "I have experience in"
            div(class="h-10 md:h-15 w-100 md:w-130") {
                (views)
            }
        }
    }
}

#[component]
fn Terminal() -> View {
    let (typer, step_typer) =
        create_reducer(Typer::empty(), |typer, step| typer.clone().step(step));
    let step_typer = Rc::new(step_typer);
    step_typer(TyperStep::SetRight(PERSONAL_INFO_TEXT.to_owned()));

    create_signal({
        let interval_1 = Interval::new(50, {
            let step_typer = step_typer.clone();
            move || step_typer(TyperStep::Forward)
        });
        let interval_2 = Interval::new(530, move || {
            if typer.get_clone().is_at_right_end() {
                step_typer(TyperStep::FlipCursorVisibility)
            }
        });
        (interval_1, interval_2)
    });

    let left_string = create_memo(move || typer.get_clone().left_as_string_with_space_for_cursor());
    let right_string = create_memo(move || typer.get_clone().right_as_string());
    view! {
        div(class="flex flex-col") {
            div(class="bg-gray-500 rounded-t-xl border-t-1 border-x-1 border-gray-600 p-2") {
                div(class="flex gap-x-2") {
                    div(class="w-4 h-4 rounded-full bg-red-400")
                    div(class="w-4 h-4 rounded-full bg-yellow-400")
                    div(class="w-4 h-4 rounded-full bg-green-400")
                }
            }
            div(class="glass rounded-b-xl border-b-1 border-x-1 border-gray-600 p-4") {
                pre(class="font-roboto font-bold text-lg leading-6") {
                    code() {
                        (left_string)
                        (if typer.get_clone().is_cursor_visible() {"\u{2588}"} else {" "})
                    }
                    code(class="opacity-0") {
                        (right_string)
                    }
                }
            }
        }
    }
}
