# DiscordBotWithRust

Rust(serenity)製のDiscord Botです。小さく作ってあります。
[Rustで作るdiscord bot入門編 (serenity使用)](https://zenn.dev/t4t5u0/articles/cd731e0293cf224cb4dc) で解説をしています。



```bash
git clone git@github.com:t4t5u0/DiscordBotWithRust.git
cd discord-bot-rust
cargo run --release
```


## 開発者の人へ
`git clone` したあとに、
```
git update-index --assume unchanged config.json
```
すると、config.json の状態が固定されて、更新が無視されます。
