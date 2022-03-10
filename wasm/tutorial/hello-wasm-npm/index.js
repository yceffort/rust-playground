const js = import("./node_modules/@yceffort/hello-wasm/hello_wasm.js");
js.then(js => {
  js.greet("yceffort's first WebAssembly");
});