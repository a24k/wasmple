import wasmurl from '../../wasmple/target/wasm32-unknown-unknown/wasmple.wasm?url';

import type { BufferPtr } from './buffer';
import { WasmConsole } from './console';
import { WasmBuffer } from './buffer';

type FnConvert = (ptr: BufferPtr) => BufferPtr;

export type FnConvertResult = { interleaved: string, reversed: string };

export class Wasmple {

    public static async init(): Promise<Wasmple> {
        const wasmbin = await (await fetch(wasmurl)).arrayBuffer();

        const console = new WasmConsole();

        const imports = {
            console: console.imports,
        };

        const wasm = (await WebAssembly.instantiate(wasmbin, imports)).instance.exports;

        console.init(wasm);

        return new Wasmple(wasm);
    }

    private buffer: WasmBuffer;
    private convert: FnConvert;

    constructor(wasm: WebAssembly.Exports) {
        this.buffer = new WasmBuffer(wasm);
        this.convert = wasm.convert as FnConvert;
    }

    convert_string(inputA: string, inputB: string): FnConvertResult {
        const inputJsonPtr = this.buffer.from.object({
            a: inputA,
            b: inputB,
        });

        const outputJsonPtr = this.convert(inputJsonPtr);
        const outputJson = this.buffer.to.object(outputJsonPtr) as FnConvertResult;

        this.buffer.dealloc(inputJsonPtr);
        this.buffer.dealloc(outputJsonPtr);

        return outputJson;
    }

}
