#[allow(unused_imports)]
use crate::{
    api::types::AlbumLog,
    api::user_api::genre_album_api,
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
    pub genre: String,
}

#[function_component(GenrePage)]
pub fn genre_page(props: &DetailProps) -> Html {
    let genre = props.genre.clone();
    let current_page = use_state(|| 1u32);

    let chart_data = {
        let genre = genre.clone();
        let current_page = current_page.clone();
        use_async(async move {
            match genre_album_api(&genre, *current_page, 40).await {
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
      <p class="text-4xl font-semibold">{decode(&genre)}</p>
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
                      <th class="border"> {"Album"}</th>
                      <th class="border"> {"Artist"}</th>
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
                                  <td class="border px-2">
                                    <a class="break-all text-white hover:text-cyan-600" href={url}>{l.name}</a>
                                  </td>
                                  <td class="border">{l.artist}</td>
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
