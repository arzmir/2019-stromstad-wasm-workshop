# Oppgave 4 - But, but, but... Strings?
WebAssembly har per dags dato kun native støtte for fire primitiver, 32 bit ints og floats, signed og unsigned. Men man kan benytte en SharedArrayBuffer for å kunne jobbe på annen data etter ønske. Denne fungere som eit enkelt byte array som man leser og skriver til med pointers (?) (indeksert med tall). Denne har standard-størrelse på 64kb og kan per dags dato utvidast opp til 4gb (kanskje det var 2gb). 64-bit støtte er under utvikling.

MEN. Det er ein del boilerplate som må til for å kunne kommunisere på denne måte, så det finst naturligvis bibliotek for dette. Vi sett i gang.

## Bindgen-macro
## Kalle funksjoner i javascript _fra_ wasm

