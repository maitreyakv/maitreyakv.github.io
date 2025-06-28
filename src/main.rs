use sycamore::{prelude::*, web::wasm_bindgen::prelude::*};
use web_sys::Event;

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|| {
        view! {
            header() { "Scroll to expand each card!" }
            div(class="h-[30vh]")
            Card(
                name="about",
                image=view! { CardImage(src="assets/face.jpeg", alt="A picture of my face", round_border=true) },
                summary=view! { CardSummary() {
                    h1() { "@maitreyakv" }
                    p() { "Howdy, I'm Maitreya Venkataswamy and I'm a data/software engineer based in Boston. "
                          "Learn about my career and interests below!" }
                }},
                content=view! { CardContent() {
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
                    div(class="flex flex-rows gap-x-4 justify-center align-center") {
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
                }}
            )
            Card(
                name="python",
                image=view! { CardImage(src="assets/python.svg", alt="The Python programming language logo") },
                image_on_left=true,
                summary=view! { CardSummary() {
                    h1() { "Python, my bread and butter" }
                    p() { "I've been programming in Python since high school, for both software development "
                          "and as an scientific and engineering tool." }
                }},
                content=view! { CardContent() {
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
                }}
            )
            Card(
                name="rust",
                image=view!{ CardImage(src="assets/rust.svg", alt="The Rust programming language logo") },
                summary=view!{ CardSummary() {
                    h1() { "Rust, my new obsession" }
                    p() { "Rust has quickly become my favorite language, and I'm looking for more opportunities "
                          "to build with it, personally and professionally!" }
                }},
                content=view! { CardContent() {
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
                }}
            )
            Card(
                name="yarn-hoard",
                image=view!{ CardImage(src="assets/construction.svg", alt="A construction sign") },
                image_on_left=true,
                summary=view! { CardSummary() {
                    h1() { "YarnHoard, track your stash" }
                    p() { "A full stack app for tracking your yarn inventory, for crafty people. The entire app "
                          "is written in Rust!" }
                }},
                content=view! { CardContent() {
                    h1(class="text-center") { "Work in progress, coming soon!" }
                    p() { "Its a pretty straightforward CRUD app with email and password login, its main functionality "
                          "being allowing the user to create and manage records for their yarn inventory and record "
                          "information about their collection." }
                }}
            )
            Card(
                name="meta",
                image=view! { CardImage(src="assets/finger.svg", alt="A hand with the index finger pointing up") },
                summary=view! { CardSummary() {
                    h1() { "Source code for this site" }
                    p() {
                        "Want to see how this site works? Check out "
                        a(href="https://github.com/maitreyakv/maitreyakv") { "the code on GitHub" }
                        ". Its made with Rust and Tailwind!"
                    }
                }},
                content=view! { CardContent() {
                    p() {
                        "The styling is done with "
                        a(href="https://tailwindcss.com") { "Tailwind CSS" }
                        " and the interactivity is implemented with the delightfully simple "
                        a(href="https://sycamore.dev") { "Sycamore" }
                        " framework in Rust, which is compiled to "
                        a(href="https://webassembly.org") { "WebAssembly" }
                        "."
                    }
                }}
            )
            div(class="h-[40vh]")
            footer(class="fixed z-10 left-0 right-0 flex justify-center content-center bg-red-400 border-black py-2") {
                "Thanks for reading!"
            }
        }
    });
}

#[component(inline_props)]
fn Card(
    name: &'static str,
    image: Option<View>,
    image_on_left: Option<bool>,
    summary: Option<View>,
    content: Option<View>,
) -> View {
    let id = format! {"card__{name}"};

    let content_is_visible = create_signal(false);

    on_mount({
        let id = id.clone();
        move || {
            let closure = Closure::wrap(Box::new(move || {
                let rect = window()
                    .document()
                    .unwrap()
                    .get_element_by_id(&id)
                    .unwrap()
                    .get_bounding_client_rect();
                let half_viewport_height =
                    f64::try_from(window().inner_height().unwrap()).unwrap() / 2.;
                content_is_visible
                    .set(rect.top() < half_viewport_height && rect.bottom() > half_viewport_height);
            }) as Box<dyn Fn()>);
            let _ = window()
                .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            let _ = window()
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
            closure.forget();

            window()
                .dispatch_event(&Event::new("scroll").unwrap())
                .unwrap();
        }
    });

    let image_and_summary = if image_on_left.unwrap_or(false) {
        view! { (image) (summary) }
    } else {
        view! { (summary) (image) }
    };

    view! {
        div(
            id=id,
            class="flex items-center m-6 p-4 border-2 border-black rounded-2xl shadow-[8px_8px_0px_rgba(0,0,0,1)] grid grid-cols-3 gap-x-4 gap-y-2"
        ) {
            (image_and_summary)
            div(class="col-span-3") {
                Collapse(open=content_is_visible.into()) {
                    (content)
                }
            }
        }
    }
}

#[component(inline_props)]
fn Collapse(open: MaybeDyn<bool>, #[prop(setter(into))] children: Children) -> View {
    let class = move || {
        if open.get() {
            "overflow-hidden transition-all duration-600 ease-in-out max-h-screen"
        } else {
            "overflow-hidden transition-all duration-600 ease-in-out max-h-0"
        }
    };

    view! {
        div(class=class) {
            (children)
        }
    }
}

#[component(inline_props)]
fn CardSummary(#[prop(setter(into))] children: Children) -> View {
    view! {
        div(class="h-full col-span-2 flex flex-col justify-around gap-y-1") {
            (children)
        }
    }
}

#[component(inline_props)]
fn CardImage(src: &'static str, alt: &'static str, round_border: Option<bool>) -> View {
    let image_class = if round_border.unwrap_or(false) {
        "w-[100px] border-2 border-black rounded-full"
    } else {
        "w-[100px]"
    };

    view! {
        div(class="flex justify-center items-center col-span-1") {
            img(class=image_class, src=src, alt=alt)
        }
    }
}

#[component(inline_props)]
fn CardContent(#[prop(setter(into))] children: Children) -> View {
    view! {
        div(class="flex flex-col gap-y-2") {
            (children)
        }
    }
}
