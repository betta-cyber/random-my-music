
use yew::prelude::*;
use std::collections::HashMap;
// use yew_router::prelude::Link;
// use crate::router::Route;
use crate::console_log;
use crate::app::log;


#[derive(Debug, PartialEq)]
pub struct Link {
    pub media_link: String,
    pub media_class: String,
    pub title: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub media_data: HashMap<String, serde_json::Value>,
}


#[function_component]
pub fn MediaLink(props: &Props) -> Html {
    let media_data = props.media_data.clone();
    let mut data: Vec<Link> = vec![];

    for (k, v) in &media_data {
        let a: Option<Link> = match k.as_str() {
            "spotify" => {
                let mut spotify = "";
                for (kk, vv) in v.as_object().unwrap() {
                    match vv.get("default") {
                        Some(default) => {
                            if default.as_bool().unwrap() {
                                spotify = kk;
                                break
                            }
                        }
                        None => {}
                    }
                }
                let link = format!("https://open.spotify.com/album/{}", spotify);
                Some(Link {
                    media_link: link,
                    title: "Spotify".to_string(),
                    media_class: "ui_media_link_btn ui_media_link_btn_spotify".to_string()
                })
            },
            "applemusic" => {
                let mut applemusic = "";
                let mut album = None;
                for (kk, vv) in v.as_object().unwrap() {
                    let a: Option<String> = match vv.get("default") {
                        Some(default) => {
                            if default.as_bool().unwrap_or(false) {
                                applemusic = kk;
                                Some(vv.get("album").unwrap().as_str().unwrap().to_string())
                            } else {
                                None
                            }
                        },
                        None => {
                            None
                        }
                    };
                    if let Some(a) = a {
                        album = Some(a);
                    }
                };
                match album {
                    Some(album) => {
                        let link = format!("https://geo.music.apple.com/gb/album/{}/{}", album.as_str(), applemusic);
                        Some(Link {
                            media_link: link,
                            title: "Apple Music".to_string(),
                            media_class: "ui_media_link_btn ui_media_link_btn_applemusic".to_string()
                        })
                    }
                    None => {None}
                }
            }
            "bandcamp" => {
                let mut url = "";
                for (kk, vv) in v.as_object().unwrap() {
                    match vv.get("default") {
                        Some(default) => {
                            if default.as_bool().unwrap() {
                                url = kk;
                            }
                        },
                        None => {}
                    };
                };
                let link = format!("https://{}", url);
                Some(Link {
                    media_link: link,
                    title: "Bandcamp".to_string(),
                    media_class: "ui_media_link_btn ui_media_link_btn_bandcamp".to_string()
                })
            }
            "youtube" => {
                let mut url = "";
                for (kk, vv) in v.as_object().unwrap() {
                    match vv.get("default") {
                        Some(default) => {
                            if default.as_bool().unwrap() {
                                url = kk;
                            }
                        },
                        None => {}
                    };
                };
                let link = format!("https://www.youtube.com/watch?v={}", url);
                Some(Link {
                    media_link: link,
                    title: "YouTube".to_string(),
                    media_class: "ui_media_link_btn ui_media_link_btn_youtube".to_string()
                })

            }
            // https://www.youtube.com/watch?v=-Kqf6vtQmPQ
            &_ => { None }
        };
        console_log!("{:#?}", a);
        if a != None {
            data.push(a.unwrap());
        }
    };

    html! {
        data.iter().map(|link| {
            let link = link.clone();
            html!{
                <a target="_blank" rel="noopener nofollow" title={link.title.clone()} class={link.media_class.clone()} href={link.media_link.clone()}></a>
            }
        }).collect::<Html>()
    }
}

// <a target="_blank" rel="noopener nofollow" title="Apple Music" aria-label="Open in Apple Music" class="ui_media_link_btn ui_media_link_btn_applemusic" href="https://geo.music.apple.com/us/album/terrifyer/68290017"></a>
                        // <a target="_blank" rel="noopener nofollow" title="Bandcamp" aria-label="Open in Bandcamp" class="ui_media_link_btn ui_media_link_btn_bandcamp" href="https://pigdestroyer.bandcamp.com/album/terrifyer"></a>
                        // <a target="_blank" rel="noopener nofollow" title="Spotify" aria-label="Open in Spotify" class="ui_media_link_btn ui_media_link_btn_spotify" href="https://open.spotify.com/album/6x95DGZhx18gJuxRlbwZso"></a>
                        // <a target="_blank" rel="noopener nofollow" title="YouTube" aria-label="Open in YouTube" class="ui_media_link_btn ui_media_link_btn_youtube" href="https://www.youtube.com/watch?v=-Kqf6vtQmPQ"></a>
