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
            let channel_name_with_sharp = Channel::Guild(chan.to_owned());
            if let Some(category_id) = chan.category_id {
                msg.channel_id
                    .say(
                        &ctx.http,
                        format!(
                            "channel_name: {} {}\nchannel_id: {}\ncategory_id:   {}\nchannel_kind: {}",
                            channel_name_with_sharp,
                            chan.name,
                            chan.id,
                            category_id,
                            chan.kind.name()
                        ),
                    )
                    .await?;
            } else {
                // カテゴリとカテゴリ外チャンネルを見分けることができていない
                msg.channel_id
                    .say(
                        &ctx.http,
                        format!(
                            "category_name: {}\ncategory_id: {}\n{}",
                            chan.name, chan.id, channel_name_with_sharp
                        ),
                    )
                    .await?;
            }
        }
    } else {
        msg.channel_id.say(&ctx.http, "ギルドではない").await?;
    }
    Ok(())
}
