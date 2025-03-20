use dioxus::prelude::*;

use super::pebble::Pebbles;

#[component]
pub fn Background() -> Element {
    rsx! {
        div {
            class: "w-screen h-screen fixed inset-0 bg-kizu-bg z-[-1]",

            Pebbles<50> {
                offset: (0., 0.),
                multiplier: (0.12, 0.12),
                scale: (2.0, 6.0),
                opacity: (0.3, 0.8),
            }

            Pebbles<25> {
                offset: (0., 0.),
                multiplier: (0.06, 0.06),
                scale: (16.0, 32.0),
                opacity: (0.1, 0.3),
            }

            Pebbles<10> {
                offset: (0., 0.),
                multiplier: (0.02, 0.02),
                scale: (48.0, 64.0),
                opacity: (0.01, 0.1),
            }
        }
    }
}
