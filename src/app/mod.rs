mod file_systems;
mod processes;

use file_systems::FileSystems;
use processes::Processes;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Routable, Switch};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Processes,
    #[at("/file_suystems")]
    FileSystems,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Processes => {
            html! { <Processes /> }
        }
        Route::FileSystems => {
            html! { <FileSystems /> }
        }
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
    <BrowserRouter>
            <Switch<Route> render={switch} />
    </BrowserRouter>

    }
}
