# Oppgave 3 - Highway to the d.. web
Vi har no pakka ein aldri så liten funksjon til WebAssembly og skal nå prøve å benytte denne i JavaScript.

Mappestrukturen vår ser slik ut, og vi ønsker å benytte oss av `2019_wasm_workshop_charlie.cleaned.wasm` (filer osv. er omdøpt ihht. oppgave, men det er ikkje nødvendig).

```
.
.
|-- Cargo.lock
|-- Cargo.toml
|-- readme.md
|-- src
|   `-- charlie.rs
|-- target
|   ...
`-- www
    |-- 2019_wasm_workshop_charlie.cleaned.wasm
    |-- charlie.js
    `-- index.html
```

## Lage nettsida vår
Sidan vi fokuserer på koblinga mellom Wasm <-> JS, så gjer vi ingenting fancy med HTML-en.

### `index.html`
```html
<!doctype HTML>
<html>
<head>
</head>
<body>
  <script src="charlie.js" type="text/javascript" charset="utf-8"></script>
</body>
</html>
```

### `charlie.js`
Det finnes nokre ulike måter å få tak i modulen, den enklaste er å benytte seg av `fetch`. Alternativt kan ein gjere kallet med `XMLHttpRequest`. Videre så har `WebAssembly`-globalen to måter å laste fil-innholdet sånn at det kan nås fra JS, `instantiate` og `instantiateStreaming`. Sistnevnte er ein god del raskare, men den krever at vi server Wasm-fila med korrekt MIME-type fra vår webserver, som vi fint glattar over i denne tutorialen.

```javascript
fetch('2019_wasm_workshop_charlie.cleaned.wasm')
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes, {}))
    .then(results => {
      alert(results.instance.exports.multiply(20, 19));
    });
```

### `live-server`
For å kunne sjå om dette faktisk fungerer, så må vi serve filene våre på et vis. Vi har valgt npm-pakken `live-server` då den er veldig enkel å handskes med. Vi må tillate Cross-Origin Resource Sharing på serveren sånn at vi får lov til å laste lokale filer via fetch-apiet.


```bash
live-server --cors ./www

# Serving "./www" at http://127.0.0.1:8080
# Ready for changes
```

Denne åpner da automatisk `index.html` i standard nettleser dersom fila eksisterer