use dioxus::prelude::*;
use serde::Deserialize;

use super::parallax::ParallaxDiv;

#[derive(Props, PartialEq, Clone)]
pub struct TechnologiesProps {
    category: TechnologyCategory,
}

#[derive(PartialEq, Clone, Copy, Debug, Deserialize)]
pub enum TechnologyCategory {
    Languages,
    Frameworks,
    Libraries,
    Platforms,
    Softwares,
}

impl TechnologyCategory {
    pub fn all_variants() -> Vec<TechnologyCategory> {
        vec![
            TechnologyCategory::Languages,
            TechnologyCategory::Frameworks,
            TechnologyCategory::Libraries,
            TechnologyCategory::Platforms,
            TechnologyCategory::Softwares,
        ]
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
struct TechnologyItem {
    name: String,
    icon: String,
    category: TechnologyCategory
}

static TECHNOLOGIES_RON_STR: &str = include_str!("../../public/data/technologies.ron");

#[component]
pub fn Technologies(props: TechnologiesProps) -> Element {
    let technologies_data = || ron::from_str(TECHNOLOGIES_RON_STR).expect("Failed to deserialize.");
    let technologies_data: Vec<TechnologyItem> = use_hook(technologies_data);

    let mut active_tab = use_signal(|| props.category);

    let filtered_data = use_memo(move || {
        let current_category = active_tab();
        technologies_data
            .iter()
            .filter(|item| item.category == current_category)
            .cloned()
            .collect::<Vec<TechnologyItem>>()
    });

    rsx! {
        ParallaxDiv {
            offset: (0., 0.),
            multiplier: (-0.01, -0.01),
            class: "flex flex-col gap-4 mx-auto bg-kizu-bg p-4 shadow-2xl/80 rounded-md max-w-[720px] h-auto transition-[height] duration-200",

            div {
                class: "text-center",
                "Here is a list of programming languages, frameworks, libraries, tools, platforms and other software I am are proficient in."
            }

            div {
                class: "flex justify-center gap-4 w-full flex-wrap",
                for category in TechnologyCategory::all_variants().into_iter() {
                    button {
                        class: (category == active_tab()).then(|| "active").unwrap_or(""),
                        onclick: move |_| active_tab.set(category),
                        "{category:?}",
                    }
                }
            }

            div {
                class: "flex flex-wrap justify-center gap-4 overflow-y-auto max-h-[320px] p-2",
                for item in filtered_data.iter() {
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
