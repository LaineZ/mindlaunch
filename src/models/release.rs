use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub url: Option<String>,
    #[serde(rename = "assets_url")]
    pub assets_url: Option<String>,
    #[serde(rename = "upload_url")]
    pub upload_url: Option<String>,
    #[serde(rename = "html_url")]
    pub html_url: Option<String>,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: Option<String>,
    #[serde(rename = "tag_name")]
    pub tag_name: Option<String>,
    #[serde(rename = "target_commitish")]
    pub target_commitish: Option<String>,
    pub name: Option<String>,
    pub draft: bool,
    pub author: Author,
    pub prerelease: bool,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "published_at")]
    pub published_at: Option<String>,
    pub assets: Vec<Asset>,
    #[serde(rename = "tarball_url")]
    pub tarball_url: Option<String>,
    #[serde(rename = "zipball_url")]
    pub zipball_url: Option<String>,
    pub body: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub login: Option<String>,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: Option<String>,
    #[serde(rename = "avatar_url")]
    pub avatar_url: Option<String>,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "html_url")]
    pub html_url: Option<String>,
    #[serde(rename = "followers_url")]
    pub followers_url: Option<String>,
    #[serde(rename = "following_url")]
    pub following_url: Option<String>,
    #[serde(rename = "gists_url")]
    pub gists_url: Option<String>,
    #[serde(rename = "starred_url")]
    pub starred_url: Option<String>,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: Option<String>,
    #[serde(rename = "organizations_url")]
    pub organizations_url: Option<String>,
    #[serde(rename = "repos_url")]
    pub repos_url: Option<String>,
    #[serde(rename = "events_url")]
    pub events_url: Option<String>,
    #[serde(rename = "received_events_url")]
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub url: Option<String>,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: Option<String>,
    pub name: Option<String>,
    pub label: Option<Option<String>>,
    pub uploader: Uploader,
    #[serde(rename = "content_type")]
    pub content_type: Option<String>,
    pub state: Option<String>,
    pub size: i64,
    #[serde(rename = "download_count")]
    pub download_count: i64,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(rename = "browser_download_url")]
    pub browser_download_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uploader {
    pub login: Option<String>,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: Option<String>,
    #[serde(rename = "avatar_url")]
    pub avatar_url: Option<String>,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "html_url")]
    pub html_url: Option<String>,
    #[serde(rename = "followers_url")]
    pub followers_url: Option<String>,
    #[serde(rename = "following_url")]
    pub following_url: Option<String>,
    #[serde(rename = "gists_url")]
    pub gists_url: Option<String>,
    #[serde(rename = "starred_url")]
    pub starred_url: Option<String>,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: Option<String>,
    #[serde(rename = "organizations_url")]
    pub organizations_url: Option<String>,
    #[serde(rename = "repos_url")]
    pub repos_url: Option<String>,
    #[serde(rename = "events_url")]
    pub events_url: Option<String>,
    #[serde(rename = "received_events_url")]
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}
