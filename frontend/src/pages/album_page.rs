use crate::api::types::AlbumDetail;
use crate::api::user_api::album_detail_api;
use crate::components::media_link::MediaLink;
use crate::store::{set_page_loading, Store};
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
// use crate::{app::log, console_log};

#[derive(Properties, PartialEq)]
pub struct DetailProps {
    pub album_id: String,
}

#[function_component(AlbumPage)]
pub fn album(props: &DetailProps) -> Html {
    let (_, dispatch) = use_store::<Store>();
    let _navigator = use_navigator().unwrap();
    let album_id = props.album_id.clone();

    let detail = use_state(AlbumDetail::default);
    {
        let store_dispatch = dispatch;
        let detail = detail.clone();
        use_effect_with_deps(
            move |_| {
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
                        Err(_) => {
                            set_page_loading(false, dispatch);
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    let mut genre_pri_text = "".to_string();
    let mut genre_sec_text = "".to_string();
    let mut genres: Vec<String> = vec![];
    for g in &detail.genres {
        if g.genre_type == "pri" {
            genre_pri_text += &g.genre;
            genres.push(g.genre.clone());
        } else {
            genre_sec_text += &g.genre;
            genres.push(g.genre.clone());
        }
    }

    let onload = Callback::from(move |e: Event| {
        if let Some(img) = e.target_dyn_into::<HtmlElement>() {
            img.toggle_attribute("hidden").unwrap();
        }
    });

    let artist_url = format!("/artist/{}", &detail.artist);
    html! {
        <div class="block bg-blue-800 w-screen md:h-full h-full lg:h-screen">
            <div class="md:absolute lg:inset-y-0 lg:left-0 lg:w-2/6 md:inset-x-0 md:top-0 md:w-full" id="container_left">
                <div class="lg:m-4">
                    <i class="lazyload-img">
                        <img class="w-full" src={detail.cover.clone()} onload={onload} hidden=true />
                    </i>
                    <div class="media-link object-center flex">
                        <MediaLink media_data={detail.media_url.clone()}></MediaLink>
                    </div>
                </div>
            </div>
            <div class="md:absolute text-left md:bottom-0 md:w-full lg:right-0 lg:inset-y-0 lg:w-4/6" id="container_right">
                <div class="width-full album_info_outer p-4">
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8">
                        <h3 class="col-span-3 m-2 text-left float-left text-2xl font-bold tracking-tight text-white sm:text-4xl">{&detail.name}</h3>
                    </div>
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8">
                        <span class="col-span-1 break-all m-2 float-left text-xl font-bold tracking-tight text-white sm:text-3xl">{"Artist"}</span>
                        <span class="col-span-2 break-all m-2 float-left text-xl tracking-tight text-white sm:text-3xl">
                        {
                            html!{
                                <a class="mr-2 tracking-tight text-white font-normal" href={artist_url}>{&detail.artist}</a>
                            }
                        }
                        </span>
                    </div>
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8">
                        <span class="col-span-1 break-all m-2 float-left text-xl font-bold tracking-tight text-white sm:text-3xl">{"Rate"}</span>
                        <span class="col-span-2 break-all m-2 float-left text-xl tracking-tight text-white sm:text-3xl">{&detail.rate}</span>
                    </div>
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8">
                        <span class="col-span-1 break-all m-2 float-left text-xl font-bold tracking-tight text-white sm:text-3xl">{"Genres"}</span>
                        <span class="col-span-2 break-all m-2 float-left text-xl tracking-tight text-white sm:text-3xl">
                        {
                            detail.genres.clone().into_iter().map(|g| {
                                let url = format!("/genre/{}", &g.genre);
                                if g.genre_type == "pri" {
                                    html!{ <><a class="mr-2 text-xl tracking-tight text-white font-normal" href={url}>{&g.genre}</a><br/></>}
                                } else {
                                    html!{<a class="mr-2 text-lg tracking-tight text-white font-normal sm:text-xl" href={url}>{&g.genre}</a>}
                                }
                            }).collect::<Html>()
                        }
                        </span>
                    </div>
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8">
                        <span class="col-span-1 break-all m-2 float-left text-xl font-bold tracking-tight text-white sm:text-3xl">{"Released"}</span>
                        <span class="col-span-2 break-all m-2 float-left text-xl tracking-tight text-white sm:text-3xl">{&detail.released}</span>
                    </div>
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8">
                        <span class="col-span-1 break-all m-2 float-left text-xl font-bold tracking-tight text-white sm:text-3xl">{"Language"}</span>
                        <span class="col-span-2 break-all m-2 float-left text-xl tracking-tight text-white sm:text-3xl">{&detail.language}</span>
                    </div>
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8 ">
                        <span class="col-span-1 break-all m-2 float-left text-xl font-bold tracking-tight text-white sm:text-3xl">{"Descriptors"}</span>
                        <span class="col-span-2 break-all m-2 float-left text-lg tracking-tight text-white sm:text-xl">{&detail.descriptors}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
