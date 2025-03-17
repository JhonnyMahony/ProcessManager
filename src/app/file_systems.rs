use serde::Deserialize;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::JsValue;
use yew::{function_component, html, use_effect, use_effect_with, use_state, Html};
use yew_hooks::use_async;

use crate::{app::invoke, components::navbar::NavBar};

#[derive(Deserialize)]
struct DiskInfo {
    device: String,
    directory: String,
    r#type: String,
    total: String,
    available: String,
    used: String,
}

#[function_component(FileSystems)]
pub fn file_systems() -> Html {
    let file_systems_state = use_state(|| Vec::new());
    let get_file_systems = use_async({
        let file_systems_state = file_systems_state.clone();

        async move {
            let result = invoke("file_systems", JsValue::NULL).await;
            if let Ok(get_file_sys) = from_value::<Vec<DiskInfo>>(result) {
                file_systems_state.set(get_file_sys)
            }

            Ok::<(), ()>(())
        }
    });

    use_effect_with((), move |_| get_file_systems.run());
    html! {
    <>
        <NavBar />
                                         <div class="flex flex-col">
                                 <div class="overflow-x-auto">
                                     <div class="inline-block min-w-full align-middle">
                                         <div class="overflow-y-auto shadow h-screen">
                                             <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-600">
                                                 <thead class="bg-gray-100 dark:bg-gray-700 sticky top-0">
                                                     <tr>
                                                         <th scope="col" class="py-1 px-2 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                     {"Device"}
                                                         </th>
                                                         <th scope="col" class="py-1 px-2 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                     {"Directory"}
                                                         </th>
                                                         <th scope="col" class="py-1 px-2 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                     {"Type"}
                                                         </th>
                                                         <th scope="col" class="py-1 px-2 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                     {"Total"}
                                                         </th>
                                                         <th scope="col" class="py-1 px-2 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                     {"Available"}
                                                         </th>
                                                         <th scope="col" class="py-1 px-2 text-xs font-medium text-left text-gray-500 uppercase dark:text-gray-400">
                                     {"Used"}
                                                         </th>
                                                     </tr>
                                                 </thead>
                                                 <tbody class="bg-white divide-y divide-gray-200 dark:bg-gray-800 dark:divide-gray-700">
                                                     {for (*file_systems_state).iter().map(|file_system| html!{
                                                 <tr
                                                         class="cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700 bg-gray-100 dark:bg-gray-700">
                                                     <td class="cursor-pointer px-2 py-1 text-base font-medium text-gray-800 whitespace-nowrap dark:text-gray-300">{file_system.device.clone()}</td>
                                                     <td class="cursor-pointer px-2 py-1 text-base font-medium text-gray-800 whitespace-nowrap dark:text-gray-300">{file_system.directory.clone()}</td>
                                                     <td class="cursor-pointer px-2 py-1 text-base font-medium text-gray-800 whitespace-nowrap dark:text-gray-300">{file_system.r#type.clone()}</td>
                                                     <td class="cursor-pointer px-2 py-1 text-base font-medium text-gray-800 whitespace-nowrap dark:text-gray-300">{file_system.total.clone()}</td>
                                                     <td class="cursor-pointer px-2 py-1 text-base font-medium text-gray-800 whitespace-nowrap dark:text-gray-300">{file_system.available.clone()}</td>
                                                     <td class="cursor-pointer px-2 py-1 text-base font-medium text-gray-800 whitespace-nowrap dark:text-gray-300">{file_system.used.clone()}</td>
                                                 </tr>
                                                     })}
                                                 </tbody>
                                             </table>
                                         </div>
                                     </div>
                                 </div>
                             </div>
                             <div class="sticky bottom-0 right-0 items-center w-full p-4 bg-gray-100 border-t border-gray-200 sm:flex sm:justify-between dark:bg-gray-800 dark:border-gray-700">
                         </div>

    {"file systems"}
    </>
    }
}
