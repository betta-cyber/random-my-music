use yew::prelude::*;
use yew_router::prelude::*;
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
                        <span class="col-span-1 break-all m-2 float-left text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">{"Released"}</span>
                        <span class="col-span-2 break-all m-2 float-left text-2xl tracking-tight text-gray-900 sm:text-3xl">{&detail.released}</span>
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
