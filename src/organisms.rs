use sycamore::prelude::*;

use crate::atoms::Collapse;
use crate::molecules::*;

#[component]
pub fn Header() -> View {
    view! {
        header(class="z-2 fixed top-0 w-full flex justify-center content-center py-2") {
            a(href="/maitreyakv-resume.pdf", download="maitreyakv-resume.pdf", on:click=|_| {}) {
                "Click to download my resume!"
            }
        }
    }
}

#[component]
pub fn AboutCard() -> View {
    let CarouselItemPosition(position) = use_context();

    view! {
        Card(id="card__about") {
            CardHeader() {
                CardImage() {
                    img(
                        class="w-[100px] border-2 border-black rounded-full",
                        src="assets/face.jpeg",
                        alt="A picture of my face",
                    )
                }
                CardSummary() {
                    h2() { "@maitreyakv" }
                    p() { "I'm Maitreya Venkataswamy and I'm a software engineer based in Boston. "
                          "Learn about my career and interests below!" }
                }
            }
            Collapse(open=move || position.get() == 0) {
                CardContent() {
                    p() { "I'm pursuing a career in software development, but I come from a scientific/engineering "
                          "background and I love working on problems in those domains."}
                    p() { "I'm most experienced in data engineering and backend development, but I also dabble in "
                          " frontend and am looking for opportunities to grow my skills there." }
                    p() {
                        "In my free time I enjoy hiking, playing videogames, reading science fiction and fantasy, and "
                        "playing with my dog "
                        a(href="https://www.instagram.com/bumblebee.the.bully") { "Bumblebee" }
                        "."
                    }
                    div(class="mt-2 flex flex-rows gap-x-6 justify-center align-center") {
                        a(href="https://github.com/maitreyakv") {
                            img(class="w-[30px]", src="assets/github.svg", alt="The GitHub logo")
                        }
                        a(href="https://www.linkedin.com/in/maitreyakv/") {
                            img(class="w-[30px]", src="assets/linkedin.png", alt="The Linkedin logo")
                        }
                        a(href="mailto:maitreyakv@gmail.com") {
                            img(class="w-[30px]", src="assets/email.svg", alt="An mail icon")
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn JobCard() -> View {
    let CarouselItemPosition(position) = use_context();
    view! {
        Card(id="card__job") {
            CardHeader() {
                CardImage() {
                    img(class="w-[100px]", src="assets/road.svg", alt="A winding path")
                }
                CardSummary() {
                    h2() { "My path to Software Engineer" }
                    p() { "I've made the transition from a data scientist, to a data engineer, and finally now to a software engineer." }
                }
            }
            Collapse(open=move || position.get() == 0) {
                CardContent() {
                    h3() {
                        a(href="https://www.titanaes.com") { "Titan Advanced Energy Solutions" }
                    }
                    p() { "I first worked as a Data Scientist at Titan, where I developed machine learning models to "
                          "inspect and monitor lithium-ion batteries using ultrasound technology." }
                    p() { "I engineered a lot of the software (Python) and cloud infrastructure (AWS) to support the required data "
                          "collection experiments and signal processing techniques." }
                    h3() {
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

                }
            }
        }
    }
}

#[component]
pub fn SchoolCard() -> View {
    let CarouselItemPosition(position) = use_context();
    view! {
        Card(id="card__school") {
            CardHeader() {
                CardImage() {
                    img(class="w-[100px]", src="assets/cap.svg", alt="A graduation cap")
                }
                CardSummary() {
                    h2() { "What I studied in school" }
                    p() { "My engineering and data science backgrounds have been helpful for building software "
                          "in various domains."}
                }
            }
            Collapse(open=move || position.get() == 0) {
                CardContent() {
                    p() { "I have a B.S. in Aerospace Engineering and a minor in computer science from the Georgia Institute of Technology." }
                    p() { "While I was there I also did four years of undergraduate research part-time at the Computational Combustion Laboratory." }
                    p() { "I have a M.S. in Data Science from Brown University as well." }
                }
            }
        }
    }
}

#[component]
pub fn SkillsCard() -> View {
    let CarouselItemPosition(position) = use_context();
    view! {
        Card(id="card__skills") {
            CardHeader() {
                CardImage() {
                    img(class="w-[100px]", src="assets/wrench.svg", alt="A wrench")
                }
                CardSummary() {
                    h2() { "Other software skills" }
                    p() { "I have a variety of technical skills covering backend development, data engineering, and frontend engineering." }
                }
            }
            Collapse(open=move || position.get() == 0) {
                CardContent() {
                    p() { "Most of my database experience is with " b() { "Postgres " } "and " b() { "Snowflake" }
                          ", and I've used " b() { "Prefect" } " regularly for orchestration." }
                    p() { "I've done cloud engineering work in both " b() { "GCP" } " and " b() { "AWS" } ", and I'm comfortable "
                          "working with " b() { "Docker" } " and " b() { "Kubernetes"} "." }
                    p() { "In addition to " b() { "Python" } " and " b() { "Rust" } ", I also have some " b() { "Javascript" }
                          " experience for frontend development, primarily in " b() { "React" } "." }
                    p() { "Additionally, I have experience with tasks like gathering requirements for software systems "
                          "and translating scientific R&D algorithms and pipelines into production software." }
                }
            }
        }
    }
}

#[component]
pub fn PythonCard() -> View {
    let CarouselItemPosition(position) = use_context();
    view! {
        Card(id="card__python") {
            CardHeader() {
                CardImage() {
                    img(class="w-[100px]", src="assets/python.svg", alt="The Python programming language logo")
                }
                CardSummary() {
                    h2() { "Python, my bread and butter" }
                    p() { "I've been programming in Python since high school, for both software development "
                          "and as an scientific and engineering tool." }
                }
            }
            Collapse(open=move || position.get() == 0) {
                CardContent() {
                    p() { "I've used Python in various applications from scientific and engineering problems, "
                          "to machine learning and data analysis, to backend engineering and application development." }
                    p() { "I'm comfortable working with large parts of the Python library ecosystem, including" }
                    ul(class="list-none") {
                        div(class="flex flex-row justify-around items-start") {
                            li(class="flex flex-col justify-center") {
                                label(class="font-bold text-center") { "Data" }
                                ul(class="flex flex-col align-center") {
                                    li(class="text-center") { "Pandas/Polars" }
                                    li(class="text-center") { "SQLAlchemy" }
                                }
                            }
                            li(class="flex flex-col justify-center") {
                                label(class="font-bold text-center") { "Engineering" }
                                ul(class="flex flex-col align-center") {
                                    li(class="text-center") { "FastAPI" }
                                    li(class="text-center") { "Prefect" }
                                }
                            }
                            li(class="flex flex-col justify-center") {
                                label(class="font-bold text-center") { "Science" }
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
}

#[component]
pub fn RustCard() -> View {
    let CarouselItemPosition(position) = use_context();
    view! {
        Card(id="card__rust") {
            CardHeader() {
                CardImage() {
                    img(class="w-[100px]", src="assets/rust.svg", alt="The Rust programming language logo")
                }
                CardSummary() {
                    h2() { "Rust, my new obsession" }
                    p() { "Rust has quickly become my favorite language, and I'm looking for more opportunities "
                          "to build with it, personally and professionally!" }
                }
            }
            Collapse(open=move || position.get() == 0) {
                CardContent() {
                    p() { "As a general-purpose language with some amazing features, I've made it my preferred "
                          " language for most software development." }
                    p() { "While I'm not quite an expert at the language yet, I've used it in personal projects (like this site!) "
                          "and professional projects using crates like" }
                    ul(class="list-none") {
                        div(class="flex flex-row justify-around items-start") {
                            li(class="flex flex-col justify-center") {
                                label(class="font-bold text-center") { "General" }
                                ul(class="flex flex-col align-center") {
                                    li(class="text-center") { "Clap" }
                                    li(class="text-center") { "Tokio" }
                                }
                            }
                            li(class="flex flex-col justify-center") {
                                label(class="font-bold text-center") { "Backend" }
                                ul(class="flex flex-col align-center") {
                                    li(class="text-center") { "SeaORM" }
                                    li(class="text-center") { "Axum" }
                                }
                            }
                            li(class="flex flex-col justify-center") {
                                label(class="font-bold text-center") { "Frontend" }
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
}

#[component]
pub fn MetaCard() -> View {
    let CarouselItemPosition(position) = use_context();
    view! {
        Card(id="card__meta") {
            CardHeader() {
                CardImage() {
                    img(class="w-[100px]", src="assets/finger.svg", alt="A hand with the index finger pointing up")
                }
                CardSummary() {
                    h2() { "Source code for this site" }
                    p() {
                        "Want to see how this site works? Check out "
                        a(href="https://github.com/maitreyakv/maitreyakv") { "the code on GitHub" }
                        ". Its made with Rust and Tailwind!"
                    }
                }
            }
            Collapse(open=move || position.get() == 0) {
                CardContent() {
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
    }
}

#[component]
pub fn YarnHoardCard() -> View {
    let CarouselItemPosition(position) = use_context();
    view! {
        Card(id="card__yarn-hoard") {
            CardHeader() {
                CardImage() {
                    img(class="w-[100px]", src="assets/construction.svg", alt="A construction sign")
                }
                CardSummary() {
                    h2() { "YarnHoard, track your stash" }
                    p() { "A full stack app for tracking your yarn inventory, for crafty people. The entire app "
                          "is written in Rust!" }
                }
            }
            Collapse(open=move || position.get() == 0) {
                CardContent() {
                    h2(class="text-center") { "Work in progress, coming soon!" }
                    p() { "Its a pretty straightforward CRUD app with email and password login, its main functionality "
                          "being allowing the user to create and manage records for their yarn inventory and record "
                          "information about their collection." }
                }
            }
        }
    }
}
