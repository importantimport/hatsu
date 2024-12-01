use maud::{html, Markup, PreEscaped, DOCTYPE};
use serde_json::json;

static CSS_CUSTOM: &str = include_str!("../../assets/custom.css");
static CSS_DARK: &str = include_str!("../../assets/dark.css");
static CSS_LIGHT: &str = include_str!("../../assets/light.css");
static CSS_TYPESCALE: &str = include_str!("../../assets/typescale.css");

fn material_web() -> Markup {
    html! {
        link href="https://fonts.googleapis.com/css2?family=Roboto:wght@400;500;700&display=swap" rel="stylesheet";
        style {
            (CSS_DARK)
            "\n"
            (CSS_LIGHT)
            "\n"
            (CSS_TYPESCALE)
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
import '@material/web/all.js';
import { styles as typescaleStyles } from '@material/web/typography/md-typescale-styles.js';
document.adoptedStyleSheets.push(typescaleStyles.styleSheet);
            "#
        }
    }
}

pub fn layout(body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="icon" href="/favicon.ico" sizes="48x48";
                link rel="icon" href="/favicon.svg" sizes="any" type="image/svg+xml";
                title { "Hatsu" }
                (material_web())
            }
            body {
                (body)
            }
        }
    }
}
