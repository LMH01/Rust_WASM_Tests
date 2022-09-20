import init from "/wasm/acquire_rs_wasm.js";
let wasm = null;

async function test() {
    console.log("Hello!");
    console.log(wasm);
    wasm.change_something();
}

document.addEventListener("DOMContentLoaded", async function(){
    console.log("DOM content loaded");
    wasm = await init();
    document.getElementById("button").addEventListener('click', test);
});