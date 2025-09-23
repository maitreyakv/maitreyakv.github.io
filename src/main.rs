use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

use site::{
    atoms::ExtrudedText,
    pages::{About, Home, NotFound},
    starscape::Starscape,
};

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
fn _Content() -> View {
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
                    //label(class="text-center") { "Data" }
                    ul(class="flex flex-col align-center") {
                        li(class="text-center") { "Pandas/Polars" }
                        li(class="text-center") { "SQLAlchemy" }
                    }
                }
                li(class="flex flex-col justify-center") {
                    //label(class="text-center") { "Engineering" }
                    ul(class="flex flex-col align-center") {
                        li(class="text-center") { "FastAPI" }
                        li(class="text-center") { "Prefect" }
                    }
                }
                li(class="flex flex-col justify-center") {
                    //label(class="text-center") { "Science" }
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
                    //label(class="text-center") { "General" }
                    ul(class="flex flex-col align-center") {
                        li(class="text-center") { "Clap" }
                        li(class="text-center") { "Tokio" }
                    }
                }
                li(class="flex flex-col justify-center") {
                    //label(class="text-center") { "Backend" }
                    ul(class="flex flex-col align-center") {
                        li(class="text-center") { "SeaORM" }
                        li(class="text-center") { "Axum" }
                    }
                }
                li(class="flex flex-col justify-center") {
                    //label(class="text-center") { "Frontend" }
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
