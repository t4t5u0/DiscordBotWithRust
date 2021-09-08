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
    // treeコマンドみたいな感じで表示したい

    if let Some(guild_id) = msg.guild_id {
        // 全チャンネルを取得
        let tmp_channels = ctx.http.get_channels(*guild_id.as_u64()).await?;
        // イテレーターにしてVecに流し込む
        let channels = tmp_channels
            .iter()
            .map(|chan| MyChannel::new(chan.to_owned()))
            .collect::<Vec<MyChannel>>();
    } else {
        msg.channel_id.say(&ctx.http, "ギルドではない").await?;
    }
    Ok(())
}

struct MyChannel {
    channel_name: String,
    channel_id: ChannelId,
    category_id: Option<ChannelId>,
}

impl MyChannel {
    fn new(channel: GuildChannel) -> Self {
        let channel_name_with_sharp = Channel::Guild(channel.to_owned()).to_string();
        let channel_id = channel.id;
        let category_id = channel.category_id;
        let is_category = channel.kind == ChannelType::Category;
        // カテゴリだったらシャープなし。チャンネルだったらシャープあり
        let channel_name = if is_category {
            // これはカテゴリ名
            channel.name
        } else {
            // これはチャンネル名
            channel_name_with_sharp
        };
        Self {
            channel_name,
            channel_id,
            category_id,
        }
    }
}

struct AllChannel {
    data: Vec<MyChannel>,
}
