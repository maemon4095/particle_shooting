use std::borrow::BorrowMut;
use std::cell::{Cell, Ref, RefCell};

use gloo_timers::callback::Interval;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

use crate::particle_system::{Particle, ParticleSystem, ParticleSystemParameters};
use fixed_vector::{vector as v, Vector};

struct ParticleParam<const N: usize> {
    randomness: f64,
    drag: f64,
    attractions: [[f64; N]; N],
    ramdom: RefCell<SmallRng>,
}

impl<const N: usize> ParticleSystemParameters for ParticleParam<N> {
    type Props = usize;

    fn external_force(&self, p: &Particle<Self::Props>, delta_time: f64) -> Vector<f64, 2> {
        let r = self.randomness;
        let rx = self.ramdom.borrow_mut().gen_range(-r..r);
        let ry = self.ramdom.borrow_mut().gen_range(-r..r);
        -p.velocity * self.drag + v!(rx, ry)
    }

    fn internal_force(
        &self,
        p_target: &Particle<Self::Props>,
        p_other: &Particle<Self::Props>,
        delta_time: f64,
    ) -> f64 {
        0.0
    }
}

fn draw<T>(context: CanvasRenderingContext2d, width: u32, height: u32, particles: &[Particle<T>]) {
    let w = width as f64;
    let h = height as f64;
    context.clear_rect(0.0, 0.0, w, h);

    context.set_fill_style(&JsValue::from("red"));
    for p in particles {
        context.begin_path();
        context
            .arc(p.position[0], p.position[1], 2.0, 0.0, std::f64::consts::PI)
            .unwrap();
        context.fill();
    }
}

#[function_component]
pub fn Particles() -> Html {
    let canvas_ref = use_node_ref();
    let system = use_mut_ref(|| {
        ParticleSystem::new(
            ParticleParam {
                randomness: 0.1,
                drag: 0.1,
                attractions: [[1.0, 0.5], [0.5, 1.0]],
                ramdom: RefCell::from(SmallRng::seed_from_u64(0)),
            },
            [
                Particle {
                    props: 0,
                    mass: 1.0,
                    position: v!(-1.0, 0.0),
                    velocity: v!(0.0, 0.0),
                },
                Particle {
                    props: 0,
                    mass: 1.0,
                    position: v!(1.0, 0.0),
                    velocity: v!(0.0, 0.0),
                },
                Particle {
                    props: 1,
                    mass: 1.0,
                    position: v!(0.0, 1.0),
                    velocity: v!(0.0, 0.0),
                },
                Particle {
                    props: 1,
                    mass: 1.0,
                    position: v!(0.0, -1.0),
                    velocity: v!(0.0, 0.0),
                },
            ],
        )
    });
    {
        let system = system.clone();
        let canvas_ref = canvas_ref.clone();
        use_effect(move || {
            let interval = Interval::new(100, move || {
                let mut s = (*system).borrow_mut();
                s.update(0.1);
                let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
                let context: CanvasRenderingContext2d = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into()
                    .unwrap();
                draw(context, canvas.width(), canvas.height(), s.particles());
            });

            move || {
                interval.forget();
            }
        });
    }

    html! {
        <canvas ref={&canvas_ref} />
    }
}
