use rand::Rng;
use sycamore::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|| {
        view! {
            Starscape()
            div(class="z-1 overflow-scroll flex justify-center") {
                div(class="flex flex-col gap-y-6 px-2 items-center max-w-110 text-center backdrop-blur-[4px]") {
                    Content()
                }
            }
        }
    });
}

#[component]
fn Content() -> View {
    view! {
        header(class="w-full flex justify-center content-center py-2") {
            a(href="/maitreyakv-resume.pdf", download="maitreyakv-resume.pdf", on:click=|_| {}) {
                "Click to download my resume!"
            }
        }
        img(
            class="w-[100px] border-2 border-black rounded-full",
            src="assets/face.jpeg",
            alt="A picture of my face",
        )

        h1(class="text-shadow-[2px_2px] text-shadow-emerald-400") { "@maitreyakv" }
        p() { "I'm Maitreya Venkataswamy and I'm a software engineer based in Boston. "
              "Learn about my career and interests below!" }
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
        div(class="flex gap-x-6 justify-center align-center") {
            a(href="https://github.com/maitreyakv") {
                img(class="w-[30px]", src="assets/github.svg", alt="The GitHub logo")
            }
            a(href="https://www.linkedin.com/in/maitreyakv/") {
                img(class="w-[30px]", src="assets/linkedin.svg", alt="The Linkedin logo")
            }
            a(href="mailto:maitreyakv@gmail.com") {
                img(class="w-[40px]", src="assets/email.svg", alt="An mail icon")
            }
        }

        h1(class="text-shadow-[2px_2px] text-shadow-blue-400") { "Python, my bread and butter" }
        p() { "I've been programming in Python since high school, for both software development "
              "and as an scientific and engineering tool." }
        p() { "I've used Python in various applications from scientific and engineering problems, "
              "to machine learning and data analysis, to backend engineering and application development." }
        p() { "I'm comfortable working with large parts of the Python library ecosystem, including" }
        ul(class="list-none") {
            div(class="flex flex-row gap-x-8 items-start") {
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
        h1(class="text-shadow-[2px_2px] text-shadow-orange-400") { "Rust, my new obsession" }
        p() { "Rust has quickly become my favorite language, and I'm looking for more opportunities "
              "to build with it, personally and professionally!" }
        p() { "As a general-purpose language with some amazing features, I've made it my preferred "
              " language for most software development." }
        p() { "While I'm not quite an expert at the language yet, I've used it in personal projects (like this site!) "
              "and professional projects using crates like" }
        ul(class="list-none") {
            div(class="flex flex-row gap-x-8 items-start") {
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

        h1(class="text-shadow-[2px_2px] text-shadow-teal-400") { "Other software skills" }
        p() { "I have a variety of technical skills covering backend development, data engineering, and frontend engineering." }
        p() { "Most of my database experience is with " b() { "Postgres " } "and " b() { "Snowflake" }
              ", and I've used " b() { "Prefect" } " regularly for orchestration." }
        p() { "I've done cloud engineering work in both " b() { "GCP" } " and " b() { "AWS" } ", and I'm comfortable "
              "working with " b() { "Docker" } " and " b() { "Kubernetes"} "." }
        p() { "In addition to " b() { "Python" } " and " b() { "Rust" } ", I also have some " b() { "Javascript" }
              " experience for frontend development, primarily in " b() { "React" } "." }
        p() { "Additionally, I have experience with tasks like gathering requirements for software systems "
              "and translating scientific R&D algorithms and pipelines into production software." }

        h1(class="text-shadow-[2px_2px] text-shadow-yellow-400") { "My path to Software Engineer" }
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

        h1(class="text-shadow-[2px_2px] text-shadow-rose-400") { "What I studied in school" }
        p() { "My engineering and data science backgrounds have been helpful for building software "
              "in various domains."}
        p() { "I have a B.S. in Aerospace Engineering and a minor in computer science from the Georgia Institute of Technology." }
        p() { "While I was there I also did four years of undergraduate research part-time at the Computational Combustion Laboratory." }
        p() { "I have a M.S. in Data Science from Brown University as well." }

        h1(class="text-shadow-[2px_2px] text-shadow-cyan-300") { "Source code for this site" }
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

        h1(class="text-shadow-[2px_2px] text-shadow-violet-400") { "YarnHoard, track your stash" }
        p() { "A full stack app for tracking your yarn inventory, for crafty people. The entire app "
              "is written in Rust!" }
        h2(class="text-center") { "Work in progress, coming soon!" }
        p() { "Its a pretty straightforward CRUD app with email and password login, its main functionality "
              "being allowing the user to create and manage records for their yarn inventory and record "
              "information about their collection." }
    }
}

#[component(inline_props)]
fn Starscape() -> View {
    let mut rng = rand::rng();

    let height = create_signal(0);
    let width = create_signal(0);

    let update_window_size_callback = move || {
        let window = web_sys::window().unwrap();
        width.set(window.inner_width().unwrap().as_f64().unwrap() as i32);
        height.set(window.inner_height().unwrap().as_f64().unwrap() as i32);
    };
    let _listener = create_signal(gloo::events::EventListener::new(
        &web_sys::window().unwrap(),
        "resize",
        move |_event| update_window_size_callback(),
    ));

    let stars = create_signal(Vec::new());
    create_effect(move || {
        stars.set(
            (0..=400)
                .into_iter()
                .map(|_| {
                    let x = rng.random_range(0..=width.get());
                    let depth = rng.random_range(0.0..=1.0);
                    let delay = rng.random_range(0.0..=40.0);
                    Star { x, depth, delay }
                })
                .collect::<Vec<Star>>(),
        );
    });

    on_mount(update_window_size_callback);

    view! {
        div(class="fixed") {
            svg(height="100vh", width="100vw", xmlns="http://www.w3.org/2000/svg") {
                Indexed(
                    list=stars,
                    view=move |star| view! {
                        circle(r=(4.0 - star.depth * 2.5).to_string(), cx=star.x.to_string(), cy="0", fill="white") {
                            animate(
                                attributeName="cy",
                                dur=(20.0 * star.depth + 20.0).to_string(),
                                begin=format!("-{}s", star.delay),
                                from="0",
                                to=height.get().to_string(),
                                repeatCount="indefinite"
                            )
                        }
                    }
                )
            }
        }
    }
}

#[derive(PartialEq, Clone)]
struct Star {
    x: i32,
    depth: f32,
    delay: f32,
}
