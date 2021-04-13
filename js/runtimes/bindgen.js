const {hello_world} = require('../../opensubtitle-wasm/pkg/opensubtitle_wasm');
const res = hello_world();
console.log(res);


for (i=0; i<100;i++){
  console.time("test")
  const res = hello_world();
  console.timeEnd("test");
}