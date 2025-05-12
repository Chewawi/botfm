use crate::core::structs::{Context, Error};
use database::model::lastfm::Lastfm;
use lumi::serenity_prelude as serenity;
use database::model::colors::Colors;

#[lumi::command(slash_command, prefix_command, aliases("tp"))]
pub async fn track_plays(ctx: Context<'_>) -> Result<(), Error> {
    let _ = ctx.defer_or_broadcast().await;

    let data = ctx.data();
    let lastfm_client = data.lastfm.clone();
    let user_id = ctx.author().id.get();

    let db_clone = data.db.clone();
    let db = Lastfm::get(&db_clone as &database::DatabaseHandler, user_id);

    let lastfm_user = match db.await {
        Ok(user) => user,
        Err(_) => {
            ctx.say("You haven't linked your Last.fm account yet. Use the `/login` command.")
                .await?;
            return Ok(());
        }
    };

    match lastfm_client
        .get_current_track(lastfm_user.expect("Missing Last.fm user"))
        .await
    {
        Ok(track_opt) => {
            let track = track_opt.unwrap();
            let track_name = track.name.clone();
            let artist_name = track.artist.text.clone();

            match lastfm_client
                .get_track_info(user_id, &artist_name, &track_name)
                .await
            {
                Ok(track_info) => {
                    let playcount = track_info.playcount;

                    let small_url = track
                        .image
                        .iter()
                        .find(|img| img.size == "small")
                        .map(|img| &img.text)
                        .ok_or_else(|| Error::from("Missing small image URL"))?;

                    let image_color = Colors::get(&data.db.cache, data.http_client.clone(), small_url)
                        .await?
                        .unwrap_or(vec![255,255,255]);

                    let embed = serenity::CreateEmbed::new()
                        .description(format!(
                            "**{}** plays for **{}** by **{}**",
                            playcount, track.name, track.artist.text
                        ))
                        .color(serenity::Colour::from_rgb(
                            image_color[0],
                            image_color[1],
                            image_color[2],
                        ));

                    ctx.send(lumi::CreateReply::default().embed(embed)).await?;
                }
                Err(err) => {
                    ctx.say(format!("Error fetching track info: {}", err))
                        .await?;
                }
            }
        }
        Err(err) => {
            ctx.say(format!("Error fetching current track: {}", err))
                .await?;
        }
    }

    Ok(())
}