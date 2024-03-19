use activitypub_federation::kinds::{link::MentionType, object::ImageType};
use hatsu_apub::{
    links::{Emoji, EmojiIcon, Mention, Tag},
    objects::Note,
    tests::test_asset,
};
use hatsu_utils::AppError;
use url::Url;

#[test]
fn test_parse_notes() -> Result<(), AppError> {
    // Akkoma
    let akkoma_note = test_asset::<Note>("assets/akkoma/objects/note.json")?;
    assert_eq!(akkoma_note.inner().clone().tag, vec![
        Tag::Mention(Mention {
            href: Url::parse(
                "https://hatsu-nightly-debug.hyp3r.link/users/kwaa-blog-next.deno.dev"
            )?,
            name: String::from("@kwaa-blog-next.deno.dev@hatsu-nightly-debug.hyp3r.link"),
            kind: MentionType::Mention
        }),
        Tag::Emoji(Emoji {
            icon: EmojiIcon {
                media_type: None,
                kind: ImageType::Image,
                url: Url::parse("https://social.qunn.eu/emoji/mergans_cats/acat_chew.webp")?,
            },
            id: Url::parse("https://social.qunn.eu/emoji/mergans_cats/acat_chew.webp")?,
            name: String::from(":acat_chew:"),
            kind: Default::default(),
            updated: Some(String::from("1970-01-01T00:00:00Z")),
        })
    ]);

    // GoToSocial
    test_asset::<Note>("assets/gotosocial/objects/note.json")?;
    test_asset::<Note>("assets/gotosocial/objects/note_without_tag.json")?;

    Ok(())
}
