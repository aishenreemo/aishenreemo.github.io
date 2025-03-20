use dioxus::prelude::*;

use crate::use_parallax;
use crate::ParallaxVector;

#[derive(Props, PartialEq, Clone)]
pub struct ParallaxImageProps {
    #[props(default)]
    class: String,
    offset: ParallaxVector,
    multiplier: ParallaxVector,
    src: String,
}

#[component]
pub fn ParallaxImage(props: ParallaxImageProps) -> Element {
    let transform = use_parallax(props.offset, props.multiplier);

    rsx! {
        img {
            style: "{transform}",
            class: props.class,
            src: props.src,
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ParallaxDivProps {
    #[props(default)]
    id: String,
    #[props(default)]
    class: String,
    offset: ParallaxVector,
    multiplier: ParallaxVector,
    children: Element,
}


#[component]
pub fn ParallaxDiv(props: ParallaxDivProps) -> Element {
    let transform = use_parallax(props.offset, props.multiplier);

    rsx! {
        div {
            style: "{transform}",
            class: props.class,
            id: props.id,
            {props.children}
        }
    }
}
