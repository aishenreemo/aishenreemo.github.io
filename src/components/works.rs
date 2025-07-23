use dioxus::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::components::parallax::ParallaxDiv;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Technology {
    pub name: String,
    pub url: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub technologies: Vec<Technology>,
    pub image_url: String,
    pub project_link: String,
}

static WORKS_RON_STR: &str = include_str!("../../public/data/works.ron");

#[component]
pub fn Works() -> Element {
    let works_data = || ron::from_str(WORKS_RON_STR).expect("Failed to deserialize.");
    let works_data: Vec<Project> = use_hook(works_data);
    let mut active_project: Signal<Option<usize>> = use_signal(|| None);

    rsx! {
        ParallaxDiv {
            offset: (0., 0.),
            multiplier: (-0.01, -0.01),
            class: "flex flex-col-reverse md:flex-row gap-4 mx-auto max-w-[960px] rounded-md mb-10",

            div {
                class: "flex flex-wrap justify-center flex-initial self-stretch md:self-start gap-6 overflow-y-scroll p-2 max-h-[560px]",

                for (i, project) in works_data.iter().enumerate() {
                    if Some(i) != active_project() {
                        div {
                            class: "flex flex-col items-center basis-1/16 md:basis-1/4 min-w-48 p-2 border border-kizu-fg/50 rounded-md bg-kizu-fg/10 hover:bg-kizu-fg/20 transition-colors",
                            onclick: move |_| active_project.set(Some(i)),
                            img {
                                class: "w-full h-32 object-fill",
                                src: "/assets/images/{project.image_url}",
                                alt: "{project.title}",
                            }

                            p {
                                class: "text-kizu-fg text-sm text-center font-semibold",
                                "{project.title}"
                            }
                        }
                    }
                }
            }

            if let Some(i) = active_project() {
                div {
                    class: "flex flex-col ease-in-out w-full md:min-w-88 bg-kizu-bg max-h-[560px]",
                    img {
                        class: "w-full h-56 object-cover",
                        src: "/assets/images/{works_data[i].image_url}",
                        alt: "{works_data[i].title}",
                    }

                    div {
                        class: "p-4 pt-0 flex flex-col flex-grow overflow-y-scroll",

                        div {
                            class: "p-4",
                            p {
                                class: "text-kizu-fg text-center font-semibold hover:text-kizu-blue text-2xl",
                                onclick: move |_| active_project.set(None),
                                "{works_data[i].title}"
                            }
                        }

                        p {
                            class: "text-kizu-fg text-sm text-center mb-8",
                            "{works_data[i].description}"
                        }

                        div {
                            class: "flex flex-wrap gap-2 mb-4",
                            p {
                                class: "text-kizu-fg text-base w-full mb-1",
                                "Technologies Used:"
                            }

                            for tech in works_data[i].technologies.iter() {
                                div {
                                    class: "flex items-center bg-gray-100 rounded-full px-2 py-0.5 text-xs font-medium text-gray-700 shadow-sm",
                                    img {
                                        class: "w-4 h-4 mr-1 rounded-full",
                                        src: "{tech.url}",
                                        alt: "{tech.name} icon",
                                    }
                                    "{tech.name}"
                                }
                            }
                        }

                        a {
                            class: "mt-auto inline-block bg-kizu-bg hover:bg-kizu-blue hover:text-kizu-bg shadow-2xl/80 text-white font-bold py-2 px-4 rounded-lg text-center transition-colors duration-200 shadow-md text-sm",
                            href: "{works_data[i].project_link}",
                            target: "_blank",
                            "View Project"
                        }
                    }
                }
            }
        }
    }
}
