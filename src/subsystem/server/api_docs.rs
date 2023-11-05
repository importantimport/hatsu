use aide::transform::TransformOpenApi;

pub fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Hatsu")
        .version(env!("CARGO_PKG_VERSION"))
        .summary("Self-hosted & Fully-automated ActivityPub Bridge for Static Sites.")
        // .description(include_str!("README.md"))
}
