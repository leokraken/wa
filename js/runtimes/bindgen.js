const { hello_world, run_js_value_string, run_js_value_struct } = require('../../opensubtitle-wasm/pkg/opensubtitle_wasm');

function helloWorld() {
  const res = hello_world("hola");
  console.log(res);
}


function runJSValueString() {
  const res = run_js_value_string("some string");
  console.log(res);
}

function runJSValueStruct() {
  const data = {text:"awesome", number: 1};
  const res = run_js_value_struct(data);
  console.log(res);
}


function helloWorldIterations() {
  for (i = 0; i < 100; i++) {
    console.time("test")
    const res = hello_world();
    console.timeEnd("test");
  }
}


helloWorld();

runJSValueString();

runJSValueStruct();