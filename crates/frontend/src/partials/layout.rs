use activitypub_federation::config::Data;
use hatsu_utils::{AppData, AppEnv};
use maud::{DOCTYPE, Markup, PreEscaped, html};
use serde_json::json;

static CSS_CUSTOM: &str = include_str!("../../assets/custom.css");
static CSS_DARK: &str = include_str!("../../assets/dark.css");
static CSS_LIGHT: &str = include_str!("../../assets/light.css");

fn material_web() -> Markup {
    html! {
        link href="https://fonts.googleapis.com/css2?family=Roboto:wght@400;500;700&display=swap" rel="stylesheet";
        style {
            (CSS_DARK)
            "\n"
            (CSS_LIGHT)
            "\n"
            (CSS_CUSTOM)
        }
        script type="importmap" {
            (PreEscaped(json!({
                "imports": {
                  "@material/web/": "https://esm.run/@material/web@2.2.0/"
                }
            }).to_string()))
        }
        script type="module" {
            r#"
import '@material/web/divider/divider.js';

import '@material/web/labs/card/filled-card.js';

import { styles as typescaleStyles } from '@material/web/typography/md-typescale-styles.js';
document.adoptedStyleSheets.push(typescaleStyles.styleSheet);
            "#
        }
    }
}

pub fn layout(body: &Markup, data: &Data<AppData>) -> Markup {
    let title = data
        .env
        .hatsu_node_name
        .clone()
        .unwrap_or_else(|| String::from("Hatsu"));

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="icon" href="/favicon.ico" sizes="48x48";
                link rel="icon" href="/favicon.svg" sizes="any" type="image/svg+xml";
                title { (&title) }
                (material_web())
            }
            body class="md-typescale-body-large" {
                header {
                    h2 class="md-typescale-title-large" style="margin: 1rem" { (title) }
                }
                main {
                    md-filled-card style="padding: 1rem" {
                        (body)
                    }
                }
                footer {
                    div style="padding: 1rem; display: flex; gap: 0.5rem" {
                        img src="favicon.svg" height="24" width="24";
                        a href="https://github.com/importantimport/hatsu" target="_blank" rel="noopener"  {
                            (AppEnv::info())
                        }
                    }
                }
            }
        }
    }
}
