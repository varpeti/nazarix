import { initSync } from "./nazarix";
import wasm_bytes from "./nazarix_bg.wasm.bin";
initSync(wasm_bytes);
export * from "./nazarix";
