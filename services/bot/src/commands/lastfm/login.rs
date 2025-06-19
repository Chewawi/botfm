use crate::core::structs::{Context, Error};
use lumi::serenity_prelude as serenity;

#[lumi::command(
    slash_command,
    description_localized("en-US", "Initiate Last.fm web authentication login.")
)]
pub async fn login(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();

    let lastfm_client = data.lastfm.clone();

    let user_id = ctx.author().id.get();
    let auth_url = lastfm_client.generate_auth_url(&user_id.to_string());

    let button = serenity::CreateComponent::Section(serenity::CreateSection::new(
        vec![serenity::CreateSectionComponent::TextDisplay(
            serenity::CreateTextDisplay::new("Click the button below to login to Last.fm."),
        )],
        serenity::CreateSectionAccessory::Button(
            serenity::CreateButton::new_link(auth_url).label("Login"),
        ),
    ));

    ctx.send(
        lumi::CreateReply::default()
            .components(vec![button])
            .reply(true)
            .flags(serenity::MessageFlags::EPHEMERAL | serenity::MessageFlags::IS_COMPONENTS_V2),
    )
    .await?;

    Ok(())
}
