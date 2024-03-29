use std::{mem::swap, ops::Mul};

use fixed_vector::{fixed_vector, Sqrt, VectorDot};

#[derive(Debug, Clone, Copy)]
#[fixed_vector(T; x, y)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Mul + Copy> Vector2<T>
where
    T::Output: std::iter::Sum,
{
    pub fn square_length(self) -> T::Output {
        self.dot(self)
    }
}

impl<T: Mul + Copy> Vector2<T>
where
    <T as Mul>::Output: std::iter::Sum + Sqrt,
{
    pub fn length(self) -> <<T as Mul>::Output as Sqrt>::Output {
        self.square_length().sqrt()
    }
}

fn v<T>(x: T, y: T) -> Vector2<T> {
    Vector2 { x, y }
}

pub struct ParticleSystem<P: ParticleSystemParameters> {
    particles0: Vec<Particle<P::Props>>,
    particles1: Vec<Particle<P::Props>>,
    params: P,
}

pub trait ParticleSystemParameters {
    type Props: Clone;
    fn external_force(&self, p: &Particle<Self::Props>, delta_time: f64) -> Vector2<f64>;
    fn internal_force(
        &self,
        p_target: &Particle<Self::Props>,
        p_other: &Particle<Self::Props>,
        delta_time: f64,
    ) -> f64;
}

#[derive(Clone, Debug)]
pub struct Particle<Props> {
    pub props: Props,
    pub mass: f64,
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
}

impl<P: ParticleSystemParameters> ParticleSystem<P> {
    pub fn new(
        params: P,
        particles: impl IntoIterator<Item = Particle<P::Props>>,
    ) -> ParticleSystem<P> {
        ParticleSystem {
            particles0: Vec::from_iter(particles),
            particles1: Vec::new(),
            params,
        }
    }

    pub fn particles(&self) -> &[Particle<P::Props>] {
        &self.particles0
    }

    fn calculate_delta_velocity(
        params: &P,
        p0: &Particle<P::Props>,
        p1: &Particle<P::Props>,
        delta_time: f64,
    ) -> Vector2<f64> {
        let delta = p1.position - p0.position;
        let sqr_len = delta.square_length();
        if sqr_len < 0.0001 {
            return v(0.0, 0.0);
        }
        let normal = delta / sqr_len.sqrt();

        let tangent = v(-normal.y, normal.x);

        let f0 = params.external_force(p0, delta_time);
        let f1 = params.external_force(p1, delta_time);
        let f10 = params.internal_force(p0, p1, delta_time);

        let dv_t = {
            let f0 = f0.dot(tangent);

            (f0 / p0.mass) * delta_time
        };
        let dv_n = {
            let f0 = f0.dot(normal);
            let f1 = f1.dot(normal);

            let im0 = 1.0 / p0.mass;
            let im1 = 1.0 / p1.mass;

            let dvc = (f0 + f1) / (p0.mass + p1.mass);
            (dvc - f1 * im1 + f0 * im0 + f10 * (im0 + im1)) * delta_time / 2.0
        };

        normal * dv_n + tangent * dv_t
    }

    pub fn update(&mut self, delta_time: f64) {
        let params = &self.params;
        let source = &mut self.particles0;
        let target = &mut self.particles1;

        for (i, p0) in source.iter().enumerate() {
            let mut dv = v(0.0, 0.0);
            for (j, p1) in source.iter().enumerate() {
                if i == j {
                    continue;
                }
                dv += Self::calculate_delta_velocity(params, p0, p1, delta_time);
            }

            let mut clone = p0.clone();
            clone.velocity += dv;
            target.push(clone);
        }

        for p in target.iter_mut() {
            p.position += p.velocity * delta_time;
        }

        source.clear();
        swap(&mut self.particles0, &mut self.particles1);
    }
}
