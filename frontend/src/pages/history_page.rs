#[allow(unused_imports)]
use crate::{
    api::user_api::{album_log_api},
    api::types::AlbumLog,
    router::Route,
    store::{set_auth_user, set_page_loading, set_show_alert, Store},
    app::log, console_log,
    components::form_input::FormInput,
    components::list_pagination::ListPagination,
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
    let current_page = use_state(|| 1u32);
    let total = use_state(|| 1u32);

    {
        let album_logs = album_logs.clone();
        let dispatch = dispatch.clone();
        let current_page = current_page.clone();
        let total = total.clone();
        use_effect_with_deps(move |_| {
            let dispatch = dispatch.clone();
            let current_page = current_page.clone();
            let album_logs = album_logs.clone();
            wasm_bindgen_futures::spawn_local(async move {
                set_page_loading(true, dispatch.clone());
                let response = album_log_api(*current_page, 40).await;
                match response {
                    Ok(data) => {
                        album_logs.set(data.res);
                        current_page.set(data.page);
                        total.set(data.total);
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

    let callback = {
        let current_page = current_page.clone();
        use_callback(
            move |page, _| {
                current_page.set(page);
            },
            (),
        )
    };

    html! {
    <>
    <div class="mx-auto overflow-hidden p-8 space-y-5 text-left">
      <p class="text-4xl font-semibold">{"History"}</p>
      if let Some(_) = user {
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
                                    <td class="border w-16">
                                        <img class="h-16 w-16" src={l.cover} />
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
              <ListPagination
                total_count={*total}
                current_page={*current_page}
                callback={callback}
              />
          </div>
      }else {
        <p class="mb-4">{"Loading..."}</p>
      }
    </div>
    </>
    }
}
