use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_interval};

#[derive(Serialize)]
struct Args {
    id: usize,
}

mod get_args {
    use serde::Serialize;
    #[derive(Serialize)]
    pub struct Args {
        pub name: Option<String>,
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}
#[derive(Serialize, Deserialize, PartialEq)]
struct ProccessInfo {
    id: String,
    name: String,
    cpu: String,
    memory: String,
    disk_read: String,
    disk_write: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let processes = use_state_eq(|| Vec::<ProccessInfo>::new());
    let process_id = use_state(|| None::<String>);
    let search_filter = use_state(|| None::<String>);

    let kill_process = use_async({
        let process_id = process_id.clone();
        async move {
            if let Some(id) = (*process_id).clone() {
                invoke(
                    "kill_process",
                    to_value(&Args {
                        id: id.parse::<usize>().unwrap_or(0),
                    })
                    .unwrap(),
                )
                .await;
            }
            Ok::<(), ()>(())
        }
    });
    let on_kill_process = Callback::from({
        let kill_process = kill_process.clone();
        move |e: SubmitEvent| {
            e.prevent_default();
            kill_process.run();
        }
    });

    let get_process = use_async({
        let processes = processes.clone();
        let search_filter = search_filter.clone();
        async move {
            let result = invoke(
                "process_info",
                to_value(&get_args::Args {
                    name: (*search_filter).clone(),
                })
                .unwrap(),
            )
            .await;
            if let Ok(pr) = from_value::<Vec<ProccessInfo>>(result) {
                processes.set(pr);
            }
            Ok::<(), ()>(())
        }
    });

    let search_input = use_node_ref();

    let on_search = Callback::from({
        let search_input = search_input.clone();
        let search_filter = search_filter.clone();
        move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(input) = search_input.cast::<HtmlInputElement>() {
                search_filter.set(Some(input.value()));
            }
        }
    });

    {
        let get_process = get_process.clone();
        use_interval(
            move || {
                get_process.run();
            },
            1000,
        )
    }
    html! {

        <>
                                         <div class="p-4 bg-white block sm:flex items-center justify-between border-b border-gray-200  dark:bg-gray-800 dark:border-gray-700">
                                     <div class="w-full mb-1">
                                             <div class="items-center justify-between block sm:flex md:divide-x md:divide-gray-100 dark:divide-gray-700">
                                             <div class="flex items-center mb-4 sm:mb-0">
                                                 <form onsubmit={on_search} class="sm:pr-3" >
                                                     <label for="products-search" class="sr-only">{"Search"}</label>
                                                     <div class="relative w-48 mt-1 sm:w-64 xl:w-96">
                                                         <input ref={search_input} type="text" name="email" id="products-search" class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Search by process name" />
                                                     </div>
                                                 </form>
                                                 <div class="flex items-center w-full sm:justify-end">
                                                     <div class="flex pl-2 space-x-1">
                                                     </div>
                                                 </div>
                                             </div>
                                            <form onsubmit={on_kill_process}>
                                             <button type="submit" id="createProductButton" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800" data-drawer-target="drawer-create-product-default" data-drawer-show="drawer-create-product-default" aria-controls="drawer-create-product-default" data-drawer-placement="right">
                                         { "end process" }
                                             </button>
                                            </form>
                                         </div>
                                     </div>
                                 </div>
                                     <div class="flex flex-col">
                                 <div class="overflow-x-auto">
                                     <div class="inline-block min-w-full align-middle">
                                         <div class="overflow-y-auto shadow h-[400px]">
                                             <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-600">
                                                 <thead class="bg-gray-100 dark:bg-gray-700 sticky top-0">
                                                     <tr>
                                                         <th scope="col" class="py-1 px-2 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                     {"Name"}
                                                         </th>
                                                         <th scope="col" class="py-1 px-2 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                     {"ID"}
                                                         </th>
                                                         <th scope="col" class="py-1 px-2 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                     {"CPU"}
                                                         </th>
                                                         <th scope="col" class="py-1 px-2 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                     {"Memory"}
                                                         </th>
                                                         <th scope="col" class="py-1 px-2 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                     {"Disk Read"}
                                                         </th>
                                                         <th scope="col" class="py-1 px-2 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                     {"Disk Write"}
                                                         </th>
                                                     </tr>
                                                 </thead>
                                                 <tbody class="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
                                                     {for (*processes).iter().map(|process| html!{
                                                 <tr
                                                    onclick={
                                                         Callback::from({
                                                            let process_id= process_id.clone();
                                                            let id = process.id.clone();
                                                            move |_| process_id.set(Some(id.clone()))
                                                         })
                                                     }
                                                         class={format!("cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700 {}",
                                        if let Some(proc_id) = (*process_id).clone() { if proc_id == process.id { "bg-gray-100 dark:bg-gray-700" }else{ "" }}else{""})}>
                                                     <td class="cursor-pointer px-2 py-1 text-base font-medium text-gray-800 whitespace-nowrap dark:text-gray-300">{process.name.clone()}</td>
                                                     <td class="cursor-pointer px-2 py-1 text-base font-medium text-gray-800 whitespace-nowrap dark:text-gray-300">{process.id.clone()}</td>
                                                     <td class="cursor-pointer px-2 py-1 text-base font-medium text-gray-800 whitespace-nowrap dark:text-gray-300">{process.cpu.clone()}</td>
                                                     <td class="cursor-pointer px-2 py-1 text-base font-medium text-gray-800 whitespace-nowrap dark:text-gray-300">{process.memory.clone()}</td>
                                                     <td class="cursor-pointer px-2 py-1 text-base font-medium text-gray-800 whitespace-nowrap dark:text-gray-300">{process.disk_read.clone()}</td>
                                                     <td class="cursor-pointer px-2 py-1 text-base font-medium text-gray-800 whitespace-nowrap dark:text-gray-300">{process.disk_write.clone()}</td>
                                                 </tr>
                                                     })}
                                                 </tbody>
                                             </table>
                                         </div>
                                     </div>
                                 </div>
                             </div>
                             <div class="sticky bottom-0 right-0 items-center w-full p-4 bg-gray-100 border-t border-gray-200 sm:flex sm:justify-between dark:bg-gray-800 dark:border-gray-700">
                          <div class="flex items-center mb-4 sm:mb-0">
                         </div>
                         </div>
    </>
    }
}
