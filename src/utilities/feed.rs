use serde::{Deserialize, Serialize};

/// Excerpt from JSON Feed 1.1
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Feed {
    // 用户名
    title: String,
    // 用户描述
    description: Option<String>,
    // 用户头像
    icon: Option<String>,
    // 备用用户头像
    // favicon: String
    // 文章列表
    items: Vec<FeedItem>
}

/// Excerpt from JSON Feed 1.1
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FeedItem {
    // 文章链接
    id: String,
    // 备用文章链接
    url: String,
    // 文章标题
    title: Option<String>,
    // 文章描述
    summary: Option<String>,
    // 文章图像
    // image: Option<String>
}
