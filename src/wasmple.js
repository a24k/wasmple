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
        const ptr = this.wasm.alloc_u16(len);

        const buf = new Uint16Array(this.wasm.memory.buffer, ptr, len);

        for (var i = 0; i < len; ++i) { buf[i] = str.charCodeAt(i); }

        return ptr;
    }

    _get_string_buffer(ptr) {
        const len = this.wasm.length_u16(ptr);
        const chars = new Uint16Array(this.wasm.memory.buffer, ptr, len);
        return String.fromCharCode(...chars);
    }

    reverse_string(input) {
        const input_ptr = this._put_string_buffer(input);

        const output_ptr = this.wasm.reverse_string(input_ptr);

        const output = this._get_string_buffer(output_ptr);

        this.wasm.dealloc(input_ptr);
        this.wasm.dealloc(output_ptr);

        return output;
    }

}
