use yew::prelude::*;

#[function_component(AboutPage)]
pub fn about() -> Html {
    html! {
        <div class="about">
            <span class="break-word">
            {"也许我们不能通过一本书的封面来判断书的质量，但当我们谈论到音乐时，专辑封面有时候会扮演一个重要的角色，他们是专辑的视觉表达。自从黑胶唱片诞生以来，专辑封面经过长时间的发展，已经从简单的个性表达逐步演变成了复杂的艺术作品。我喜欢看它们，也喜欢谈论它们，它们为音乐赋予了视觉表现力。一张专辑的封面背后往往有许多关于音乐的有趣故事，所以我设计创作了这个网站。致力于从封面视觉出发，基于音乐流派，让用户探索发现更多有趣的音乐。"}
            </span>
        </div>
    }
}
