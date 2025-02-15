use crate::utils::image::get_image_color;
use crate::{Context, Error};
use database::model::lastfm::Lastfm;
use poise::command;
use poise::serenity_prelude as serenity;

#[command(slash_command, prefix_command, aliases("tp"))]
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
                .get_track_info(user_id, &track_name, &artist_name)
                .await
            {
                Ok(track_info) => {
                    let playcount = track_info.playcount;

                    let image_url = &track
                        .image
                        .iter()
                        .find(|image| image.size == "large")
                        .unwrap()
                        .text;

                    let image_color =
                        get_image_color(data.http_client.clone(), image_url.clone()).await?;

                    let embed = serenity::CreateEmbed::new()
                        .title(format!("Play Count for {}", track_name))
                        .description(format!(
                            "**Artist:** {}\n**Total Plays:** {}",
                            artist_name, playcount
                        ))
                        .color(serenity::Colour::from_rgb(
                            image_color[0],
                            image_color[0],
                            image_color[2],
                        ));

                    ctx.send(poise::CreateReply::default().embed(embed)).await?;
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
