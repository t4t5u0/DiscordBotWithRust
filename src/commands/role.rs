use serenity::framework::standard::macros::group;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::http::CacheHttp;
use serenity::model::prelude::*;
use serenity::prelude::*;

// TODO:
// [ ] コマンドたちをroleのサブコマンドにする
// [ ] roleの設定を変更するコマンド
// [ ] role一覧を表示するコマンド
// [ ] あるroleの詳しい情報を表示するコマンド

#[command]
#[description("role一覧")]
async fn all_role(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild_id.unwrap();
    let roles = guild.roles(&ctx.http()).await?;

    for (role_id, role) in roles.iter() {
        let name = if &role.name == "@everyone" {
            "everyone"
        } else {
            &role.name
        };
        msg.channel_id
            .say(
                &ctx.http,
                format!("id: `{}`, role_name: {}", &role_id, name),
            )
            .await?;
    }
    Ok(())
}

#[command]
// #[num_args(1)]
#[description("role作成")]
async fn create_role(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        &msg.channel_id
            .say(&ctx.http, format!("引数が1つ必要です",))
            .await;
    }
    let role_names = args
        .iter::<String>()
        .map(|s| s.unwrap_or("".to_string()))
        .collect::<Vec<_>>();

    let guild_id = msg.guild_id.unwrap();
    for role_name in role_names {
        match guild_id.create_role(&ctx.http, |r| r.name(role_name)).await {
            Ok(role) => {
                &msg.channel_id
                    .say(&ctx.http, format!("{}", role.name))
                    .await;
            }
            Err(error) => {
                &msg.channel_id
                    .say(
                        &ctx.http,
                        format! {"Error: {}\nロールを作成することができませんでした。", error},
                    )
                    .await;
            }
        }
    }
    Ok(())
}

#[command]
#[description("role削除")]
async fn delete_role(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        &msg.channel_id
            .say(&ctx.http, format!("引数が1つ以上必要です",))
            .await;
        return Ok(());
    }
    // args -> Vec<String> -> Vec<u_64> -> Vec<RoleId ,Role>
    // これすると、<@&[0-9]{16}> みたいな感じで入ってくる。桁数は適当

    args.parse::<String>().unwrap();
    let mut role_ids: Vec<u64> = vec![];
    args.parse::<String>().unwrap();
    while let Ok(role) = args.single::<String>() {
        println!("{}", role);
        if role.starts_with("<@&") && role.ends_with(">") {
            let role_id = role
                .strip_prefix("<@&")
                .unwrap()
                .strip_suffix(">")
                .unwrap()
                .parse::<u64>()
                .unwrap();
            role_ids.push(role_id);
        }
    }

    let guild_id = msg.guild_id.unwrap();

    // ここ適当なので再実装する
    let roles = guild_id.roles(&ctx.http).await?;
    let (want_to_delete, _not_exist_roles): (Vec<_>, Vec<_>) = roles
        .into_iter()
        .partition(|role| role_ids.contains(role.0.as_u64()));

    for (id, r) in want_to_delete {
        println!("{}", r.name);
        guild_id.delete_role(&ctx.http, id).await?;
        &msg.channel_id
            .say(&ctx.http, format!("{} というロールを消去しました", r.name))
            .await;
    }

    // for (_, r) in not_exist_roles {
    //     &msg.channel_id
    //         .say(&ctx.http, format!("{} というロールは存在しません", r.name))
    //         .await;
    // }
    Ok(())
}

#[command]
async fn args(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut a: Vec<u64> = vec![];
    args.parse::<String>().unwrap();
    while let Ok(role) = args.single::<String>() {
        println!("{}", role);
        if role.starts_with("<@&") && role.ends_with(">") {
            let role_id = role
                .strip_prefix("<@&")
                .unwrap()
                .strip_suffix(">")
                .unwrap()
                .parse::<u64>()
                .unwrap();
            a.push(role_id);
        }
    }
    &msg.channel_id.say(&ctx.http, format!("{:?}", a)).await?;
    Ok(())
}

#[group]
#[description("ロール系")]
#[summary("ロール")]
#[commands(all_role, create_role, delete_role)]
pub struct Role;
