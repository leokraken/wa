const assert = require('assert');
const {
  hello_world,
  run_js_value_string,
  run_js_value_struct,
  run_jsvalue_on_return
} = require('../../opensubtitle-wasm/pkg/opensubtitle_wasm');

function helloWorld() {
  const result = hello_world("hola");
  assert.strictEqual("hello world hola", result);
  console.log(result);
}

function runJSValueString() {
  const result = run_js_value_string("some string");
  assert.strictEqual("hello world some string", result);
  console.log(result);
}

function runJSValueStruct(data) {
  try {
    const result = run_js_value_struct(data);
    assert.strictEqual("hello world awesome 1", result);
    console.log(result);
  } catch (err) {
    console.error("Error!", err);
  }
}

function runJSValueAsReturn() {
  try {
    const result = run_jsvalue_on_return();
    assert.deepStrictEqual({ text: 'text', number: 1 }, result);
    console.log(result);
  } catch (err) {
    console.error("Error happens :'(", err);
  }
}

function helloWorldIterations() {
  for (i = 0; i < 100; i++) {
    console.time("test")
    const res = hello_world();
    console.timeEnd("test");
  }
}

// Hello world with name
helloWorld();

// Any value serialized to string
runJSValueString();

// Working with rust struct serialization json
const dataOk = { text: "awesome", number: 1 };
const dataErr = "";

runJSValueStruct(dataOk);
runJSValueStruct(dataErr);


runJSValueAsReturn();