# BIN



```sh
# test enpoint
timeout 2s curl -N http://192.168.4.34:8080
# bin
source .env

# validate and error check
cargo run --bin fetch_final_block_json
cargo run --bin stream_blocks_error_log

# stream print
cargo run --bin stream_blocks_print
cargo run --bin stream_shards_print
# actions
cargo run --bin stream_tx_actions_createaccount_print
cargo run --bin stream_tx_actions_deleteaccount_print
cargo run --bin stream_tx_actions_functioncall_print
cargo run --bin stream_tx_actions_transfer_print

# functioncall/other
cargo run --bin stream_tx_functioncall_create_account
cargo run --bin stream_tx_receiver_blackjack

# discord webhooks
export DISCORD_WEBHOOK_URL_BLACKJACK=""
cargo run --bin stream_tx_discord_blackjack
```

---



---

copyright 2026 by sleet.near