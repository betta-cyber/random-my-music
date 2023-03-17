#[allow(unused_imports)]
use crate::{
    api::types::AlbumLog,
    api::user_api::album_log_api,
    app::log,
    components::form_input::FormInput,
    components::list_pagination::ListPagination,
    console_log,
    router::Route,
    store::{set_auth_user, set_page_loading, set_show_alert, Store},
};
// use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_hooks::use_async;
// use yewdux::prelude::*;

#[function_component(HistoryPage)]
pub fn history_page() -> Html {
    // let (store, dispatch) = use_store::<Store>();
    // let user = store.auth_user.clone();
    let current_page = use_state(|| 1u32);

    let album_logs = {
        let current_page = current_page.clone();
        use_async(async move {
            match album_log_api(*current_page, 40).await {
                Ok(data) => Ok(data),
                Err(e) => Err(e),
            }
        })
    };

    {
        let current_page = current_page.clone();
        use_effect_with_deps(
            move |_| {
                // Reset to first page
                current_page.set(1);
                || ()
            },
            (),
        );
    }

    {
        let album_logs = album_logs.clone();
        use_effect_with_deps(
            move |_| {
                album_logs.run();
                || ()
            },
            *current_page,
        );
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
      if let Some(data) = album_logs.data.clone() {
          <div>
              <ListPagination
                total_count={data.total}
                current_page={data.page}
                callback={callback.clone()}
              />
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
                          data.res.iter().map(|l| {
                              let l = l.clone();
                              let url = format!("/album/{}", l.album_id);
                              html! {
                                  <tr>
                                    <td class="border w-16">
                                        <img class="h-16 w-16" src={l.cover} />
                                    </td>
                                    <td class="border px-2">
                                        <a class="break-all text-white hover:text-cyan-600" href={url}>{l.album_name}</a>
                                    </td>
                                    <td class="border text-center">{l.click_count}</td>
                                    <td class="border text-center">{l.listen_count}</td>
                                  </tr>
                              }
                          }).collect::<Html>()
                      }
                  </tbody>
              </table>
              <ListPagination
                total_count={data.total}
                current_page={data.page}
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
