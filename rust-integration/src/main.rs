use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, Wasi};
use wasmtime_wasi::sync::Dir;
use std::path::{Path};
use std::fs::File;
//use cap_std::fs::Dir;


fn main() {


    // Define the WASI functions globally on the `Config`.
    let mut config = Config::default();
    Wasi::add_to_config(&mut config);

    let store = Store::new(&Engine::new(&config).unwrap());

    // Set the WASI context in the store; all instances in the store share this context.
    // `WasiCtxBuilder` provides a number of ways to configure what the target program
    // will have access to.
    let path = Path::new("/Users/lclavijo/node/wa/js");
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open file!"),
        Ok(file) => {println!("could open !"); file},
     }; 

    let a = unsafe {
        Dir::from_std_file(file)
    };
    assert!(Wasi::set_context(
        &store,
        WasiCtxBuilder::new()
            .inherit_stdio()
            .arg("/sandbox/Book.pdf").unwrap()
            //.inherit_args().unwrap()
            .preopened_dir(a, "/sandbox").unwrap()
            .build().unwrap()
    )
    .is_ok());

    let mut linker = Linker::new(&store);

    // Instantiate our module with the imports we've created, and run it.
    let module = Module::from_file(store.engine(), "/Users/lclavijo/node/wa/opensubtitle-wasi/target/wasm32-wasi/release/opensubtitle-wasi.wasm").unwrap();

    linker.module("", &module).unwrap();
    linker.get_default("")
        .unwrap()
        .typed::<(), ()>()
        .unwrap()
        .call(())
        .unwrap();

    linker.get_default("")
        .unwrap()
        .typed::<(), ()>()
        .unwrap()
        .call(())
        .unwrap();

}
