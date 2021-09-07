use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "チャンネル一覧を取得"]
async fn all_channels(ctx: &Context, msg: &Message) -> CommandResult {
    // チャンネル一覧を取得したい
    // 順番は
    // text -> voice
    // カテゴリごとにグルーピングしたい
    // カテゴリはカテゴリIDごとに取得する
    // カテゴリIDごとにソート
    // カテゴリはないこともある
    // ないやつを優先する

    if let Some(guild_id) = msg.guild_id {
        let channels = ctx.http.get_channels(*guild_id.as_u64()).await?;
        for chan in channels {
            if let Some(category_id) = chan.category_id {
                // let category = category_id.
                let channel_name_with_sharp = Channel::Guild(chan.to_owned());
                msg.channel_id
                    .say(
                        &ctx.http,
                        format!(
                            "channel_name: {} {}\ncategory_id:   {}\nchannel_kind: {}",
                            channel_name_with_sharp,
                            chan.name,
                            category_id,
                            chan.kind.name()
                        ),
                    )
                    .await?;
            } else {
                // カテゴリ
                msg.channel_id
                    .say(&ctx.http, format!("{}", chan.name,))
                    .await?;
            }
        }
    } else {
        // なんかバグってる
        msg.channel_id.say(&ctx.http, "ギルドではない").await?;
    }
    Ok(())
}
