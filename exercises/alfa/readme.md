# Oppgaver 1 - Installere nødvendige verktøy og initialisere et Rust-prosjekt

## Nødvendige verktøy for å starte
`NB:` Vi kjem til å legge til nokre ekstra verktøy etterkvart.

### Basic støtte for Rust som språk - ([offisell nettside](https://www.rust-lang.org))
Installerer:
- **rustup** - Runtime
- **rustc** - Kompilator
- **cargo** - Pakkestyrer tilsvarende npm for node

```bash
curl https://sh.rustup.rs -sSf | sh
```

### Wasm-pack - ([offisiell nettside](https://rustwasm.github.io/wasm-pack/installer/))
Installerer:
- **wasm-pack** - Byggeverktøy som gjør det mulig å kompilere til WebAssembly

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Sjekk at man har wasm-target
rustup target add wasm32-unknown-unknown
```

### Node - ([offisell nettside](https://nodejs.org/en/))
Installerer:
- **nvm** - Pakkestyrer som gjer det enkelt å bruke fleire versjoner av node
- **node** - JavaScript-motor som kjører lokalt på maskina
```bash
# Installer nvm via brew
brew install nvm

# Installer node 10 (LTS) via nvm
nvm install 10

# Sett til at vi bruker node 10
nvm use 10
```

### Live-server - ([npm-repo](https://www.npmjs.com/package/live-server))
Installerer:
- **live-server** - Liten node webserver sånn at man får lov til å hente wasm-filer i fetch-kall. Man kan velge å installere globalt, lokalt i mappa ein jobbar i, eller berre kjøre den via npx (lastes ned, installeres midlertidig, og avinstalleres med en gang man terminerer).

```bash
# globalt
npm install -g live-server

# lokalt
npm install live-server

# npx
npx live-server [--options]
```

## Lager oss eit lite Rust-prosjekt
Oppsett for `Hello world` er lagt rett inn i `cargo` sånn at ein kjem raskt opp å kjøre. Man kan splitte opp oppgavene i fleire prosjekter om man ønsker det.

```bash
# Naturligvis berre å endre plassering om ønskelig
mkdir -p ~/2019-stromstad-wasm-workshop/
cd ~/2019-stromstad-wasm-workshop/
cargo init

# Åpne i ønsket editor f.eks. VS Code
code ./
```

Her ser vi at `cargo init` har laget ein liten mappestruktur for oss
```
# Bruker `tree` for å liste ut
.
|-- Cargo.toml     # Inneholder configurasjon for rust-prosjektet
`-- src
    `-- main.rs    # Main-metode som kjører printer "Hello world" nå
```

Vi kan så teste dette ved å kjøre `cargo run`. Dette kjører main-metoden og legger til litt ekstra til strukturen vår.

```
.
|-- Cargo.lock
|-- Cargo.toml
|-- src
|   `-- main.rs
`-- target
    `-- debug
        |-- alfa      <--- Vårt program som no kan kjøres rett fra kommandolinja
        |-- alfa.d
        |-- alfa.dSYM -> deps/alfa-2e53be4a79decf27.dSYM
        |-- build
        |-- deps
        |-- examples
        |-- incremental
        `-- native
```

## Vi mekker oss ein liten funksjon
```rust
// ./src/main.rs
fn multiply(a: u32, b: u32) -> u32 {
    a * b
}

// Og endrer main funksjonen til å benytte dette
fn main() {
    println!("Hello, area {}!", multiply(17,3));
}
```

Kjører dette på nytt og skal få `Hello, area 51!`.

Gratulerer du er no ein Rust-ekspert! :P