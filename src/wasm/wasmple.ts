import wasmurl from '../../wasmple/target/wasm32-unknown-unknown/wasmple.wasm?url';

import type { BufferPtr } from './buffer';
import { WasmConsole } from './console';
import { WasmBuffer } from './buffer';

type FnInterleave = (ptr_a: BufferPtr, ptr_b: BufferPtr) => BufferPtr;
type FnReverse = (ptr: BufferPtr) => BufferPtr;

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
    private interleave: FnInterleave;
    private reverse: FnReverse;

    constructor(wasm: WebAssembly.Exports) {
        this.buffer = new WasmBuffer(wasm);
        this.interleave = wasm.interleave as FnInterleave;
        this.reverse = wasm.reverse as FnReverse;
    }

    interleave_string(inputA: string, inputB: string): string {
        const inputPtrA = this.buffer.from.string(inputA);
        const inputPtrB = this.buffer.from.string(inputB);

        const outputPtr = this.interleave(inputPtrA, inputPtrB);
        const output = this.buffer.to.string(outputPtr);

        this.buffer.dealloc(inputPtrA);
        this.buffer.dealloc(inputPtrB);
        this.buffer.dealloc(outputPtr);

        return output;
    }

    reverse_string(input: string): string {
        const inputPtr = this.buffer.from.string(input);

        const outputPtr = this.reverse(inputPtr);
        const output = this.buffer.to.string(outputPtr);

        this.buffer.dealloc(inputPtr);
        this.buffer.dealloc(outputPtr);

        return output;
    }

}
