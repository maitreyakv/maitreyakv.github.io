use std::{
    cell::{OnceCell, RefCell},
    rc::Rc,
};

use derive_builder::Builder;
use gloo::{
    events::EventListener,
    render::{AnimationFrame, request_animation_frame},
};
use rand::{Rng, seq::IndexedRandom};
use site::utils::{WindowDims, request_animation_frame_loop};
use sycamore::prelude::*;
use web_sys::{Element, HtmlElement, SvgCircleElement, wasm_bindgen::JsCast};

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|| {
        view! {
            Starscape()
            div(class="z-1 overflow-scroll flex justify-center") {
                div(class="flex flex-col gap-y-6 px-6 items-center max-w-150 text-center backdrop-blur-[2px]") {
                    Content()
                    div(class="h-8")
                }
            }
        }
    });
}

#[component(inline_props)]
fn ExtrudedH1(
    #[prop(setter(into))] children: Children,
    color: &'static str,
    depth: Option<f32>,
    resolution: Option<f32>,
) -> View {
    let n_layers = (depth.unwrap_or(4.0) / resolution.unwrap_or(0.1)) as i32;

    let style = (0..=n_layers)
        .into_iter()
        .map(|i| format!("{}px {}px {color}", 0.1 * i as f32, 0.1 * i as f32))
        .fold(String::new(), |a, b| a + ", " + &b);

    view! {
        h1(style=format!("text-shadow: {};", style.strip_prefix(",").unwrap())) {
            (children)
        }
    }
}

#[component]
fn Content() -> View {
    view! {
        div(class=r#"mt-6 mb-4 md:mb-8 font-bold italic text-7xl md:text-9xl text-white
                     text-shadow-[0.05em_0.05em_#0090bc,0.1em_0.1em_#8fb629,0.15em_0.15em_#fce60f,0.2em_0.2em_#f85d1b,0.25em_0.25em_#fd3f8f]"#
        ) { "@maitreyakv" }

        img(
            class="w-[200px] rounded-full",
            src="assets/face.jpeg",
            alt="A picture of my face",
        )
        div(class="flex gap-x-6 justify-center align-center") {
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

        ExtrudedH1(color="#00d492") { "Howdy, I'm Maitreya" }
        h2(class="w-full flex justify-center content-center py-2") {
            a(href="/maitreyakv-resume.pdf", download="maitreyakv-resume.pdf", on:click=|_| {}) {
                "Click to download my resume!"
            }
        }
        p() { "I'm Maitreya Venkataswamy, and I'm a software engineer based in Boston. "
              "Learn about my career and interests below!" }
        p() { "I'm pursuing a career in software development, but I come from a scientific/engineering "
              "background and I love working on problems in those domains."}
        p() { "I'm more experienced in data engineering and backend development, but I also dabble in "
              " frontend and am looking for opportunities to grow my skills there." }
        p() {
            "In my free time I enjoy hiking, playing videogames, reading science fiction and fantasy, and "
            "playing with my dog "
            a(href="https://www.instagram.com/bumblebee.the.bully") { "Bumblebee" }
            "."
        }

        ExtrudedH1(color="#51a2ff") { "Python, my bread and butter" }
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
        ExtrudedH1(color="#f54900") { "Rust, my new obsession" }
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

        ExtrudedH1(color="#00d5be") { "Other software skills" }
        p() { "I have a variety of technical skills covering backend development, data engineering, and frontend engineering." }
        p() { "Most of my database experience is with " b() { "Postgres " } "and " b() { "Snowflake" }
              ", and I've used " b() { "Prefect" } " regularly for orchestration." }
        p() { "I've done cloud engineering work in both " b() { "GCP" } " and " b() { "AWS" } ", and I'm comfortable "
              "working with " b() { "Docker" } " and " b() { "Kubernetes"} "." }
        p() { "In addition to " b() { "Python" } " and " b() { "Rust" } ", I also have some " b() { "Javascript" }
              " experience for frontend development, primarily in " b() { "React" } "." }
        p() { "Additionally, I have experience with tasks like gathering requirements for software systems "
              "and translating scientific R&D algorithms and pipelines into production software." }

        ExtrudedH1(color="#d08700") { "Path to Software Engineer" }
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

        ExtrudedH1(color="#ec003f") { "What I studied in school" }
        p() { "My engineering and data science backgrounds have been helpful for building software "
              "in various domains."}
        p() { "I have a B.S. in Aerospace Engineering and a minor in computer science from the Georgia Institute of Technology." }
        p() { "While I was there I also did four years of undergraduate research part-time at the Computational Combustion Laboratory." }
        p() { "I have a M.S. in Data Science from Brown University as well." }

        ExtrudedH1(color="#0092b8") { "Source code for this site" }
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

        ExtrudedH1(color="#7f22fe") { "YarnHoard, track your stash" }
        p() { "A full stack app for tracking your yarn inventory, for crafty people. The entire app "
              "is written in Rust!" }
        h2(class="text-center") { "Work in progress, coming soon!" }
        p() { "Its a pretty straightforward CRUD app with email and password login, its main functionality "
              "being allowing the user to create and manage records for their yarn inventory and record "
              "information about their collection." }
    }
}

const STAR_COLORS: [&str; 5] = ["#a4c2ff", "#cadaff", "#fff6ed", "#ffcea6", "#ffb16e"];
const STAR_DENSITY: f64 = 0.00075; // stars per square pixel

#[component(inline_props)]
fn Starscape() -> View {
    let window_dims = create_signal(WindowDims::now());
    let _resize_listener_handle =
        create_signal(EventListener::new(&window(), "resize", move |_event| {
            window_dims.set(WindowDims::now());
        }));

    view! {
        div(class="fixed bg-black") {
            svg(height="100vh", width="100vw", xmlns="http://www.w3.org/2000/svg") {
                (move || StarController::new(window_dims.get()).view())
            }
        }
    }
}

#[derive(Clone)]
struct StarController {
    stars: Vec<Star>,
}
impl StarController {
    fn new(window_dims: WindowDims) -> Self {
        let mut rng = rand::rng();
        let n_stars = (window_dims.area() * STAR_DENSITY) as i64;
        let stars = (0..n_stars)
            .map(move |_| {
                StarBuilder::default()
                    .x_initial(rng.random_range(0.0..=window_dims.width))
                    .y_initial(rng.random_range(0.0..=window_dims.height))
                    .depth(rng.random_range(0.0..=1.0))
                    .color(STAR_COLORS.choose(&mut rng).unwrap())
                    .node_ref(create_node_ref())
                    .window_dims(window_dims)
                    .build()
                    .unwrap()
            })
            .collect::<Vec<Star>>();
        Self { stars }
    }

    fn animate(&self, t: f64) {
        for star in &self.stars {
            star.animate(t);
        }
    }

    fn view(self) -> View {
        let views = self
            .stars
            .iter()
            .map(|star| star.view())
            .collect::<Vec<View>>();

        request_animation_frame_loop(move |t| {
            self.animate(t);
        });

        view! {
            (views)
        }
    }
}

#[derive(Builder, Clone)]
struct Star {
    x_initial: f64,
    y_initial: f64,
    depth: f64,
    color: &'static str,
    node_ref: NodeRef,
    window_dims: WindowDims,
}
impl Star {
    fn view(&self) -> View {
        let radius = self.radius().to_string();
        let x = self.x_initial.to_string();
        let y = self.y_initial.to_string();
        view! {
            circle(r#ref=self.node_ref, r=radius, fill=self.color, cx=x, cy=y)
        }
    }

    fn animate(&self, t: f64) {
        let element = self.node_ref.get().dyn_into::<SvgCircleElement>().unwrap();
        let y = (self.y_initial + self.speed() * t) % self.window_dims.height;
        element.set_attribute("cy", &y.to_string()).unwrap();
    }

    fn radius(&self) -> f64 {
        4.0 - self.depth * 2.5
    }

    fn speed(&self) -> f64 {
        0.08 - self.depth * 0.04
    }
}
