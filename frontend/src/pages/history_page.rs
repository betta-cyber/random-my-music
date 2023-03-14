#[allow(unused_imports)]
use crate::{
    api::user_api::{album_log_api},
    api::types::AlbumLog,
    router::Route,
    store::{set_auth_user, set_page_loading, set_show_alert, Store},
    app::log, console_log,
    components::form_input::FormInput,
};
// use serde::{Deserialize, Serialize};
use yew::prelude::*;
// use yew_router::prelude::*;
use yewdux::prelude::*;


#[function_component(HistoryPage)]
pub fn history_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let user = store.auth_user.clone();
    let album_logs: yew::UseStateHandle<Vec<AlbumLog>> = use_state(|| vec![]);

    {
        let album_logs = album_logs.clone();
        let dispatch = dispatch.clone();
        use_effect_with_deps(move |_| {
            let dispatch = dispatch.clone();
            let album_logs = album_logs.clone();
            wasm_bindgen_futures::spawn_local(async move {
                set_page_loading(true, dispatch.clone());
                let response = album_log_api(1, 40).await;
                match response {
                    Ok(data) => {
                        album_logs.set(data);
                        set_page_loading(false, dispatch);
                    }
                    Err(e) => {
                        set_page_loading(false, dispatch.clone());
                        set_show_alert(e.to_string(), dispatch.clone());
                    }
                }
            });
        },
        ());
    }

    html! {
    <>
      <section class="bg-blue-800 min-h-screen py-20">
        <div class="max-w-4xl mx-auto rounded-md">
          <div class="w-10/12 mx-auto overflow-hidden shadow-2xl bg-gray-700 bg-opacity-50 rounded-2xl p-8 space-y-5 text-left">
            <p class="text-4xl font-semibold">{"History"}</p>
            if let Some(user) = user {
                <div class="mt-8">
                    <p class="mb-4">{format!("Name: {}", user.username)}</p>
                </div>
                <div>
                    <table class="table-auto border-spacing-px border">
                        <thead>
                          <tr>
                            <th class="border"> {"Cover"}</th>
                            <th class="border"> {"Album"}</th>
                            <th class="border"> {"Click Count"}</th>
                            <th class="border">{"Listen Count"}</th>
                          </tr>
                        </thead>
                        <tbody>
                            {
                                album_logs.iter().map(|l| {
                                    let l = l.clone();
                                    html! {
                                        <tr>
                                          <td class="border">
                                              <img src={l.cover} />
                                          </td>
                                          <td class="border">{l.album_name}</td>
                                          <td class="border text-center">{l.click_count}</td>
                                          <td class="border text-center">{l.listen_count}</td>
                                        </tr>
                                    }
                                }).collect::<Html>()
                            }
                        </tbody>
                    </table>
                </div>
            }else {
              <p class="mb-4">{"Loading..."}</p>
            }
          </div>
        </div>
      </section>
    </>
    }
}
