use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::{Request, RequestCredentials};
use crate::components::media_link::MediaLink;
use crate::api::types::AlbumDetail;
use crate::api::user_api::album_detail_api;


#[derive(Properties, PartialEq)]
pub struct DetailProps {
    pub album_id: String,
}

#[function_component(AlbumPage)]
pub fn album(props: &DetailProps) -> Html {
    let _navigator = use_navigator().unwrap();
    let album_id = props.album_id.clone();
    let detail = use_state(|| AlbumDetail::default());
    {
        let detail = detail.clone();
        use_effect_with_deps(move |_| {
            let detail = detail.clone();
            // let album_id = album_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let album_id = album_id.clone();
                match album_detail_api(&album_id).await {
                    Ok(data) => {
                        detail.set(data);
                    }
                    Err(_) => {}
                }
            });
            || ()
        }, ());
    }

    html! {
        <div class="block bg-blue-800 w-auto h-screen">
            <div class="absolute inset-y-0 left-0 w-2/6" id="container_left">
                <div class="m-4">
                    <img class="w-full" src={detail.cover.clone()} />
                    <div class="media-link h-40 object-center flex">
                        <MediaLink media_data={detail.media_url.clone()}></MediaLink>
                    </div>
                </div>
            </div>
            <div class="text-left absolute inset-y-0 right-0 w-4/6" id="container_right">
                <div class="width-full album_info_outer m-4">
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8">
                        <h1 class="col-span-3 m-2 text-left float-left text-4xl font-bold tracking-tight text-gray-900 sm:text-4xl">{&detail.name}</h1>
                    </div>
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8">
                        <span class="col-span-1 m-2 float-left text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">{"Artist"}</span>
                        <span class="col-span-2 m-2 float-left text-2xl tracking-tight text-gray-900 sm:text-3xl">{&detail.artist}</span>
                    </div>
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8">
                        <span class="col-span-1 m-2 float-left text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">{"Released"}</span>
                        <span class="col-span-2 m-2 float-left text-2xl tracking-tight text-gray-900 sm:text-3xl">{&detail.released}</span>
                    </div>
                    <div class="grid grid-cols-3 lg:border-l lg:border-blue-600 lg:pl-8 ">
                        <span class="col-span-1 m-2 float-left text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">{"Descriptors"}</span>
                        <span class="col-span-2 break-all m-2 float-left text-2xl tracking-tight text-gray-900 sm:text-3xl">{&detail.descriptors}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
