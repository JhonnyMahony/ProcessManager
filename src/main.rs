mod app;
mod components;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<app::App>::new().render();
}
