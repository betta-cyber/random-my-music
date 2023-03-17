#[allow(unused_imports)]
use crate::{
    api::types::AlbumLog,
    api::user_api::artist_album_api,
    app::log,
    components::form_input::FormInput,
    components::list_pagination::ListPagination,
    console_log,
    router::Route,
    store::{set_auth_user, set_page_loading, set_show_alert, Store},
};
// use serde::{Deserialize, Serialize};
use url_escape::decode;
use yew::prelude::*;
use yew_hooks::use_async;
// use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DetailProps {
    pub artist: String,
}

#[function_component(ArtistPage)]
pub fn artist_page(props: &DetailProps) -> Html {
    let artist = props.artist.clone();
    let current_page = use_state(|| 1u32);

    let chart_data = {
        let artist = artist.clone();
        let current_page = current_page.clone();
        use_async(async move {
            match artist_album_api(&artist, *current_page, 40).await {
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
        let chart_data = chart_data.clone();
        use_effect_with_deps(
            move |_| {
                chart_data.run();
                || ()
            },
            *current_page,
        );
    }

    let callback = {
        let current_page = current_page;
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
      <p class="text-4xl font-semibold">{decode(&artist)}</p>
      if let Some(data) = chart_data.data.clone() {
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
                      <th class="border px-3"> {"Album"}</th>
                      <th class="border px-3"> {"Artist"}</th>
                      <th class="border">{"Rate"}</th>
                    </tr>
                  </thead>
                  <tbody>
                    {
                        data.res.iter().map(|l| {
                            let l = l.clone();
                            let url = format!("/album/{}", l.id);
                            html! {
                                <tr>
                                  <td class="border w-16">
                                      <img class="h-16 w-16" src={l.cover} />
                                  </td>
                                  <td class="border px-3">
                                    <a class="break-all text-white hover:text-cyan-600" href={url}>{l.name}</a>
                                  </td>
                                  <td class="border px-3">{l.artist}</td>
                                  <td class="border text-center">{l.rate}</td>
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
