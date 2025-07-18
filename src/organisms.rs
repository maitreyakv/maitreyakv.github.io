use sycamore::prelude::*;

use crate::atoms::Collapse;
use crate::molecules::*;

#[component]
pub fn Header() -> View {
    view! {
        header(class="z-20 fixed top-0 w-full flex justify-center content-center bg-red-400 border-black border-b-2 py-2") {
            a(href="/maitreyakv-resume.pdf", download="maitreyakv-resume.pdf", on:click=|_| {}) {
                "Click to download my resume!"
            }
        }
    }
}

#[component]
pub fn Footer() -> View {
    view! {
        footer(class="z-20 fixed bottom-0 w-full flex justify-center content-center bg-red-400 border-black border-t-2 py-2") {
            "Scroll to see see more cards!"
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
                    h1() { "@maitreyakv" }
                    p() { "Howdy, I'm Maitreya Venkataswamy and I'm a data/software engineer based in Boston. "
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
                        "In my free time I enjoy playing videogames, reading science fiction and fantasy, and "
                        "playing with my dog "
                        a(href="https://www.instagram.com/bumblebee.the.bully") { "Bumblebee" }
                        "."
                    }
                    div(class="mt-2 flex flex-rows gap-x-4 justify-center align-center") {
                        a(href="https://github.com/maitreyakv") {
                            img(class="w-[20px]", src="assets/github.svg", alt="The GitHub logo")
                        }
                        a(href="https://www.linkedin.com/in/maitreyakv/") {
                            img(class="w-[20px]", src="assets/linkedin.png", alt="The Linkedin logo")
                        }
                        a(href="mailto:maitreyakv@gmail.com") {
                            img(class="w-[20px]", src="assets/email.svg", alt="An mail icon")
                        }
                    }
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
                    h1() { "Python, my bread and butter" }
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
                    h1() { "Rust, my new obsession" }
                    p() { "Rust has quickly become my favorite language, and I'm looking for more opportunities "
                          "to build with it, personally and professionally!" }
                }
            }
            Collapse(open=move || position.get() == 0) {
                CardContent() {
                    p() { "As a general-purpose language with an amazing feature set, I've made it my preferred "
                          " language for most software development." }
                    p() { "While I'm not quite an expert at the language yet, I've used it in personal (like this site!) "
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
pub fn YarnHoardCard() -> View {
    let CarouselItemPosition(position) = use_context();
    view! {
        Card(id="card__yarn-hoard") {
            CardHeader() {
                CardImage() {
                    img(class="w-[100px]", src="assets/construction.svg", alt="A construction sign")
                }
                CardSummary() {
                    h1() { "YarnHoard, track your stash" }
                    p() { "A full stack app for tracking your yarn inventory, for crafty people. The entire app "
                          "is written in Rust!" }
                }
            }
            Collapse(open=move || position.get() == 0) {
                CardContent() {
                    h1(class="text-center") { "Work in progress, coming soon!" }
                    p() { "Its a pretty straightforward CRUD app with email and password login, its main functionality "
                          "being allowing the user to create and manage records for their yarn inventory and record "
                          "information about their collection." }
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
                    h1() { "Source code for this site" }
                    p() {
                        "Want to see how this site works? Check out "
                        a(href="https://github.com/maitreyakv/maitreyakv.github.io") { "the code on GitHub" }
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
                }
            }
        }
    }
}
