const fs = require("fs");
const { WASI } = require("wasi");

async function runWasi(wasm) {
  const wasi = new WASI({
    args: ["/sandbox/Book.pdf"],
    preopens: {
      '/sandbox': process.cwd(),
    },
  });
  const importObject = { wasi_snapshot_preview1: wasi.wasiImport };
  const instance = await WebAssembly.instantiate(wasm, importObject);
  wasi.start(instance);
}


async function main() {
  const wasm = await WebAssembly.compile(
    fs.readFileSync("./wasm/opensubtitle-wasi.wasm"),
  );
  await runWasi(wasm);
};

main();
