const {hash, hello_world} = require('./out/opensubtitle_wasm');

console.time('wasi')
console.log(hello_world());
console.timeEnd('wasi')

console.time('wasi')
console.log(hello_world());
console.timeEnd('wasi')