use std::mem::swap;

use fixed_vector::{vector as v, Vector};

pub struct ParticleSystem<P: ParticleSystemParameters> {
    particles0: Vec<Particle<P::Props>>,
    particles1: Vec<Particle<P::Props>>,
    params: P,
}

pub trait ParticleSystemParameters {
    type Props: Clone;
    fn external_force(&self, p: &Particle<Self::Props>, delta_time: f64) -> Vector<f64, 2>;
    fn internal_force(
        &self,
        p_target: &Particle<Self::Props>,
        p_other: &Particle<Self::Props>,
        delta_time: f64,
    ) -> f64;
}

#[derive(Clone)]
pub struct Particle<Props> {
    pub props: Props,
    pub mass: f64,
    pub position: Vector<f64, 2>,
    pub velocity: Vector<f64, 2>,
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

    fn calculate_delta_velocity(
        params: &P,
        p0: &Particle<P::Props>,
        p1: &Particle<P::Props>,
        delta_time: f64,
    ) -> Vector<f64, 2> {
        let normal = (p1.position - p0.position).normalized();
        let tangent = v!(-normal[1], normal[0]);

        let f0 = params.external_force(p0, delta_time);
        let f1 = params.external_force(p1, delta_time);
        let f10 = params.internal_force(p0, p1, delta_time);

        let dv_t = {
            let v0 = p0.velocity.dot(tangent);
            let f0 = f0.dot(tangent);

            f0 / p0.mass * delta_time
        };
        let dv_n = {
            let v0 = p0.velocity.dot(normal);
            let v1 = p1.velocity.dot(normal);
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
            let mut dv = v!(0.0, 0.0);
            for (j, p1) in source.iter().enumerate() {
                if i == j {
                    break;
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
