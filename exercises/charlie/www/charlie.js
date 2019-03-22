fetch("2019_wasm_workshop_charlie.cleaned.wasm")
  .then(response => response.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes, {}))
  .then(results => {
    alert(results.instance.exports.multiply(20, 19));
  });
