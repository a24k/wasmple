import wasmurl from '../../target/wasm32-unknown-unknown/wasmple.wasm?url';

import type { FnConvert, FnConvertParameters, FnConvertReturns } from '../../target/bridge';
export type { FnConvertParameters, FnConvertReturns };

import { WasmConsole } from './console';
import { WasmBuffer } from './buffer';

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

    convert_string(params: FnConvertParameters): FnConvertReturns {
        const inputJsonPtr = this.buffer.from.object(params);

        const outputJsonPtr = this.convert(inputJsonPtr);
        const outputJson = this.buffer.to.object(outputJsonPtr) as FnConvertReturns;

        this.buffer.dealloc(inputJsonPtr);
        this.buffer.dealloc(outputJsonPtr);

        return outputJson;
    }

}
