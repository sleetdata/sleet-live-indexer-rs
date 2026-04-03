# sleet-live-indexer
indexer playground built around near-stream

🚧 WIP

ℹ️ this indexer playground is built around near-stream
<br/>
ℹ️ check it out at "https://github.com/sleetdata/near-stream"
<br/>
ℹ️ must run near-stream yourself

**WHY/WHAT?**
<br/>
Becuase fetching blocks from neardata for every indexer you want to run puts to much stress on fastnear's servers. So this is a playground of indexers built around near-stream, ingest them once, stream through muilple indexers.
<br/>
Obiviously there are a few ways to do this. I want to set up test indexers for indexing one data type. But maybe I will create one binnary that indexes muliple data types. Because I am hostng a near-stream server I can use it anyway I want.
<br/>
This is a playground after all.
<br/>
EDIT: I guess i should claify what i meant, the bin folder has runnable bins that are indexers. you can pick and choose the ones you want to run, run as many as you want, or remix them and make your own. I also have reuable functions that makes remixing easier.
<br/>
the indexers are clearly named, some just stream and print, some send or save data.

---

### CARGO COMMANDS
```sh
cargo run
cargo check
# cargo test
cargo clean
cargo fmt
cargo update
```

bins see [src/bin/README.md](./src/bin/README.md)

---

copyright 2026 by sleet.near