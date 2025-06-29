use sycamore::{prelude::*, web::wasm_bindgen::prelude::*};

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|| {
        view! {
            header(class="fixed w-full flex justify-center content-center bg-red-400 border-black border-b-2 py-2") {
                "Scroll to expand each card!"
            }
            div(class="h-full flex justify-center items-center") {
                div(class="max-w-[400px]") {
                    div(class="h-[30px]")
                    Cards()
                    div(class="h-[100vh]")
                }
            }
        }
    });
}

#[component]
fn Cards() -> View {
    view! {
        Card(
            id="card__about",
            image=view! {
                img(
                    class="w-[100px] border-2 border-black rounded-full",
                    src="assets/face.jpeg",
                    alt="A picture of my face",
                )
            },
            summary=view! {
                h1() { "@maitreyakv" }
                p() { "Howdy, I'm Maitreya Venkataswamy and I'm a data/software engineer based in Boston. "
                      "Learn about my career and interests below!" }
            },
            content=view! {
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
            }
        )
        Card(
            id="card__python",
            image=view! {
                img(class="w-[100px]", src="assets/python.svg", alt="The Python programming language logo")
            },
            image_on_left=true,
            summary=view! {
                h1() { "Python, my bread and butter" }
                p() { "I've been programming in Python since high school, for both software development "
                      "and as an scientific and engineering tool." }
            },
            content=view! {
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
        )
        Card(
            id="card__rust",
            image=view!{ img(class="w-[100px]", src="assets/rust.svg", alt="The Rust programming language logo") },
            summary=view!{
                h1() { "Rust, my new obsession" }
                p() { "Rust has quickly become my favorite language, and I'm looking for more opportunities "
                      "to build with it, personally and professionally!" }
            },
            content=view! {
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
        )
        Card(
            id="card__yarn-hoard",
            image=view!{ img(class="w-[100px]", src="assets/construction.svg", alt="A construction sign") },
            image_on_left=true,
            summary=view! {
                h1() { "YarnHoard, track your stash" }
                p() { "A full stack app for tracking your yarn inventory, for crafty people. The entire app "
                      "is written in Rust!" }
            },
            content=view! {
                h1(class="text-center") { "Work in progress, coming soon!" }
                p() { "Its a pretty straightforward CRUD app with email and password login, its main functionality "
                      "being allowing the user to create and manage records for their yarn inventory and record "
                      "information about their collection." }
            }
        )
        Card(
            id="card__meta",
            image=view! { img(class="w-[100px]", src="assets/finger.svg", alt="A hand with the index finger pointing up") },
            summary=view! {
                h1() { "Source code for this site" }
                p() {
                    "Want to see how this site works? Check out "
                    a(href="https://github.com/maitreyakv/maitreyakv") { "the code on GitHub" }
                    ". Its made with Rust and Tailwind!"
                }
            },
            content=view! {
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
        )
    }
}

#[component(inline_props)]
fn Card(
    id: &'static str,
    image: Option<View>,
    image_on_left: Option<bool>,
    summary: Option<View>,
    content: Option<View>,
) -> View {
    let content_is_visible = create_signal(false);

    on_mount({
        let callback = move || {
            let rect = window()
                .document()
                .unwrap()
                .get_element_by_id(&id)
                .unwrap()
                .get_bounding_client_rect();
            let viewport_height = f64::try_from(window().inner_height().unwrap()).unwrap();
            content_is_visible.set(rect.top() < 0.5 * viewport_height);
        };

        move || {
            add_event_listener_with_callback("scroll", Box::new(callback));
            add_event_listener_with_callback("resize", Box::new(callback));
            callback();
        }
    });

    view! {
        div(id=id, class="m-6 p-4 border-2 border-black rounded-2xl shadow-[8px_8px_0px_rgba(0,0,0,1)]") {
            div(class="flex flex-col gap-y-2") {
                div(class="grid grid-cols-3 gap-x-4") {
                    div(class="col-span-2 flex flex-col justify-around gap-y-1") {
                        (summary)
                    }
                    div(
                        data-image-on-left=image_on_left.unwrap_or(false).then_some(""),
                        class="col-span-1 flex justify-center items-center data-image-on-left:-order-1"
                    ) {
                        (image)
                    }
                }
                div(class="col-span-3") {
                    Collapse(open=content_is_visible.into()) {
                        div(class="flex flex-col gap-y-2") {
                            (content)
                        }
                    }
                }
            }
        }
    }
}

#[component(inline_props)]
fn Collapse(open: MaybeDyn<bool>, #[prop(setter(into))] children: Children) -> View {
    view! {
        div(
            data-state=move || if open.get() {"open"} else {"closed"},
            class="overflow-hidden transition-all duration-600 ease-in-out data-[state=closed]:max-h-0 data-[state=open]:max-h-screen"
        ) {
            (children)
        }
    }
}

fn add_event_listener_with_callback(event_type: &str, callback: Box<dyn Fn()>) {
    let closure = Closure::wrap(callback);
    let _ = window().add_event_listener_with_callback(event_type, closure.as_ref().unchecked_ref());
    closure.forget();
}
