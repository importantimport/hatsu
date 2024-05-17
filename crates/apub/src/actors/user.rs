use activitypub_federation::{
    fetch::object_id::ObjectId,
    kinds::actor::{PersonType, ServiceType},
    protocol::{helpers::deserialize_one_or_many, public_key::PublicKey},
};
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

use crate::{
    actors::{ApubUser, UserAttachment, UserImage},
    links::Tag,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum UserType {
    /// `ActivityPub` Service (Bot User)
    ///
    /// Default type of local user
    ///
    /// <https://www.w3.org/ns/activitystreams#Service>
    ServiceType(ServiceType),
    /// `ActivityPub` Person (Normal User)
    ///
    /// Default type of outside user
    ///
    /// <https://www.w3.org/ns/activitystreams#Person>
    PersonType(PersonType),
}

/// Hatsu User
/// Supported: Service, Person
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[schema(value_type = Url)]
    pub id: ObjectId<ApubUser>,
    #[schema(value_type = String)]
    #[serde(rename = "type")]
    pub kind: UserType,
    /// feed title. example: `Example Domain`
    pub name: String,
    /// domain. example: `example.com`
    pub preferred_username: String,
    /// feed description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// feed icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<UserImage>,
    /// feed extension: `_hatsu.banner_image`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<UserImage>,
    /// auto-generated
    pub attachment: Vec<UserAttachment>,
    /// example: `https://hatsu.local/user/example.com/inbox`
    pub inbox: Url,
    /// exmaple: `https://hatsu.local/user/example.com/outbox`
    pub outbox: Url,
    /// example: `https://hatsu.local/user/example.com/followers`
    pub followers: Url,
    /// example: `https://hatsu.local/user/example.com/following`
    pub following: Url,
    /// user custom emoji, should be empty.
    #[serde(default, deserialize_with = "deserialize_one_or_many")]
    pub tag: Vec<Tag>,
    /// FEP-4adb
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// FEP-2c59
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webfinger: Option<String>,
    /// Public Key
    #[schema(value_type = PublicKeySchema)]
    pub public_key: PublicKey,
}

/// impl `ToSchema` for `PublicKey`
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeySchema {
    pub id: String,
    pub owner: Url,
    pub public_key_pem: String,
}
