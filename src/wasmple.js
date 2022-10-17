import wasmurl from '../wasmple/target/wasm32-unknown-unknown/wasmple.wasm?url';

export class Wasmple {

    static async prelude() {
        const wasmple = new Wasmple();
        await wasmple.init();
        return wasmple;
    }

    async init() {
        const wasmbin = await (await fetch(wasmurl)).arrayBuffer();

        const imports = {
            console: this._imports_console,
        };

        this.wasm = (await WebAssembly.instantiate(wasmbin, imports)).instance.exports;

        this.wasm.init();
    }

    _imports_console = {
        console_message: (level, ptr, len) => {
            const chars = new Uint16Array(this.wasm.memory.buffer, ptr, len);
            const msg = String.fromCharCode(...chars);
            switch (level) {
                case 4:
                    console.error(msg); break;
                case 3:
                    console.warn(msg); break;
                case 2:
                    console.info(msg); break;
                case 1:
                    console.debug(msg); break;
                case 0:
                default:
                    console.log(msg); break;
            }
        },
    };

    alloc_and_free() {
        const ptr1 = this.wasm.alloc(0x100);
        console.log("js: ptr1 allocated\tat 0x" + ptr1.toString(16));

        const ptr2 = this.wasm.alloc(0x100);
        console.log("js: ptr2 allocated\tat 0x" + ptr2.toString(16));

        const len1 = this.wasm.free(ptr1);
        console.log("js: ptr1 free " + len1 + " bytes");

        const ptr3 = this.wasm.alloc(0x80);
        console.log("js: ptr3 allocated\tat 0x" + ptr3.toString(16));

        const ptr4 = this.wasm.alloc(0x80);
        console.log("js: ptr4 allocated\tat 0x" + ptr4.toString(16));

        const len2 = this.wasm.free(ptr2);
        console.log("js: ptr2 free " + len2 + " bytes");

        const len3 = this.wasm.free(ptr3);
        console.log("js: ptr3 free " + len3 + " bytes");

        const len4 = this.wasm.free(ptr4);
        console.log("js: ptr4 free " + len4 + " bytes");
    }

}
