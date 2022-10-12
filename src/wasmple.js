import wasmurl from '../wasmple/target/wasm32-unknown-unknown/release/wasmple.wasm?url';

export class Wasmple {

    constructor(wasm) {
        this.wasm = wasm.exports;
    }

    static async prelude() {
        const wasmbin = await (await fetch(wasmurl)).arrayBuffer();

        const wasm = (await WebAssembly.instantiate(wasmbin)).instance;

        return new Wasmple(wasm);
    }

}
