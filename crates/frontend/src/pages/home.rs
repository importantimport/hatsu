use hatsu_utils::AppEnv;
use maud::{html, Markup};

use crate::partials::layout;

pub async fn home() -> Markup {
    layout(html! {
        h1 class="md-typescale-display-medium" { (AppEnv::info()) }
    })
}
