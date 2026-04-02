# tmux
if running this on personal home server

session
```sh
tmux ls
tmux new -s my-session
tmux attach -t my-session

# stream_tx_discord_blackjack
tmux new -s stream_tx_discord_blackjack
tmux attach -t stream_tx_discord_blackjack
export DISCORD_WEBHOOK_URL_BLACKJACK=""
cargo run --bin stream_tx_discord_blackjack
```


---

copyright 2026 by sleet.near