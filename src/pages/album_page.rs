use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use crate::components::media_link::MediaLink;
use crate::api::types::AlbumDetail;
use crate::api::user_api::album_detail_api;
use crate::store::{Store, set_page_loading};


#[derive(Properties, PartialEq)]
pub struct DetailProps {
    pub album_id: String,
}

#[function_component(AlbumPage)]
pub fn album(props: &DetailProps) -> Html {
    let (_, dispatch) = use_store::<Store>();
    let _navigator = use_navigator().unwrap();
    let album_id = props.album_id.clone();

    let detail = use_state(|| AlbumDetail::default());
    {
        let store_dispatch = dispatch.clone();
        let detail = detail.clone();
        use_effect_with_deps(move |_| {
            let detail = detail.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let album_id = album_id.clone();
                let dispatch = store_dispatch.clone();
                set_page_loading(true, dispatch.clone());
                match album_detail_api(&album_id).await {
                    Ok(data) => {
                        detail.set(data);
                        set_page_loading(false, dispatch);
                    }
                    Err(_) => {}
                }
            });
            || ()
        }, ());
    }

    let mut genre_pri_text = "".to_string();
    let mut genre_sec_text = "".to_string();
    for g in &detail.genres {
        if g.genre_type == "pri" {
            genre_pri_text += &g.genre
        } else {
            genre_sec_text += &g.genre
        }
    }
    // let genre_text = format!("{}\r\n{}", genre_pri_text, genre_sec_text);

    html! {
        <div class="block bg-blue-800 w-screen md:h-full h-full lg:h-screen">
            <div class="md:absolute lg:inset-y-0 lg:left-0 lg:w-2/6 md:inset-x-0 md:top-0 md:w-full" id="container_left">
                <div class="lg:m-4">
                    <img class="w-full" src={detail.cover.clone()} />
                    <div class="media-link object-center flex">
                        <MediaLink media_data={detail.media_url.clone()}></MediaLink>
                    </div>
                </div>
            </div>
            <div class="md:absolute text-left md:bottom-0 md:w-full lg:right-0 lg:inset-y-0 lg:w-4/6" id="container_right">
                <div class="width-full album_info_outer m-4">
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8">
                        <h3 class="col-span-3 m-2 text-left float-left text-3xl font-bold tracking-tight text-gray-900 sm:text-3xl">{&detail.name}</h3>
                    </div>
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8">
                        <span class="col-span-1 break-all m-2 float-left text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">{"Artist"}</span>
                        <span class="col-span-2 break-all m-2 float-left text-2xl tracking-tight text-gray-900 sm:text-3xl">{&detail.artist}</span>
                    </div>
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8">
                        <span class="col-span-1 break-all m-2 float-left text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">{"Rate"}</span>
                        <span class="col-span-2 break-all m-2 float-left text-2xl tracking-tight text-gray-900 sm:text-3xl">{&detail.rate}</span>
                    </div>
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8">
                        <span class="col-span-1 break-all m-2 float-left text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">{"Genres"}</span>
                        <span class="col-span-2 break-all m-2 float-left text-2xl tracking-tight text-gray-900 sm:text-3xl">
                        {
                            detail.genres.clone().into_iter().map(|g| {
                                if g.genre_type == "pri" {
                                    html!{ <><a class="mr-2 tracking-tight text-gray-900 font-normal">{&g.genre}</a><br/></>}
                                } else {
                                    html!{<a class="mr-2 text-xl tracking-tight text-gray-900 font-normal">{&g.genre}</a>}
                                }
                            }).collect::<Html>()
                        }
                        </span>
                    </div>
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8">
                        <span class="col-span-1 break-all m-2 float-left text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">{"Released"}</span>
                        <span class="col-span-2 break-all m-2 float-left text-2xl tracking-tight text-gray-900 sm:text-3xl">{&detail.released}</span>
                    </div>
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8">
                        <span class="col-span-1 break-all m-2 float-left text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">{"Language"}</span>
                        <span class="col-span-2 break-all m-2 float-left text-2xl tracking-tight text-gray-900 sm:text-3xl">{&detail.language}</span>
                    </div>
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8 ">
                        <span class="col-span-1 break-all m-2 float-left text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">{"Descriptors"}</span>
                        <span class="col-span-2 break-all m-2 float-left text-2xl tracking-tight text-gray-900 sm:text-3xl">{&detail.descriptors}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
