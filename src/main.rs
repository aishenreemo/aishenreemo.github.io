use std::fmt::Display;

use components::background::Background;
use components::bar::Bar;
use components::parallax::ParallaxDiv;
use components::parallax::ParallaxImage;
use components::technologies::Technologies;
use dioxus::prelude::*;

mod components;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/images/favicon.svg") }
        document::Link { rel: "stylesheet", href: asset!("/assets/styles/main.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/styles/tailwind.css") }

        Window {
            Background { }
            Bar { }

            div {
                id: "home",
                class: "w-screen h-screen",
                ParallaxImage {
                    offset: (-50., -50.),
                    multiplier: (-0.1, -0.1),
                    class: "relative top-1/2 left-1/2 min-w-[320px] w-2/5",
                    src: asset!("/assets/images/aivan.png"),
                }
            }

            div {
                id: "about",
                class: "pt-[10vh] lg:pt-0 w-3/4 mx-auto",
                ParallaxImage {
                    offset: (-50., 0.),
                    multiplier: (-0.01, -0.01),
                    class: "relative left-1/2 w-2/3 pb-4 mb-4 min-w-[320px]",
                    src: asset!("/assets/images/about_me.svg"),
                }

                div {
                    class: "flex w-full justify-stretch items-center gap-4 flex-col md:flex-row",

                    ParallaxImage {
                        offset: (0., 0.),
                        multiplier: (-0.05, -0.05),
                        class: "w-full md:w-[calc(700%/16-1rem)] -rotate-3",
                        src: asset!("assets/images/profile.svg"),
                    }

                    ParallaxDiv {
                        offset: (0., 0.),
                        multiplier: (-0.01, -0.01),
                        class: "flex-9/16 min-w-9/16 bg-kizu-bg border-1 p-4 rounded-md border-kizu-fg text-justify",
                        "My real name is "
                        span {
                            class: "text-kizu-violet",
                            "Aivan Ross"
                        }
                        ", but I prefer "
                        span {
                            class: "text-kizu-violet",
                            "Aishen"
                        }
                        " on the internet. My favorite programming language is "
                        a {
                            target: "_blank",
                            href: "https://www.rust-lang.org",
                            class: "text-kizu-red hover:font-bold",
                            "Rust"
                        }
                        ". "
                        "I'm interested in low-level computing, and data science. "
                        "In my work, my preference extends to minimal technologies such as "
                        a {
                            target: "_blank",
                            href: "https://archlinux.org/",
                            class: "text-kizu-blue hover:font-bold",
                            "Arch Linux",
                        }
                        ", "
                        a {
                            target: "_blank",
                            href: "https://neovim.io/",
                            class: "text-kizu-green hover:font-bold",
                            "Neovim",
                        }
                        ", and other "
                        a {
                            target: "_blank",
                            href: "https://en.wikipedia.org/wiki/Free_and_open-source_software",
                            class: "text-kizu-orange hover:font-bold",
                            "FOSS",
                        }
                        ". "
                        "Currently, I am a Computer Science student at a university in Cavite. "
                        "Outside of code, I find enjoyment in activities such as cycling and "
                        "basketball, and I also enjoy playing guitar."
                    }
                }
            }

            div {
                id: "technologies",
                class: "pt-[10vh] lg:pt-0 my-[10vh] w-3/4 m-auto",

                ParallaxImage {
                    offset: (-50., 0.),
                    multiplier: (-0.01, -0.01),
                    class: "relative left-1/2 w-2/3 pb-4 mb-4 min-w-[320px]",
                    src: asset!("/assets/images/technologies.svg"),
                }

                Technologies {
                    category: components::technologies::TechnologyCategory::Softwares,
                }
            }

            div {
                id: "projects",
                class: "pt-[10vh] lg:pt-0 mt-[10vh] w-3/4 m-auto",

                ParallaxImage {
                    offset: (-50., 0.),
                    multiplier: (-0.01, -0.01),
                    class: "relative left-1/2 w-2/3 pb-4 mb-4 min-w-[320px]",
                    src: asset!("/assets/images/projects.svg"),
                }
            }
        }
    }
}

#[component]
pub fn Window(children: Element) -> Element {
    let window = web_sys::window().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();

    use_context_provider(|| CursorPositionContext::from((0., 0.)));

    rsx! {
        div {
            class: "w-screen h-screen overflow-y-scroll overflow-x-hidden",

            onmousemove: move |event| {
                let mut cursor_position = use_context::<CursorPositionContext>().0;
                let coordinates = event.client_coordinates();
                cursor_position.set((coordinates.x / width * 100. - 50., coordinates.y / height * 100. - 50.));
            },

            {children}
        }
    }
}

pub type ParallaxVector = (f64, f64);

#[derive(Default)]
pub struct ParallaxTransform {
    x: f64,
    y: f64,
}

pub fn use_parallax(offset: ParallaxVector, multiplier: ParallaxVector) -> Signal<ParallaxTransform> {
    let cursor_position = use_context::<CursorPositionContext>().0;
    let mut transform = use_signal(|| ParallaxTransform::default());

    use_effect(move || {
        let cursor_position = cursor_position.read();
        transform.set(ParallaxTransform {
            x: offset.0 + cursor_position.0 * multiplier.0,
            y: offset.1 + cursor_position.1 * multiplier.1,
        });
    });

    transform
}

impl Display for ParallaxTransform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "transform: translate({}%, {}%);", self.x, self.y)
    }
}

#[derive(Clone)]
pub struct CursorPositionContext(pub Signal<(f64, f64)>);

impl From<(f64, f64)> for CursorPositionContext {
    fn from(value: (f64, f64)) -> Self {
        Self(Signal::new(value))
    }
}
