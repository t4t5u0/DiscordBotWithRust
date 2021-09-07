use serenity::prelude::*;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
#[description = "猫のように鳴く"]
async fn neko(ctx: &Context, msg: &Message) -> CommandResult {
    // msg.channel_id.say で、channel_id の channel にメッセージを投稿
    msg.channel_id
        .say(&ctx.http, format!("{} にゃーん", msg.author.mention()))
        .await?;
    // CommandResultはResultを継承している
    // `Result?` は正常な値の場合、Resultの中身を返し、エラーの場合は即座にreturnする演算子
    Ok(())
}
