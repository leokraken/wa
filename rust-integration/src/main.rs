use std::fs::File;
use std::path::Path;
use wasmtime::*;
use wasmtime_wasi::sync::Dir;
use wasmtime_wasi::{sync::WasiCtxBuilder, Wasi};

fn run_wasi() {
    // Define the WASI functions globally on the `Config`.
    let mut config = Config::default();
    Wasi::add_to_config(&mut config);

    let store = Store::new(&Engine::new(&config).unwrap());

    // Set the WASI context in the store; all instances in the store share this context.
    // `WasiCtxBuilder` provides a number of ways to configure what the target program
    // will have access to.
    let path = Path::new("/Users/lclavijo/node/wa/js");
    let file = match File::open(&path) {
        Err(_err) => panic!("couldn't open file!"),
        Ok(file) => {
            println!("could open !");
            file
        }
    };

    let a = unsafe { Dir::from_std_file(file) };
    assert!(Wasi::set_context(
        &store,
        WasiCtxBuilder::new()
            .inherit_stdio()
            .arg("/sandbox/Book.pdf")
            .unwrap()
            //.inherit_args().unwrap()
            .preopened_dir(a, "/sandbox")
            .unwrap()
            .build()
            .unwrap()
    )
    .is_ok());

    let mut linker = Linker::new(&store);

    // Instantiate our module with the imports we've created, and run it.
    let module = Module::from_file(store.engine(), "/Users/lclavijo/node/wa/opensubtitle-wasi/target/wasm32-wasi/release/opensubtitle-wasi.wasm").unwrap();

    linker.module("", &module).unwrap();
    linker
        .get_default("")
        .unwrap()
        .typed::<(), ()>()
        .unwrap()
        .call(())
        .unwrap();

    linker
        .get_default("")
        .unwrap()
        .typed::<(), ()>()
        .unwrap()
        .call(())
        .unwrap();
}

fn run_wasm(){
    let mut config = Config::new();
    config.wasm_reference_types(true);
    let engine = Engine::new(&config).unwrap();
    let store = Store::new(&engine);
    let imports = [];
    let module = Module::from_file(&engine, "/Users/lclavijo/node/wa/opensubtitle-wasm/pkg/opensubtitle_wasm_bg.wasm").unwrap();
    let instance = Instance::new(&store, &module, &imports).unwrap();

    let func = instance.get_typed_func::<(), Option<ExternRef>>("hello_world_simple").unwrap();
    func.call(()).unwrap();

    store.gc();
}

fn main() {
    //run_wasi();
    run_wasm();
}
