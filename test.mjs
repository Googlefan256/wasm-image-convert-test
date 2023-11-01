import { readFileSync, writeFileSync } from "node:fs";
import * as img from "./wasm/pkg/wasm_bg.js";
img.__wbg_set_wasm((await WebAssembly.instantiate(readFileSync("./wasm/pkg/wasm_bg.wasm"))).instance.exports);

const inputFile = readFileSync("artifacter.png");
const input = new Uint8Array(inputFile);
const now = Date.now();
let image = null;
for(let i = 0; i < 100; i++) {
    image = img.convert(input, "jpeg")
}
const end = Date.now();
console.log(end - now);
writeFileSync("test.jpeg", image);