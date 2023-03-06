mod app;
mod drawing;
mod particle_system;
mod particles;
fn main() {
    yew::Renderer::<app::App>::new().render();
}
