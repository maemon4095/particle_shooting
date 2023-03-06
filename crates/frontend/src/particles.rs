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

#[function_component]
pub fn Particles() -> Html {
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
        use_effect(move || {
            let interval = Interval::new(100, move || {
                let mut s = (*system).borrow_mut();
                s.update(0.1);
            });

            move || {
                interval.forget();
            }
        });
    }

    html! {}
}
