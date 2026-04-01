# BIN



```sh
# test enpoint
timeout 2s curl -N http://192.168.4.34:8080
# bin
source .env

# validate final block test
cargo run --bin fetch_final_block_json

# stream print
cargo run --bin stream_blocks_print
cargo run --bin stream_shards_print
cargo run --bin stream_tx_actions_createaccount_print
cargo run --bin stream_tx_actions_deleteaccount_print
cargo run --bin stream_tx_actions_functioncall_print
cargo run --bin stream_tx_actions_transfer_print
```

---

copyright 2026 by sleet.near