use std::time::Duration;

use gloo::timers::callback::Timeout;
use site::{
    atoms::{ExtrudedText, SlideInOut, SlideInOutState},
    starscape::Starscape,
};
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router, navigate};

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|| App());
}

#[component]
fn App() -> View {
    view! {
        Starscape()
        Router(
            integration=HistoryIntegration::new(),
            view=|route| view! {
                (match route.get_clone() {
                    AppRoutes::Home => Home(),
                    AppRoutes::About => About(),
                    AppRoutes::NotFound => NotFound(),
                })
            }
        )
    }
}

#[derive(Route, Clone)]
enum AppRoutes {
    #[to("/")]
    Home,
    #[to("/about")]
    About,
    #[not_found]
    NotFound,
}

#[component]
fn NotFound() -> View {
    let slide = create_signal(SlideInOutState::Right);
    on_mount(move || {
        Timeout::new(10, move || slide.set(SlideInOutState::OnScreen)).forget();
    });

    view! {
        div(class="w-screen h-screen flex flex-col") {
            SlideInOut(state=*slide) {
                Header(return_callback=move || slide.set(SlideInOutState::Right), return_delay_ms=400)
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

#[component]
fn Home() -> View {
    let slide = create_signal(SlideInOutState::Left);
    on_mount(move || {
        Timeout::new(10, move || slide.set(SlideInOutState::OnScreen)).forget();
    });

    view! {
        div(class="w-screen h-screen flex flex-col") {
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
                    PageLink(slide=slide, delay_ms=100, url="/about", color="var(--color-1)") { "About" }
                    PageLink(slide=slide, delay_ms=150, url="/skills", color="var(--color-2)") { "Skills" }
                    PageLink(slide=slide, delay_ms=200, url="/career", color="var(--color-4)") { "Career" }
                    PageLink(slide=slide, delay_ms=250, url="/projects", color="var(--color-5)") { "Projects" }
                }
            }
            Footer()
        }
    }
}

#[component(inline_props)]
fn PageLink(
    slide: Signal<SlideInOutState>,
    delay_ms: u64,
    url: &'static str,
    color: &'static str,
    #[prop(setter(into))] children: Children,
) -> View {
    let navigate_after_delay = move |_event| {
        slide.set(SlideInOutState::Left);
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

#[component(inline_props)]
fn Header(return_delay_ms: Option<u32>, return_callback: impl Fn() + 'static) -> View {
    let return_delay_ms = return_delay_ms.unwrap_or(0);
    view! {
        header(class="w-screen p-4") {
            Glass() {
                div(class="text-3xl md:text-5xl hover:cursor-pointer") {
                    div(
                        on:click=move |_event| {
                            return_callback();
                            Timeout::new(return_delay_ms, || navigate("/")).forget();
                        }
                    ) {
                        div(class="pt-4 pb-6 pl-4") {
                            FancyHandleText() { "Home" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Footer() -> View {
    view! {
        div(class="w-full p-4") {
            Glass() {
                div(class="p-4") {
                    div(class="flex justify-between") {
                        SocialLinks()
                        a(
                            class="font-bold text-3xl md:text-4xl",
                            style="text-decoration: none;",
                            href="/maitreyakv-resume.pdf",
                            download="maitreyakv-resume.pdf",
                            on:click=|_| {}
                        ) { "Resume" }
                    }
                }
            }
        }
    }
}

#[component(inline_props)]
fn Glass(#[prop(setter(into))] children: Children) -> View {
    view! {
        div(class="backdrop-blur-xs bg-[rgba(255,255,255,0.1)] border-gray-600 border-1 rounded-xl") {
            (children)
        }
    }
}

#[component]
fn About() -> View {
    let slide = create_signal(SlideInOutState::Right);
    on_mount(move || {
        Timeout::new(10, move || slide.set(SlideInOutState::OnScreen)).forget();
    });

    view! {
        div(class="w-screen h-screen flex flex-col items-center") {
            SlideInOut(state=*slide) {
                Header(return_delay_ms=400, return_callback=move || slide.set(SlideInOutState::Right))
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
                    Glass() {
                        div(class=r#"p-6 flex flex-col gap-y-4 max-w-100 md:max-w-180 "
                                     font-bold text-md md:text-xl"#) {
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
            Footer()
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

#[component]
fn SocialLinks() -> View {
    view! {
        div(class="flex gap-x-6 justify-left align-center") {
            a(href="https://github.com/maitreyakv") {
                img(class="w-[40px]", src="assets/github.svg", alt="The GitHub logo")
            }
            a(href="https://www.linkedin.com/in/maitreyakv/") {
                img(class="w-[40px]", src="assets/linkedin.svg", alt="The Linkedin logo")
            }
            a(href="mailto:maitreyakv@gmail.com") {
                img(class="w-[50px]", src="assets/email.svg", alt="An mail icon")
            }
        }
    }
}

// TODO: Use rotating "conic-gradient" to give dynamic border
#[component]
fn ProfilePhoto() -> View {
    view! {
        img(
            class="w-48 md:w-64 rounded-full",
            src="assets/face.jpeg",
            alt="A picture of my face",
        )
    }
}

#[component(inline_props)]
fn FancyHandleText(#[prop(setter(into))] children: Children) -> View {
    let text_shadow = (1..=5)
        .map(|n| {
            let offset = 0.05 * n as f32;
            format!("{offset}em {offset}em var(--color-{})", n)
        })
        .reduce(|left, right| format!("{left}, {right}"))
        .unwrap();

    let style = format!("text-shadow: {text_shadow};");

    view! {
        div(class="font-bold italic text-white", style=style) {
            (children)
        }
    }
}

#[component]
fn Content() -> View {
    view! {
        ExtrudedText(color="#51a2ff") { "Python, my bread and butter" }
        p() { "I've been programming in Python since high school, for both software development "
              "and as an scientific and engineering tool." }
        p() { "I've used Python in various applications from scientific and engineering problems, "
              "to machine learning and data analysis, to backend engineering and application development." }
        p() { "I'm comfortable working with large parts of the Python library ecosystem, including" }
        ul(class="list-none") {
            div(class="flex flex-row gap-x-8 items-start") {
                li(class="flex flex-col justify-center") {
                    //label(class="font-bold text-center") { "Data" }
                    ul(class="flex flex-col align-center") {
                        li(class="text-center") { "Pandas/Polars" }
                        li(class="text-center") { "SQLAlchemy" }
                    }
                }
                li(class="flex flex-col justify-center") {
                    //label(class="font-bold text-center") { "Engineering" }
                    ul(class="flex flex-col align-center") {
                        li(class="text-center") { "FastAPI" }
                        li(class="text-center") { "Prefect" }
                    }
                }
                li(class="flex flex-col justify-center") {
                    //label(class="font-bold text-center") { "Science" }
                    ul(class="flex flex-col align-center") {
                        li(class="text-center") { "Scikit-Learn" }
                        li(class="text-center") { "Tensorflow" }
                    }
                }
            }
        }
        ExtrudedText(color="#f54900") { "Rust, my new obsession" }
        p() { "Rust has quickly become my favorite language, and I'm looking for more opportunities "
              "to build with it, personally and professionally!" }
        p() { "As a general-purpose language with some amazing features, I've made it my preferred "
              " language for most software development." }
        p() { "While I'm not quite an expert at the language yet, I've used it in personal projects (like this site!) "
              "and professional projects using crates like" }
        ul(class="list-none") {
            div(class="flex flex-row gap-x-8 items-start") {
                li(class="flex flex-col justify-center") {
                    //label(class="font-bold text-center") { "General" }
                    ul(class="flex flex-col align-center") {
                        li(class="text-center") { "Clap" }
                        li(class="text-center") { "Tokio" }
                    }
                }
                li(class="flex flex-col justify-center") {
                    //label(class="font-bold text-center") { "Backend" }
                    ul(class="flex flex-col align-center") {
                        li(class="text-center") { "SeaORM" }
                        li(class="text-center") { "Axum" }
                    }
                }
                li(class="flex flex-col justify-center") {
                    //label(class="font-bold text-center") { "Frontend" }
                    ul(class="flex flex-col align-center") {
                        li(class="text-center") { "Sycamore" }
                        li(class="text-center") { "web_sys" }
                    }
                }
            }
        }

        ExtrudedText(color="#00d5be") { "Other software skills" }
        p() { "I have a variety of technical skills covering backend development, data engineering, and frontend engineering." }
        p() { "Most of my database experience is with " b() { "Postgres " } "and " b() { "Snowflake" }
              ", and I've used " b() { "Prefect" } " regularly for orchestration." }
        p() { "I've done cloud engineering work in both " b() { "GCP" } " and " b() { "AWS" } ", and I'm comfortable "
              "working with " b() { "Docker" } " and " b() { "Kubernetes"} "." }
        p() { "In addition to " b() { "Python" } " and " b() { "Rust" } ", I also have some " b() { "Javascript" }
              " experience for frontend development, primarily in " b() { "React" } "." }
        p() { "Additionally, I have experience with tasks like gathering requirements for software systems "
              "and translating scientific R&D algorithms and pipelines into production software." }

        ExtrudedText(color="#d08700") { "Path to Software Engineer" }
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

        ExtrudedText(color="#ec003f") { "What I studied in school" }
        p() { "My engineering and data science backgrounds have been helpful for building software "
              "in various domains."}
        p() { "I have a B.S. in Aerospace Engineering and a minor in computer science from the Georgia Institute of Technology." }
        p() { "While I was there I also did four years of undergraduate research part-time at the Computational Combustion Laboratory." }
        p() { "I have a M.S. in Data Science from Brown University as well." }

        ExtrudedText(color="#0092b8") { "Source code for this site" }
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

        ExtrudedText(color="#7f22fe") { "YarnHoard, track your stash" }
        p() { "A full stack app for tracking your yarn inventory, for crafty people. The entire app "
              "is written in Rust!" }
        h2(class="text-center") { "Work in progress, coming soon!" }
        p() { "Its a pretty straightforward CRUD app with email and password login, its main functionality "
              "being allowing the user to create and manage records for their yarn inventory and record "
              "information about their collection." }
    }
}
