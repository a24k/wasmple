import wasmurl from '../../wasmple/target/wasm32-unknown-unknown/wasmple.wasm?url';

import { Console } from './console';
import { WasmBuffer } from './buffer';

type FnRevstr = (ptr: number) => number;

export class Wasmple {

    public static async init(): Promise<Wasmple> {
        const wasmbin = await (await fetch(wasmurl)).arrayBuffer();

        const console = new Console();

        const imports = {
            console: console.imports,
        };

        const wasm = (await WebAssembly.instantiate(wasmbin, imports)).instance.exports;

        console.init(wasm);

        return new Wasmple(wasm);
    }

    private buffer: WasmBuffer;
    private revstr: FnRevstr;

    constructor(wasm: WebAssembly.Exports) {
        this.buffer = new WasmBuffer(wasm);
        this.revstr = wasm.revstr as FnRevstr;
    }

    reverse_string(input: string): string {
        const input_ptr = this.buffer.put_string(input);

        const output_ptr = this.revstr(input_ptr);

        const output = this.buffer.get_string(output_ptr);

        this.buffer.dealloc(input_ptr);
        this.buffer.dealloc(output_ptr);

        return output;
    }

}
