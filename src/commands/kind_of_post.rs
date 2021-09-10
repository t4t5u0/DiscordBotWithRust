use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "発言"]
async fn say(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "これは発言です").await?;
    Ok(())
}

#[command]
#[description = "リプライ"]
async fn reply(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "これはリプライです").await?;
    Ok(())
}

#[command]
#[description = "リプライ"]
async fn reply_m(ctx: &Context, msg: &Message) -> CommandResult {
    // msg.reply_mentionは普通にメンションされるだけ
    msg.reply_ping(&ctx.http, "これはメンション付きリプライです")
        .await?;
    Ok(())
}

#[command]
#[description = "メンション"]
async fn mention(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            format!("{} これはメンションです。", msg.author.mention()),
        )
        .await?;
    Ok(())
}

#[command]
#[description = "リアクションをつける"]
async fn react(ctx: &Context, msg: &Message) -> CommandResult {
    // どっちでもいける
    msg.react(&ctx.http, '👍').await?;
    msg.channel_id
        .create_reaction(&ctx.http, msg.id, '👎')
        .await?;
    Ok(())
}
