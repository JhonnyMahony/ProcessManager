use yew::{html, prelude::function_component, Callback, Html};
use yew_router::{
    hooks::{use_location, use_navigator},
    Routable,
};

use crate::app::Route;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    let navigator = use_navigator().unwrap();
    let nav_processes = Callback::from({
        let navigator = navigator.clone();
        move |_| navigator.push(&Route::Processes)
    });
    let nav_file_systems = Callback::from(move |_| navigator.push(&Route::FileSystems));

    let location = use_location().unwrap().path().to_string();

    let active_class="cursor-pointer inline-block p-4 text-blue-600 border-b-2 border-blue-600 rounded-t-lg active dark:text-blue-500 dark:border-blue-500";
    let passive_class = "cursor-pointer inline-block p-4 border-b-2 border-transparent rounded-t-lg hover:text-gray-600 hover:border-gray-300 dark:hover:text-gray-300";

    html! {
    <div class="z-50 bg-gray-50 absolute sticky top-0 text-base font-medium text-center text-gray-500 border-b border-gray-200 dark:text-gray-400 dark:border-gray-700">
        <ul class="w-full  flex flex-wrap justify-center -mb-px">
            <li class="me-1">
                <a onclick={nav_processes} class={if location == Route::Processes.to_path(){active_class}else {passive_class}} aria-current="page">{"Processes"}</a>
            </li>
            <li class="me-1">
                <a onclick={nav_file_systems} class={if location == Route::FileSystems.to_path(){active_class}else {passive_class}} aria-current="page">{"File Systems"}</a>
            </li>
        </ul>
    </div>
        }
}
