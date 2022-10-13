import wasmurl from '../wasmple/target/wasm32-unknown-unknown/wasmple.wasm?url';

export class Wasmple {

    static async prelude() {
        const wasmbin = await (await fetch(wasmurl)).arrayBuffer();

        const imports = {
            env: {
                console_log: () => {
                    console.log("Hello WebAssembly!");
                },
            },
        };

        const wasm = (await WebAssembly.instantiate(wasmbin, imports)).instance;
        return new Wasmple(wasm);
    }

    constructor(wasm) {
        this.wasm = wasm.exports;
    }

    hello() {
        return this.wasm.hello();
    }

}
