import init from "/wasm/acquire_rs_wasm.js";
//let wasm = null;
//const importObject = {
//        module: {},
//        env: {
//          memory: new WebAssembly.Memory({ initial: 256 }),
//        }
//      };
//WebAssembly.instantiateStreaming(fetch("wasm"), importObject).then(
//  (results) => {
//    // Do something with the results!
//    wasm = results;
//    console.log("Successfully fetched wasm code!")
//  }
//);
//
//
async function test() {
    console.log("Hello!");
    let wasm = await init();
    console.log(wasm);
    wasm.change_something();
}
//
//function changeButton() {
//    wasm.instance.exports.change_something();
//}

document.addEventListener("DOMContentLoaded", function(){
    console.log("DOM content loaded");
    document.getElementById("button").addEventListener('click', test);
});