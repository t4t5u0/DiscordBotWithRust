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
        let channel_with_category = AllChannel::new(channels);
        msg.channel_id
            .say(&ctx.http, &channel_with_category.repl())
            .await?;
    } else {
        msg.channel_id.say(&ctx.http, "ギルドではない").await?;
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct MyChannel {
    is_category: bool,
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
            is_category,
            channel_name,
            channel_id,
            category_id,
        }
    }
}

#[derive(Debug)]
struct AllChannel {
    categories: Vec<MyChannel>,
    inner_channels: Vec<MyChannel>,
    outer_channels: Vec<MyChannel>,
}

impl AllChannel {
    fn new(data: Vec<MyChannel>) -> Self {
        // ほんとはpartition使いたかった
        // 夜糸さんありがとうございます
        // カテゴリーを抽出
        let (categories, lesidual): (Vec<_>, Vec<_>) =
            data.into_iter().partition(|x| x.is_category);
        // カテゴリー内外に振り分け
        let (inner_channels, outer_channels): (Vec<_>, Vec<_>) =
            lesidual.into_iter().partition(|x| x.category_id != None);
        Self {
            categories,
            inner_channels,
            outer_channels,
        }
    }
    fn repl(&self) -> String {
        let mut result = String::new();
        result.push_str(
            &self
                .outer_channels
                .to_owned()
                .into_iter()
                .map(|x| x.channel_name)
                .collect::<Vec<String>>()
                .join("\n"),
        );
        result.push_str("\n");
        for category in &self.categories {
            result.push_str(&format!("{}\n", category.channel_name));
            let mut matched_inner_channels = self
                .inner_channels
                .to_owned()
                .into_iter()
                .filter(|x| x.category_id.unwrap() == category.channel_id)
                .collect::<Vec<MyChannel>>();
            if matched_inner_channels.len() != 0 {
                let last_item = &matched_inner_channels.remove(matched_inner_channels.len() - 1);
                for inner in &matched_inner_channels {
                    result.push_str(&format!("├──{}\n", inner.channel_name));
                }
                result.push_str(&format!("└──{}\n", last_item.channel_name));
            }
        }
        result
    }
}
