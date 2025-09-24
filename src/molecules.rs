use gloo::timers::callback::Timeout;
use sycamore::prelude::*;
use sycamore_router::navigate;

use crate::atoms::Glass;

#[component(inline_props)]
pub fn FancyHandleText(#[prop(setter(into))] children: Children) -> View {
    let text_shadow = (1..=5)
        .map(|n| {
            let offset = 0.05 * n as f32;
            format!("{offset}em {offset}em var(--color-{})", n)
        })
        .reduce(|left, right| format!("{left}, {right}"))
        .unwrap();

    let style = format!("text-shadow: {text_shadow};");

    view! {
        div(class="italic text-white", style=style) {
            (children)
        }
    }
}

#[component(inline_props)]
pub fn Header(return_delay_ms: Option<u32>, return_callback: impl Fn() + 'static) -> View {
    let return_delay_ms = return_delay_ms.unwrap_or(0);
    view! {
        header(class="w-screen max-w-300 p-4") {
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
pub fn Footer() -> View {
    view! {
        div(class="w-full max-w-300 p-4") {
            Glass() {
                div(class="p-4") {
                    div(class="flex justify-between") {
                        SocialLinks()
                        a(
                            class="text-3xl md:text-4xl",
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
