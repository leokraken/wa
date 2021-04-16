const { hello_world, http_client } = require('../../opensubtitle-wasm/pkg/opensubtitle_wasm');

function helloWorld() {
  const res = hello_world("hola");
  console.log(res);
}

async function client(){
  const result = await http_client();
  console.log(result);
  return result;
}


function helloWorldIterations() {
  for (i = 0; i < 100; i++) {
    console.time("test")
    const res = hello_world();
    console.timeEnd("test");
  }
}


helloWorld();