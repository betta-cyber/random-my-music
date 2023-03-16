use yew::prelude::*;

const ITEMS_PER_PAGE: u32 = 40;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub total_count: u32,
    pub current_page: u32,
    pub callback: Callback<u32>,
}

/// Pagination component
#[function_component(ListPagination)]
pub fn list_pagination(props: &Props) -> Html {
    if props.total_count < ITEMS_PER_PAGE {
        return html! {};
    }

    // Calculate page numbers
    let max_page = (props.total_count as f32 / 10.0).ceil() as u32;
    let mut pages: Vec<u32> = vec![];
    for page in 0..max_page {
        pages.push(page);
    }

    html! {
        <nav>
            <ul class="pagination">
            {for pages.iter().map(|page| {
                let is_current = page == &props.current_page;
                let page_item_class = if is_current {
                    "page-item active"
                } else {
                    "page-item"
                };
                let page = *page;
                let callback = props.callback.clone();
                let onclick = Callback::from(move |ev: MouseEvent| {
                    ev.prevent_default();
                    callback.emit(page)
                });
                html! {
                    <li
                        class={page_item_class}
                        onclick={onclick}>
                        <a class="page-link" href="">{page + 1}</a>
                    </li>
                }
            })}
            </ul>
        </nav>
    }
}
