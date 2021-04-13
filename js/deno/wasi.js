import Context from "https://deno.land/std@0.92.0/wasi/snapshot_preview1.ts";

const context = new Context({
  args: Deno.args,
  env: Deno.env.toObject(),
  args: ["/sandbox/Book.pdf"],
  preopens: {
    '/sandbox': Deno.cwd(),
  },
});

const binary = await Deno.readFile("./wasm/opensubtitle-wasi.wasm");
const module = await WebAssembly.compile(binary);
const instance = await WebAssembly.instantiate(module, {
  "wasi_snapshot_preview1": context.exports,
});

context.start(instance);