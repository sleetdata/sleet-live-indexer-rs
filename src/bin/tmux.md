# tmux
if running this on personal home server

session
```sh
tmux ls
tmux new -s my-session
tmux attach -t my-session

source .env

# stream_tx_discord_blackjack
tmux new -s stream_tx_discord_blackjack
tmux attach -t stream_tx_discord_blackjack
export DISCORD_WEBHOOK_URL_BLACKJACK=""
cargo run --bin stream_tx_discord_blackjack

# delete account notifications (SQLite + Discord)
tmux new -s stream_tx_discord_deleteaccount
tmux attach -t stream_tx_discord_deleteaccount
export DISCORD_WEBHOOK_URL_DELETEACCOUNT=""
cargo run --bin stream_tx_discord_deleteaccount

# validate and error check
tmux new -s stream_blocks_error_log
tmux attach -t stream_blocks_error_log
cargo run --bin stream_blocks_error_log

```


---

copyright 2026 by sleet.near