mod app;
mod closures;
mod drawing;
mod glue;
mod particle_system;
mod particles;
fn main() {
    yew::Renderer::<app::App>::new().render();
}
