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

    _put_string_buffer(str) {
        const len = str.length; // number of UTF-16 code units
        const ptr = this.wasm.alloc(len * 2);

        const buf = new Uint16Array(this.wasm.memory.buffer, ptr, len);

        for (var i = 0; i < len; ++i) {
            buf[i] = str.charCodeAt(i);
        }

        return ptr;
    }

    reverse_string(str) {
        const ptr = this._put_string_buffer(str);

        this.wasm.revstr(ptr);

        this.wasm.free(ptr);
    }

}
