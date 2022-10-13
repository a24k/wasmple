import wasmurl from '../wasmple/target/wasm32-unknown-unknown/wasmple.wasm?url';

export class Wasmple {

    static async prelude() {
        const wasmbin = await (await fetch(wasmurl)).arrayBuffer();
        const wasm = (await WebAssembly.instantiate(wasmbin)).instance;
        return new Wasmple(wasm);
    }

    constructor(wasm) {
        this.wasm = wasm.exports;
    }

    add(left, right) {
        return this.wasm.add(left, right);
    }

}
