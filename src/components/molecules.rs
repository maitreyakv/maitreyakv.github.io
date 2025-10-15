use chrono::Datelike;
use gloo::timers::callback::Timeout;
use sycamore::prelude::*;
use sycamore_router::navigate;

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
        div(class="italic text-white font-bold", style=style) {
            (children)
        }
    }
}

#[component(inline_props)]
pub fn Header(return_delay_ms: Option<u32>, return_callback: impl Fn() + 'static) -> View {
    let return_delay_ms = return_delay_ms.unwrap_or(0);
    view! {
        // TODO: Lift this up to a common Page component
        header(class="glass glass-border w-full") {
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

#[component]
pub fn Footer() -> View {
    view! {
        div(class="glass glass-border p-4") {
            div(class="flex flex-wrap gap-y-2 justify-between") {
                Socials()
                p() {
                    "All rights reserved \u{00A9} Maitreya Venkataswamy "
                    (chrono::Local::now().year().to_string())
                }
            }
        }
    }
}

#[component]
fn Socials() -> View {
    view! {
        div(class="flex gap-x-6 justify-left align-center") {
            Social(
                href="https://github.com/maitreyakv",
                src="assets/github.svg",
                text="GitHub"
            )
            Social(
                href="https://www.linkedin.com/in/maitreyakv/",
                src="assets/linkedin.svg",
                text="LinkedIn"
            )
            Social(
                href="mailto:maitreyakv@gmail.com",
                src="assets/email.svg",
                text="Email"
            )
            // FIXME: Need to click twice to download
            Social(
                href="/maitreyakv-resume.pdf",
                src="assets/doc.svg",
                download="maitreyakv-resume.pdf",
                text="Resume"
            )
        }
    }
}

#[component(inline_props)]
fn Social(
    href: &'static str,
    src: &'static str,
    text: &'static str,
    download: Option<&'static str>,
) -> View {
    view! {
        a(href=href, download=download) {
            div(class="flex items-center gap-x-2") {
                img(class="w-5", src=src)
                p() {
                    (text)
                }
            }
        }
    }
}
