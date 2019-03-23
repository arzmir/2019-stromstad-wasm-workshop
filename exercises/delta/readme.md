# Oppgave 4 - But, but, but... Strings?
WebAssembly har per dags dato kun native støtte for fire primitiver, 32 bit ints og floats, signed og unsigned. Men man kan benytte en SharedArrayBuffer for å kunne jobbe på annen data etter ønske. Denne fungere som eit enkelt byte array som man leser og skriver til med pointers (?) (indeksert med tall). Denne har standard-størrelse på 64kb og kan per dags dato utvidast opp til 4gb (kanskje det var 2gb). 64-bit støtte er under utvikling.

MEN. Det er ein del boilerplate som må til for å kunne kommunisere på denne måte, så det finst naturligvis bibliotek for dette. Vi sett i gang.

## wasm-bindgen
Den eklaste måten å lage funksjoner som kan returnere annen data enn tall er via `wasm-bindgen`. Det er eit par steg vi må gjennom for å benytte det.

Gitt følgende struktur:
```
|-- Cargo.lock
|-- Cargo.toml
|-- readme.md
`-- src
    `-- delta.rs
```

La oss gå gjennom og generere det som trengs av kode via `wasm-bindgen`. Først må vi redigere `Cargo.toml` og `delta.rs`

### `cargo.toml`
```toml
# Legg til under dependency

[dependencies]
wasm-bindgen = "0.2.29"
```

### `delta.rs`
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub extern "C" fn return_string(my_text :&str) -> String {
  return my_text;
}
```

