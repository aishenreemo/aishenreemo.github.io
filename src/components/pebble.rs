use dioxus::prelude::*;
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::components::parallax::ParallaxDiv;
use crate::ParallaxVector;

#[derive(Clone)]
struct PebbleData {
    width: f64,
    height: f64,
    top: f64,
    left: f64,
    opacity: f64,
}

pub type PebbleBounds = (f64, f64);

#[derive(Props, PartialEq, Clone)]
pub struct PebblesProps<const TOTAL: usize> {
    offset: ParallaxVector,
    multiplier: ParallaxVector,
    scale: PebbleBounds,
    opacity: PebbleBounds,
}

impl<const TOTAL: usize> PebblesProps<TOTAL> {
    fn generate(&self, rng: &mut ThreadRng) -> Vec<PebbleData> {
        (0..TOTAL)
            .map(move |_| PebbleData {
                width: rng.gen_range(self.scale.0..self.scale.1),
                height: rng.gen_range(self.scale.0..self.scale.1),
                top: rng.gen_range(0.0..100.0),
                left: rng.gen_range(0.0..100.0),
                opacity: rng.gen_range(self.opacity.0..self.opacity.1),
            })
            .collect()
    }
}

#[component]
pub fn Pebbles<const TOTAL: usize>(props: PebblesProps<TOTAL>) -> Element {
    let mut rng = rand::thread_rng();
    let pebbles = use_hook(|| props.generate(&mut rng));

    rsx! {
        ParallaxDiv {
            class: "pebbles w-screen h-screen absolute top-0 left-0",
            offset: props.offset,
            multiplier: props.multiplier,
            for pebble in pebbles.into_iter() {
                div {
                    class: "absolute rounded-full bg-kizu-fg",
                    style: r#"
                        width: {pebble.width}px;
                        height: {pebble.height}px;
                        top: {pebble.top}%;
                        left: {pebble.left}%;
                        opacity: {pebble.opacity};
                    "#
                }
            }
        }
    }
}
