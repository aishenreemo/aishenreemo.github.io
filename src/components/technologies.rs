use dioxus::prelude::*;
use serde::Deserialize;

use super::parallax::ParallaxDiv;

#[derive(Props, PartialEq, Clone)]
pub struct TechnologiesProps {
    category: TechnologyCategory,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TechnologyCategory {
    Languages,
    Frameworks,
    Libraries,
    Platforms,
    Softwares,
}

#[derive(Debug, Deserialize, Clone)]
struct TechnologiesData {
    languages: Vec<TechnologyItem>,
    frameworks: Vec<TechnologyItem>,
    libraries: Vec<TechnologyItem>,
    platforms: Vec<TechnologyItem>,
    softwares: Vec<TechnologyItem>,
}

#[derive(Debug, Deserialize, Clone)]
struct TechnologyItem {
    name: String,
    icon: String,
}

static TECHNOLOGIES_RON_STR: &str = include_str!("../../public/data/technologies.ron");

#[component]
pub fn Technologies(props: TechnologiesProps) -> Element {
    use TechnologyCategory::*;

    let mut active_tab = use_signal(|| props.category);
    let active = |a: TechnologyCategory, b: TechnologyCategory| {
        format!("{}", if a == b { "active" } else { "" })
    };
    let technologies_data: TechnologiesData = use_hook(|| ron::from_str(TECHNOLOGIES_RON_STR).expect("Failed to deserialize."));

    let iter = match active_tab() {
        Languages => technologies_data.languages.iter(),
        Frameworks => technologies_data.frameworks.iter(),
        Libraries => technologies_data.libraries.iter(),
        Platforms => technologies_data.platforms.iter(),
        Softwares => technologies_data.softwares.iter(),
    };

    rsx! {
        ParallaxDiv {
            offset: (0., 0.),
            multiplier: (-0.01, -0.01),
            class: "flex flex-col gap-4 mx-auto bg-kizu-bg border-1 p-4 rounded-md border-kizu-fg max-w-[720px]",

            div {
                class: "text-center",
                "Here is a list of programming languages, frameworks, libraries, tools, platforms and other software I am are proficient in."
            }

            div {
                class: "flex justify-center gap-4 w-full flex-wrap",
                button {
                    class: active(Languages, active_tab()),
                    onclick: move |_| active_tab.set(Languages),
                    "Languages",
                }

                button {
                    class: active(Frameworks, active_tab()),
                    onclick: move |_| active_tab.set(Frameworks),
                    "Frameworks",
                }

                button {
                    class: active(Libraries, active_tab()),
                    onclick: move |_| active_tab.set(Libraries),
                    "Libraries",
                }

                button {
                    class: active(Platforms, active_tab()),
                    onclick: move |_| active_tab.set(Platforms),
                    "Platforms",
                }

                button {
                    class: active(Softwares, active_tab()),
                    onclick: move |_| active_tab.set(Softwares),
                    "Softwares",
                }
            }

            div {
                class: "flex flex-wrap justify-center gap-4 overflow-y-auto max-h-[240px] p-2",
                for item in iter {
                    div {
                        class: "flex flex-col items-center basis-1/4 min-w-32 p-2 border border-kizu-fg/50 rounded-md bg-kizu-fg/10 hover:bg-kizu-fg/20 transition-colors",
                        img {
                            src: "/assets/images/{item.icon}",
                            alt: "{item.name} icon",
                            class: "w-16 h-16 object-contain mb-2",
                        },
                        p {
                            class: "text-kizu-fg text-sm text-center font-semibold",
                            "{item.name}"
                        }
                    }
                }
            }
        }
    }
}
