use activitypub_federation::config::Data;
use hatsu_utils::AppData;
use maud::{html, Markup};

use crate::partials::layout;

pub async fn home(data: Data<AppData>) -> Markup {
    let title = data
        .env
        .hatsu_node_name
        .clone()
        .unwrap_or_else(|| String::from("Hatsu"));

    layout(
        &html! {
            @if let Some(description) = &data.env.hatsu_node_description {
                h2 class="md-typescale-title-large" style="margin-top: 0" { "About this instance" }
                p style="margin: 0" { (description) }
                br;
                md-divider {}
                h2 class="md-typescale-title-large" { "What is this?" }
            } @else {
                h2 class="md-typescale-title-large" style="margin-top: 0"  { "What is this?" }
            }
            p style="margin: 0" { r#"
                The web page you're reading right now is served by an instance of Hatsu,
                a self-hosted bridge that interacts with Fediverse on behalf of static site.
            "# }
            br;
            md-divider {}
            h2 class="md-typescale-title-large" { "Register an Account on " (title) }
            p style="margin: 0" {
                "New account registration is currently "
                // TODO: add registration status
                b { "closed" }
                "."
            }
        },
        &data,
    )
}
