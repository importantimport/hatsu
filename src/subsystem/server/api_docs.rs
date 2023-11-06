use aide::{
    openapi::{ApiKeyLocation, SecurityScheme},
    transform::TransformOpenApi
};

pub fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Hatsu")
        .version(env!("CARGO_PKG_VERSION"))
        .summary("Self-hosted & Fully-automated ActivityPub Bridge for Static Sites.")
        .description(include_str!("../../../README.md"))
        // .tag(Tag {
        //     name: "Hatsu API".into(),
        //     ..Default::default()
        // })
        .security_scheme(
            "Hatsu Access Token",
            SecurityScheme::ApiKey {
                location: ApiKeyLocation::Query,
                name: "token".to_string(),
                description: Some("env HATSU_ACCESS_TOKEN".to_string()),
                extensions: Default::default()
            }
        )
}
