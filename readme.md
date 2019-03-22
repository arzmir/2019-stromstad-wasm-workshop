# Workshop: En liten introduksjon til WebAssembly
- **Laget for** Knowit Objectnet - Fagseminar
- **Holdt på** Strömstad Spa, Sverige - 23. mars 2019

## Abstract
WebAssembly har vært på manges tunger de siste årene. I det tilhørende foredraget gikk vi gjennom hva WebAssembly er, samt når og hvorfor man skulle ønske å benytte det. I workshoppen fokuserer vi på det grunnleggende - Å kompilere Rust-kode til en Wasm-modul som vi skal ta i bruk på en webside gjennom JavaScript.

For å legge til rette for at man skal kunne flytte seg mellom Villages så fokuserer vi bare på det grunnleggende, men har laget noen ekstra oppgaver og noen idéer til oppgaver til de som ønsker å jobbe mer med det.

**Omtrentlig tidsbruk:** 30-45 min


> The dream of WebAssembly is not to kill JavaScript but to work alongside of it, to help super charge processing-heavy or low-level tasks — tasks that benefit from Rust’s focus on performance.

## Ressurser
- Fra Rusts offiselle side
  - [Editor-verktøy](https://www.rust-lang.org/tools)
  - [Brukerforumet](https://users.rust-lang.org)
  - [Discord-kanal](https://discordapp.com/invite/rust-lang)
  - [Rust by example](https://doc.rust-lang.org/rust-by-example/index.html)
  - https://www.rust-lang.org/what/wasm
- Crates som man kan bruke
  - [Diverse algoritmer som er implementert i Rust](https://crates.io/categories/algorithms)

## Stikkord som du vil få kjennskap til
- Shared Array Buffer
- JavaScript-modules

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

## Fartsdumper
- `WebAssembly.instantiateStreaming` bryr seg om MIME-type, `.instantiate` gjer ikkje. Om man går for super-simpel lokal web-server med t.d. **live-server** så bruk sistnevnte metode.
- Ikkje bruk Wasm for mikrosaker, det er ein overhead på det.