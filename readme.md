# Workshop: En liten introduksjon til WebAssembly
- **Laget for** Knowit Objectnet - Fagseminar
- **Holdt på** Strömstad Spa, Sverige - 23. mars 2019

## Abstract
WebAssembly har vært på manges tunger de siste årene. I det tilhørende foredraget gikk vi gjennom hva WebAssembly er, samt når og hvorfor man skulle ønske å benytte det. I workshoppen fokuserer vi på det grunnleggende - Å kompilere Rust-kode til en Wasm-modul som vi skal ta i bruk på en webside gjennom JavaScript.

For å legge til rette for at man skal kunne flytte seg mellom Villages så fokuserer vi bare på det grunnleggende, men har laget noen ekstra oppgaver og noen idéer til oppgaver til de som ønsker å jobbe mer med det.

**Omtrentlig tidsbruk:** 30-45 min


## Ressurser
- Offisell nettside for Rust: https://www.rust-lang.org
- https://www.rust-lang.org/what/wasm


## Stikkord som du vil få kjennskap til
- Shared Array Buffer
- JavaScript-modules

## Nødvendige verktøy
- **rustup** - Runtime
- **rustc** - Kompilator
- **cargo** - Pakkestyrer tilsvarende npm for node
- **wasm-pack** - Byggeverktøy som gjør det mulig å kompilere til WebAssembly

**Quick-start:**
```bash
# https://www.rust-lang.org/tools/install
curl https://sh.rustup.rs -sSf | sh

# https://rustwasm.github.io/wasm-pack/installer/
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# https://github.com/arzmir/2019-stromstad-wasm-workshop
git clone https://github.com/arzmir/2019-stromstad-wasm-workshop.git
```

## Oppgaver
1. Kompilere Rust-kode til Wasm
2. Ta i bruk den pakkede modulen i JS
3. Kalle JavaScript-funksjoner fra Wasm-modulen
4. Benytte Shared Array Buffer for annen data enn tall
5.


## Ekstraoppgaver
- Pakke og publisere en wasm-modul til npm (https://rustwasm.github.io/wasm-pack/book/tutorials/npm-browser-packages/index.html)

## Idéer som kan utforskes
- Hente en liste med navn fra server og se om man kan øke hastigheten på sortering sammenlignet JavaScript ved å streame dette direkte til Shared Array Buffer samtidig som man laster inn initiell data.