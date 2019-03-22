# Oppgave 2 - Men dette er jo ein Wasm-workshop?
Det stemmer! Så no skal vi gjere absolutt minimum for å kjøre vår Rust-kode i nettleseren som WebAssembly.

## Kompilere til wasm
Dette gjer vi ved å spesifisere for kva plattform vi ynskjer å kompilere for. Når vi la inn `wasm-pack` så kom den med eit nytt build-target som heitte `wasm32-unknown-unknown` som tolkes slik `{architecture}-{vendor}-{system}`. Man kan liste ut dei arkitekturane som er støtta ved å kjøre `rustup target list` (ikkje relevant for workshopen). Vi må også endre litt på koda vår sann at den blir tilgjengelig i JavaScript, men det kjem i eit seinare steg.

```bash
# Vi gir cargo et build-target
$ cargo build --target wasm32-unknown-unknown

# Så sjekker vi om vi har fått en wasm fil eller to
find ./ -name ".wasm"
# > ./target/wasm32-unknown-unknown/debug/beta.wasm
# > ./target/wasm32-unknown-unknown/debug/deps/beta.wasm
```

### Tar en liten kikk på .wasm-filen
```bash
ls -lh ./target/**/*.wasm
# 1.4M  Mar 22 07:30  ./target/wasm32-unknown-unknown/debug/beta.wasm
```

Yup. **`1.4mb!`**.



## Tilpasninger av prosjektet
Webassembly-filer blir relativt store fordi de pakker med seg alt det dei treng for at Rust kan kjøre uavhengig av rotsystemet. Men man kan ta ein del grep for å redusere den endelige størrelsen.


### Velge ut kva funksjonar som skal eksporterast
```rust
// src/beta.rs

// Legg til pub extern "C"
pub extern "C" fn multiply(a: u32, b: u32) -> u32 {
    a * b
}
```

### Benytte `cdylib`
Det er et bibliotek som lar oss bygge et dynamisk bibliotek som skal kalles fra C eller via et "Foreign Function Interface" (ffi). Når vi benytter det så strippes enkelte Rust-spesifikke funksjonaliteter fra den ferdige pakken.

```toml
# Legg til i Cargo.toml

[lib]
# Ønskenavn. Chars: [_a-z0-9]
name = "2019_wasm_workshop_beta"

# Filsti til det man skal pakke som bibliotek
path = "src/beta.rs"

# Defineringen av at det skal vere eit CDYnamiskLIBrary
crate-type = ['cdylib']
```

Når vi bygger ser vi at vi har tatt av 100kb:
```bash
# 1.3M  Mar 22 08:33  target/wasm32-unknown-unknown/debug/2019_wasm_workshop_beta.wasm
```

### Benytte `wasm-gc` - ([git repo](https://github.com/alexcrichton/wasm-gc))
Mange av dei statiske avhengighetene som Rust har pakka med, sjølv etter man sei man skal pakke som `cdylib`, så er det mykje ubrukt kode som kan fjernes. Dette er eit verktøy for akkurat det. Når vi seinare benytter oss av dei litt meir fullverdige biblioteka som `wasm-pack` så vil dette bli gjort som default, men det er greit å ha sett forskjellen.

```bash
cargo install --git https://github.com/alexcrichton/wasm-gc

wasm-gc target/wasm32-unknown-unknown/debug/2019_wasm_workshop_beta.wasm 2019_wasm_workshop_beta.cleaned.wasm

# 290B  Mar 22 08:42  2019_wasm_workshop_beta.cleaned.wasm
```

**Vi reduserte filstørrelsen med `1 mb`**!

Naturligvis så vil denne gevinsten vere variabel med kor mykje av standardbiblioteket til Rust man faktisk bruker.
