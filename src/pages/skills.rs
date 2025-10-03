use std::time::Duration;

use gloo::timers::callback::Timeout;
use sycamore::prelude::*;

use crate::{
    atoms::{ExtrudedText, SlideInOut, SlideInOutState},
    molecules::Header,
    pages::Page,
    starscape::State,
};

#[component(inline_props)]
pub fn Skills(state: Signal<State>) -> View {
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
                        return_delay_ms=450,
                        return_callback=move || {
                            slide.set(SlideInOutState::Right);
                            state.set(State::Right);
                        }
                    )
                }
            }
            div(class="grow flex flex-col gap-y-8 items-center") {
                SlideInOut(state=*slide, delay=Duration::from_millis(50)) {
                    div(class="text-3xl md:text-5xl mb-4 md:mb-6") {
                        ExtrudedText(color="#51a2ff") { "Python, my bread and butter" }
                    }
                    PythonText()
                }
                SlideInOut(state=*slide, delay=Duration::from_millis(100)) {
                    div(class="text-3xl md:text-5xl mb-4 md:mb-6") {
                        ExtrudedText(color="#f54900") { "Rust, my new obsession" }
                    }
                    RustText()
                }
                SlideInOut(state=*slide, delay=Duration::from_millis(150)) {
                    div(class="text-3xl md:text-5xl mb-4 md:mb-6") {
                        ExtrudedText(color="#00d5be") { "Other software skills" }
                    }
                    OtherSkillsText()
                }
            }
        }
    }
}

#[component]
fn PythonText() -> View {
    view! {
        div(class="glass glass-border") {
            div(class=r#"p-6 flex flex-col gap-y-4 max-w-100 md:max-w-180 text-md md:text-xl"#) {
                p() { "I've been programming in Python since high school, for both software development "
                      "and as an scientific and engineering tool." }
                p() { "I've used Python in various applications from scientific and engineering problems, "
                      "to machine learning and data analysis, to backend engineering and application development." }
                p() { "I'm comfortable working with large parts of the Python library ecosystem, including" }
                ul(class="list-none") {
                    div(class="flex flex-row gap-x-8 justify-center items-start") {
                        li(class="flex flex-col") {
                            //label(class="text-center") { "Data" }
                            ul(class="flex flex-col align-center") {
                                li(class="text-center") { "Pandas/Polars" }
                                li(class="text-center") { "SQLAlchemy" }
                            }
                        }
                        li(class="flex flex-col") {
                            //label(class="text-center") { "Engineering" }
                            ul(class="flex flex-col align-center") {
                                li(class="text-center") { "FastAPI" }
                                li(class="text-center") { "Prefect" }
                            }
                        }
                        li(class="flex flex-col") {
                            //label(class="text-center") { "Science" }
                            ul(class="flex flex-col align-center") {
                                li(class="text-center") { "Scikit-Learn" }
                                li(class="text-center") { "Tensorflow" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RustText() -> View {
    view! {
        div(class="glass glass-border") {
            div(class=r#"p-6 flex flex-col gap-y-4 max-w-100 md:max-w-180 text-md md:text-xl"#) {
                p() { "Rust has quickly become my favorite language, and I'm looking for more opportunities "
                      "to build with it, personally and professionally!" }
                p() { "As a general-purpose language with some amazing features, I've made it my preferred "
                      " language for most software development." }
                p() { "While I'm not an expert at the language yet, I've used it in personal projects (like this site!) "
                      "and professional projects using crates like" }
                ul(class="list-none") {
                    div(class="flex flex-row gap-x-8 justify-center items-start") {
                        li(class="flex flex-col") {
                            //label(class="text-center") { "General" }
                            ul(class="flex flex-col align-center") {
                                li(class="text-center") { "Clap" }
                                li(class="text-center") { "Tokio" }
                            }
                        }
                        li(class="flex flex-col") {
                            //label(class="text-center") { "Backend" }
                            ul(class="flex flex-col align-center") {
                                li(class="text-center") { "SeaORM" }
                                li(class="text-center") { "Axum" }
                            }
                        }
                        li(class="flex flex-col") {
                            //label(class="text-center") { "Frontend" }
                            ul(class="flex flex-col align-center") {
                                li(class="text-center") { "Sycamore" }
                                li(class="text-center") { "web_sys" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn OtherSkillsText() -> View {
    view! {
        div(class="glass glass-border") {
            div(class=r#"p-6 flex flex-col gap-y-4 max-w-100 md:max-w-180 text-md md:text-xl"#) {
                p() { "I have a variety of technical skills covering backend development, data engineering, and frontend engineering." }
                p() { "Most of my database experience is with " b() { "Postgres " } "and " b() { "Snowflake" }
                      ", and I've used " b() { "Prefect" } " regularly for orchestration." }
                p() { "I've done cloud engineering work in both " b() { "GCP" } " and " b() { "AWS" } ", and I'm comfortable "
                      "working with " b() { "Docker" } " and " b() { "Kubernetes"} "." }
                p() { "In addition to " b() { "Python" } " and " b() { "Rust" } ", I also have " b() { "Javascript" }
                      " experience for frontend development, primarily in " b() { "React" } "." }
                p() { "Additionally, I have experience with tasks like gathering requirements for software systems "
                      "and translating scientific R&D algorithms and pipelines into production software." }
                }
            }

    }
}
