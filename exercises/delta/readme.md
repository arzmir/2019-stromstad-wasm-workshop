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

### Pakking av dette som ein npm-modul vi kan importere
```bash
wasm-pack build
```

Da får vi ein `/pkg` som inneholder:
```
`-- pkg
    |-- 2019_wasm_workshop_delta.d.ts
    |-- 2019_wasm_workshop_delta.js
    |-- 2019_wasm_workshop_delta_bg.d.ts
    |-- 2019_wasm_workshop_delta_bg.wasm
    |-- README.md
    `-- package.json
```

### Sette opp prosjektet som ein enkel webapp
```bash
npm init wasm-app www
```
```
# tree www
www
|-- LICENSE-APACHE
|-- LICENSE-MIT
|-- README.md
|-- bootstrap.js
|-- index.html
|-- index.js
|-- package-lock.json
|-- package.json
`-- webpack.config.js
```

### Linke modulene og skifte ut kva som kjøres
```bash
# Sette opp /pkg som lokal npm modul
cd ./pkg && npm link && cd ..
# npm notice created a lockfile as package-lock.json. You should commit this # file.
# npm WARN delta@0.1.0 No repository field.
# npm WARN delta@0.1.0 No license field.
#
# up to date in 2.894s
# found 0 vulnerabilities
#
# ..../lib/node_modules/delta -> .../2019-stromstad-wasm-workshop/exercises/delta/pkg
```

### Redigere `index.js` til å bruke ønsket pakke
```javascript
import * as wasm from "delta";

console.log(wasm.return_string("Wheeee... Roundtrip through WASM!"));
```

### Installere dependencies, linke pakken vår og kjøre
```bash
cd www
npm install
npm link delta
npm run start
```