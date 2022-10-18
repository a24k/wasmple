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
        const _allocate_with_log = (name, len) => {
            const ptr = this.wasm.alloc(len);

            console.log("js: " + name + " allocated"
                + "\tat 0x" + ptr.toString(16)
                + "\twith " + this.wasm.size_of(ptr) + " bytes");

            return ptr;
        }

        const _log_size_of = (name, ptr) => {
            const len = this.wasm.size_of(ptr);

            console.log("js: " + name + " has " + len + " bytes");

            return len;
        }

        const _free_with_log = (name, ptr) => {
            const len = this.wasm.free(ptr);

            console.log("js: " + name + " freed "
                + "\t" + len + " bytes");

            return len;
        }

        const ptr1 = _allocate_with_log("ptr1", 0x100);
        const ptr2 = _allocate_with_log("ptr2", 0x100);

        _log_size_of("ptr1", ptr1);

        _free_with_log("ptr1", ptr1);

        _log_size_of("ptr1", ptr1);

        const ptr3 = _allocate_with_log("ptr3", 0x80);

        _log_size_of("ptr1", ptr1);

        const ptr4 = _allocate_with_log("ptr4", 0x80);

        _free_with_log("ptr2", ptr2);
        _free_with_log("ptr3", ptr3);
        _free_with_log("ptr4", ptr4);
    }

}
