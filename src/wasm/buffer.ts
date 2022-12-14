import type { BufferPtr, FnBufferClear, FnBufferDealloc, FnBufferLength, FnBufferAlloc } from '../../target/bridge-buffer';
export type { BufferPtr };

import { T } from '../../target/bridge-buffer';
export { T };

export class WasmBuffer {

    private memory: WebAssembly.Memory;

    public alloc: FnBufferAlloc;
    public length: FnBufferLength;
    public dealloc: FnBufferDealloc;
    public clear: FnBufferClear;

    constructor(wasm: WebAssembly.Exports) {
        this.memory = wasm.memory as WebAssembly.Memory;

        this.alloc = wasm.buffer_alloc as FnBufferAlloc;
        this.length = wasm.buffer_length as FnBufferLength;
        this.dealloc = wasm.buffer_dealloc as FnBufferDealloc;
        this.clear = wasm.buffer_clear as FnBufferClear;
    }

    public slice = {
        i8: (ptr: BufferPtr): Int8Array => {
            return new Int8Array(this.memory.buffer, ptr, this.length(T.I8, ptr));
        },
        u8: (ptr: BufferPtr): Uint8Array => {
            return new Uint8Array(this.memory.buffer, ptr, this.length(T.U8, ptr));
        },
        i16: (ptr: BufferPtr): Int16Array => {
            return new Int16Array(this.memory.buffer, ptr, this.length(T.I16, ptr));
        },
        u16: (ptr: BufferPtr): Uint16Array => {
            return new Uint16Array(this.memory.buffer, ptr, this.length(T.U16, ptr));
        },
        i32: (ptr: BufferPtr): Int32Array => {
            return new Int32Array(this.memory.buffer, ptr, this.length(T.I32, ptr));
        },
        u32: (ptr: BufferPtr): Uint32Array => {
            return new Uint32Array(this.memory.buffer, ptr, this.length(T.U32, ptr));
        },
        i64: (ptr: BufferPtr): BigInt64Array => {
            return new BigInt64Array(this.memory.buffer, ptr, this.length(T.I64, ptr));
        },
        u64: (ptr: BufferPtr): BigUint64Array => {
            return new BigUint64Array(this.memory.buffer, ptr, this.length(T.U64, ptr));
        },
        f32: (ptr: BufferPtr): Float32Array => {
            return new Float32Array(this.memory.buffer, ptr, this.length(T.F32, ptr));
        },
        f64: (ptr: BufferPtr): Float64Array => {
            return new Float64Array(this.memory.buffer, ptr, this.length(T.F64, ptr));
        },
    };

    public from = {
        string: (str: string): BufferPtr => {
            const len = str.length; // number of UTF-16 code units
            const ptr = this.alloc(T.U16, len);

            const buf = this.slice.u16(ptr);
            for (let i = 0; i < len; ++i) { buf[i] = str.charCodeAt(i); }

            return ptr;
        },
        object: (obj: object): BufferPtr => {
            return this.from.string(JSON.stringify(obj));
        },
    };

    public to = {
        string: (ptr: BufferPtr): string => {
            const chars = this.slice.u16(ptr);
            return String.fromCharCode(...chars);
        },
        object: (ptr: BufferPtr): object => {
            return JSON.parse(this.to.string(ptr));
        },
    };

}
