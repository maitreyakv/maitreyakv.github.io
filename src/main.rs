//use wasm_bindgen::JsCast;
//use wasm_bindgen::prelude::*;
//use web_sys::window;

use sycamore::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|| {
        view! {
            header() { "Click each card to read more!" }
            div(class="h-10")
            Card(
                name="about",
                image=view! { CardImage(src="assets/face.jpeg", alt="A picture of my face", round_border=true) },
                summary=view! { CardSummary() {
                    h1() { "@maitreyakv" }
                    p() { "Howdy, I'm Maitreya Venkataswamy and I'm a data/software engineer based in Boston. "
                          "Learn about my career and interests below!" }
                }}
            )
            Card(
                name="python",
                image=view! { CardImage(src="assets/python.svg", alt="The Python programming language logo", on_left=true) },
                summary=view! { CardSummary() {
                    h1() { "Python, my bread and butter" }
                    p() { "I've been programming in Python since high school, for both software development "
                          "and as an scientific and engineering tool." }
                }},
                content=&|visible: MaybeDyn<bool>| view! { CardContent(visible=visible) {
                    p() { "I've used Python in various applications from scientific and engineering problems, "
                          "to machine learning and data analysis, to backend engineering and application development." }
                    p() { "I'm comfortable working with large parts of the Python library ecosystem, including" }
                    ul(class="list-none") {
                        div(class="grid grid-cols-2 gap-x-2") {
                            li() {
                                label(class="font-bold") { "Engineering" }
                                p() { "Pandas, Polars, SQLAlchemy, FastAPI" }
                            }
                            li() {
                                label(class="font-bold") { "Data science" }
                                p() { "Numpy, Scikit-Learn, Tensorflow" }
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
                }}
            )
            Card(
                name="yarn-hoard",
                image=view!{ CardImage(src="assets/construction.svg", alt="A construction sign", on_left=true) },
                summary=view! { CardSummary() {
                    h1() { "YarnHoard, track your stash" }
                    p() { "A full stack app for tracking your yarn inventory, for crafty people. The entire app "
                          "is written in Rust! (Work in progress, coming soon)" }
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
                }}
            )
            footer(class="fixed z-10 left-0 right-0 flex justify-center content-center bg-red-400 border-black py-2") {
                "Scroll to see more cards!"
            }
        }
    });
}

//#[component(inline_props)]
//fn Section() -> View {
//    view! {
//        section() {
//            Card(id=format!("card__{name}"), img=img, header=header, summary=summary, direction=direction)
//        }
//    }
//}

#[component(inline_props)]
fn Card(
    name: &'static str,
    image: Option<View>,
    summary: Option<View>,
    content: Option<&'static dyn Fn(MaybeDyn<bool>) -> View>,
) -> View {
    let content_is_visible = create_signal(true);
    let content = match content {
        Some(f) => f(content_is_visible.into()),
        None => view! {},
    };

    view! {
        div(
            on:click=move |_| content_is_visible.set_fn(|val| !val),
            id=format!("card__{name}"),
            class="flex items-center m-6 p-4 border-2 border-black rounded-2xl shadow-[8px_8px_0px_rgba(0,0,0,1)] grid grid-cols-3 gap-x-4 gap-y-2"
        ) {
            (image)
            (summary)
            (content)
        }
    }
}

#[component(inline_props)]
fn CardSummary(#[prop(setter(into))] children: Children) -> View {
    view! {
        div(class="h-full col-span-2 flex flex-col justify-around") {
            (children)
        }
    }
}

#[component(inline_props)]
fn CardImage(
    src: &'static str,
    alt: &'static str,
    round_border: Option<bool>,
    on_left: Option<bool>,
) -> View {
    let image_class = if round_border.unwrap_or(false) {
        "w-[100px] border-2 border-black rounded-full"
    } else {
        "w-[100px]"
    };

    let container_class = if on_left.unwrap_or(false) {
        "flex justify-center items-center col-span-1"
    } else {
        "flex justify-center items-center col-span-1 order-1"
    };

    view! {
        div(class=container_class) {
            img(class=image_class, src=src, alt=alt)
        }
    }
}

#[component(inline_props)]
fn CardContent(visible: MaybeDyn<bool>, #[prop(setter(into))] children: Children) -> View {
    let class = {
        let visible = visible.clone();
        move || {
            if visible.get() {
                "overflow-hidden transition-all duration-300 ease-in-out max-h-0"
            } else {
                "overflow-hidden transition-all duration-300 ease-in-out max-h-screen"
            }
        }
    };

    view! {
        div(class="col-span-3") {
            div(class=class) {
                div(class="flex flex-col gap-y-2") {
                    (children)
                }
            }
        }
    }
}

#[derive(Clone)]
enum Direction {
    Left,
    Right,
}

//let window = window().unwrap();
//
//let closure = Closure::wrap(Box::new(update_page) as Box<dyn Fn()>);
//let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
//let _ = window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
//closure.forget();
//
//// Trigger an update immediately to stage the element positions initially
//update_page()
//}

//fn update_page() {
//    update_sliding_section("section__about", Direction::Left);
//    update_sliding_section("section__python", Direction::Right);
//    update_sliding_section("section__rust", Direction::Left);
//    update_sliding_section("section__yarn-hoard", Direction::Right);
//    update_sliding_section("section__meta", Direction::Left);
//}
//
//fn update_sliding_section(id: &str, direction: Direction) {
//    let window = window().unwrap();
//    let viewport_height = f64::try_from(window.inner_height().unwrap()).unwrap();
//    let element = window.document().unwrap().get_element_by_id(id).unwrap();
//    let rect = element.get_bounding_client_rect();
//    let class_to_hide = match direction {
//        Direction::Left => "-translate-x-full",
//        Direction::Right => "translate-x-full",
//    };
//    let hidden = rect.bottom() > viewport_height;
//    element
//        .class_list()
//        .toggle_with_force(class_to_hide, hidden)
//        .unwrap();
//}
