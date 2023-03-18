use gloo_console::log;
use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, Ref, RefCell, UnsafeCell};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Deref, Index};
use std::rc::Rc;

use gloo_timers::callback::{Interval, Timeout};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

use crate::glue::register_animation_frame;
use crate::particle_system::{Particle, ParticleSystem, ParticleSystemParameters, Vector2};
// ポテンシャルベースの計算もありかも．でもポテンシャルだけだと電磁気力を表現できない．
struct ParticleParam {
    randomness: f64,
    drag: f64,
    params: FrozenSortedMap<(usize, usize), Vector2<f64>>,
}

fn v<T>(x: T, y: T) -> Vector2<T> {
    Vector2 { x, y }
}

static KINDS: usize = 6;
static D_0: f64 = 30.0;
static D_1: f64 = 60.0;
static D_MAX: f64 = 120.0;

thread_local! {
    static RANDOM: RefCell<SmallRng> = RefCell::from(SmallRng::seed_from_u64(0));
}

impl ParticleSystemParameters for ParticleParam {
    type Props = usize;

    fn external_force(&self, p: &Particle<Self::Props>, delta_time: f64) -> Vector2<f64> {
        let r = self.randomness;
        -p.velocity * self.drag + rnd_vec(-r..r)
    }

    fn internal_force(
        &self,
        p_target: &Particle<Self::Props>,
        p_other: &Particle<Self::Props>,
        delta_time: f64,
    ) -> f64 {
        let params = self
            .params
            .get(&(p_target.props, p_other.props))
            .or_else(|| self.params.get(&(p_other.props, p_target.props)))
            .unwrap()
            .to_owned();

        let distance = (p_other.position - p_target.position).length();
        if distance < D_0 {
            params.x * (distance - D_0)
        } else if distance < D_1 {
            params.y * (distance - D_0)
        } else if distance < D_MAX {
            params.y * (D_1 - D_0) * (D_MAX - distance) / (D_MAX - D_1)
        } else {
            0.0
        }
    }
}

fn draw(
    context: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    particles: &[Particle<usize>],
) {
    let w = width as f64;
    let h = height as f64;
    context.clear_rect(0.0, 0.0, w, h);
    for p in particles {
        context.set_fill_style(&JsValue::from(format!(
            "hsl({}, 80%, 50%)",
            p.props as f64 / KINDS as f64 * 360.0
        )));

        let Vector2 { x, y } = p.position;
        let Vector2 { x: vx, y: vy } = p.velocity;
        context.begin_path();
        context
            .arc(x, y, 3.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        context.fill();
        /*
        context.begin_path();
        context.move_to(x, y);
        context.line_to(x + vx, y + vy);
        context.stroke();
        */
    }
}

fn rnd_vec(range: std::ops::Range<f64>) -> Vector2<f64> {
    RANDOM.with(|f| {
        let x = f.borrow_mut().gen_range(range.clone());
        let y = f.borrow_mut().gen_range(range);
        v(x, y)
    })
}

fn rnd(range: std::ops::Range<f64>) -> f64 {
    RANDOM.with(|f| f.borrow_mut().gen_range(range))
}

#[function_component]
pub fn Particles() -> Html {
    let canvas_ref = use_node_ref();

    let system = use_mut_ref(|| {
        let ps: Vec<_> = (0..KINDS)
            .flat_map(|k| {
                (0..30).map(move |_| Particle {
                    props: k,
                    mass: 1.0,
                    position: rnd_vec(0.0..500.0),
                    velocity: v(0.0, 0.0),
                })
            })
            .collect();
        let params = (0..KINDS)
            .flat_map(|k0| (k0..KINDS).map(move |k1| ((k0, k1), rnd_vec(0.0..1.0))))
            .collect::<FrozenSortedMap<_, _>>();

        ParticleSystem::new(
            ParticleParam {
                randomness: 1.0,
                drag: 0.01,
                params,
            },
            ps,
        )
    });
    let canvas = Rc::new(TryLazy::new({
        let canvas_ref = canvas_ref.clone();
        move || canvas_ref.cast::<HtmlCanvasElement>()
    }));

    let context = Rc::new(TryLazy::new({
        let canvas = canvas.clone();
        move || {
            canvas
                .get()
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .ok()
        }
    }));

    {
        let system = system.clone();
        register_animation_frame(move |ts| {
            let Some(ts) = ts else {
                return true;
            };
            let ts = ts / 1000.0;
            let mut s = (*system).borrow_mut();
            s.update(ts);
            let canvas = canvas.get();
            draw(
                context.get(),
                canvas.width(),
                canvas.height(),
                s.particles(),
            );
            return true;
        });
    }

    html! {
        <canvas ref={&canvas_ref} width=500 height=500 />
    }
}

struct FrozenSortedMap<K: Ord, V> {
    vec: Vec<(K, V)>,
}

impl<K: Ord + Copy, V> FromIterator<(K, V)> for FrozenSortedMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut vec = iter.into_iter().collect::<Vec<_>>();
        vec.sort_by(|l, r| l.0.cmp(&r.0));

        FrozenSortedMap { vec }
    }
}

impl<K: Ord + Copy, V> FrozenSortedMap<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        let index = self.vec.binary_search_by_key(key, |pair| pair.0);
        index.ok().map(|i| &self.vec[i].1)
    }
}

struct TryLazy<T, F: FnMut() -> Option<T>> {
    state: UnsafeCell<TryLazyState<T, F>>,
}

enum TryLazyState<T, F: FnMut() -> Option<T>> {
    Uninit(F),
    Initialized(T),
}

impl<T, F: FnMut() -> Option<T>> TryLazy<T, F> {
    fn get(&self) -> &T {
        let state = unsafe { &mut *self.state.get() };

        match state {
            TryLazyState::Initialized(ref value) => value,
            TryLazyState::Uninit(ref mut f) => {
                *state = TryLazyState::Initialized(f().unwrap());
                return self.get();
            }
        }
    }

    fn new(initializer: F) -> TryLazy<T, F> {
        TryLazy {
            state: UnsafeCell::new(TryLazyState::Uninit(initializer)),
        }
    }
}
