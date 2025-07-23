use dioxus::prelude::*;

use crate::components::parallax::ParallaxDiv;
use crate::components::parallax::ParallaxImage;

#[component]
pub fn About() -> Element {
    rsx! {
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
                class: "flex-9/16 min-w-9/16 bg-kizu-bg shadow-2xl/80 p-4 rounded-md text-justify",
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
}
