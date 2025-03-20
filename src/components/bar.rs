use dioxus::prelude::*;

use crate::components::parallax::ParallaxDiv;
use crate::use_parallax;
use crate::ParallaxVector;

const FAVICON: Asset = asset!("/assets/images/favicon.svg");

#[derive(Props, PartialEq, Clone)]
pub struct BarProps {
    #[props(default = (0., 0.))]
    offset: ParallaxVector,
    #[props(default = (0.005, 0.05))]
    multiplier: ParallaxVector,
}

#[component]
pub fn Bar(props: BarProps) -> Element {
    rsx! {
        ParallaxDiv {
            id: "bar",
            class: "fixed top-0 left-0 flex items-center p-4 gap-2 w-[calc(100vw-0.5rem)] h-[10vh] z-10",
            offset: props.offset,
            multiplier: props.multiplier,

            img {
                class: "backdrop-blur-lg h-[calc(9vh-0.5rem*2)]",
                src: FAVICON
            }

            a {
                class: "backdrop-blur-lg font-bold text-kizu-blue font-fredoka",
                style: "text-shadow: 1px 1px var(--color-fg); font-size: 2rem;",
                href: "#home",
                "Aivan"
            }

            div {
                class: "m-auto",
            }

            a {
                class: "px-2 backdrop-blur-lg no-underline border-b-0 text-kizu-fg hover:border-b \
                    hover:border-kizu-fg hover:font-bold transition-all duration-100 active:text-kizu-red",
                href: "#about",
                "About"
            }
            a {
                class: "px-2 backdrop-blur-lg no-underline border-b-0 text-kizu-fg hover:border-b \
                    hover:border-kizu-fg hover:font-bold transition-all duration-100 active:text-kizu-red",
                href: "#technologies",
                "Technologies"
            }
            a {
                class: "px-2 backdrop-blur-lg no-underline border-b-0 text-kizu-fg hover:border-b \
                    hover:border-kizu-fg hover:font-bold transition-all duration-100 active:text-kizu-red",
                href: "#projects",
                "Projects"
            }
        }
    }
}
