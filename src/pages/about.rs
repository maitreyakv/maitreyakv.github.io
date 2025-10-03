use std::time::Duration;

use gloo::timers::callback::{Interval, Timeout};
use sycamore::prelude::*;

use crate::{
    atoms::{ExtrudedText, Glass, SlideInOut, SlideInOutState},
    molecules::Header,
    pages::Page,
    starscape::State,
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
                        AboutText()
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
            div(class="h-10 md:h-15 w-100 md:w-140") {
                (views)
            }
        }
    }
}

#[component]
fn AboutText() -> View {
    view! {
        Glass() {
            div(class="h-full flex flex-col justify-between gap-y-4 p-6") {
                div(class="text-2xl font-bold") {
                    "Howdy, I'm Maitreya Venkataswamy!"
                }
                div(class="md:text-xl") {
                    "I'm a software developer based in Boston, currently working in "
                    "the biotechnology industry."
                }
                div(class="md:text-xl") {
                    "I come from a scientific/engineering background and I love "
                    "working on software problems in those kinds of domains."
                }
                div(class="md:text-xl") {
                    "I enjoy hiking, playing videogames, reading science "
                    "fiction & fantasy, and playing with my dog "
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
        div(class="flex flex-col") {
            div(class="bg-gray-500 rounded-t-xl border-t-1 border-x-1 border-gray-600 p-2") {
                div(class="flex gap-x-2") {
                    div(class="w-4 h-4 rounded-full bg-red-400")
                    div(class="w-4 h-4 rounded-full bg-yellow-400")
                    div(class="w-4 h-4 rounded-full bg-green-400")
                }
            }
            div(class="backdrop-blur-xs bg-[rgba(255,255,255,0.1)] rounded-b-xl border-b-1 border-x-1 border-gray-600 p-4") {
                TypingCodeBlock(text=PERSONAL_INFO_TEXT)
            }
        }
    }
}

#[component(inline_props)]
fn TypingCodeBlock(text: &'static str) -> View {
    let split_string = create_signal(SplitString::new(text.to_owned()));
    let is_cursor_visible = create_signal(true);

    create_signal(Interval::new(50, move || {
        split_string.set_fn(|s| s.clone().step_forward())
    }));

    create_signal(Interval::new(530, move || {
        if split_string.get_clone().is_right_empty() {
            is_cursor_visible.update(|v| *v = !*v)
        }
    }));

    let left_string = create_memo(move || {
        let split_string = split_string.get_clone();
        let mut left_string = split_string.left_as_string();
        if !split_string.is_right_empty() {
            left_string.pop();
        }
        left_string
    });
    let right_string = create_memo(move || split_string.get_clone().right_as_string());

    view! {
        pre(class="font-roboto font-bold text-lg leading-6") {
            code() {
                (left_string)
                (if is_cursor_visible.get() {"\u{2588}"} else {" "})
            }
            code(class="opacity-0") {
                (right_string)
            }
        }
    }
}

enum Typer {
    Forward(TyperInner<Forward>),
    Idle(TyperInner<Idle>),
}
impl Typer {
    fn step(mut self) -> Self {
        match self {
            Self::Forward(typer) => {
                todo!()
            }
            Self::Idle(_) => self,
        }
    }
}

struct TyperInner<S> {
    split_string: SplitString,
    state: S,
}

struct Forward;
struct Idle;

#[derive(Debug, Clone)]
struct SplitString {
    left: Vec<char>,
    right: Vec<char>,
}
impl SplitString {
    fn new(value: String) -> Self {
        Self {
            left: Vec::new(),
            right: value.chars().rev().collect(),
        }
    }

    // TODO: Check for tail all optimization???
    fn step_forward(mut self) -> Self {
        match self.right.pop() {
            None => self,
            Some(c) if c == '\t' || c == '\n' => {
                self.left.push(c);
                self.step_forward()
            }
            Some(c) => {
                self.left.push(c);
                self
            }
        }
    }

    fn left_as_string(&self) -> String {
        self.left.iter().collect()
    }

    fn right_as_string(&self) -> String {
        self.right.iter().rev().collect()
    }

    fn is_right_empty(&self) -> bool {
        self.right.is_empty()
    }
}
