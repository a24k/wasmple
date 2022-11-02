export enum Type {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
}

export type BufferPtr = number;

export type TypedArray = Int8Array | Uint8Array | Int16Array | Uint16Array | Int32Array | Uint32Array | BigInt64Array | BigUint64Array | Float32Array | Float64Array;

type FnAlloc = (t: Type, len: number) => number;
type FnLength = (t: Type, ptr: BufferPtr) => number;
type FnDealloc = (ptr: BufferPtr) => void;
type FnClear = () => void;

export class WasmBuffer {

    private memory: WebAssembly.Memory;

    public alloc: FnAlloc;
    public length: FnLength;
    public dealloc: FnDealloc;
    public clear: FnClear;

    constructor(wasm: WebAssembly.Exports) {
        this.memory = wasm.memory as WebAssembly.Memory;

        this.alloc = wasm.buffer_alloc as FnAlloc;
        this.length = wasm.buffer_length as FnLength;
        this.dealloc = wasm.buffer_dealloc as FnDealloc;
        this.clear = wasm.buffer_clear as FnClear;
    }

    public slice(t: Type, ptr: BufferPtr): TypedArray {
        switch (t) {
            case Type.I8:
                return new Int8Array(this.memory.buffer, ptr, this.length(t, ptr));
            case Type.U8:
                return new Uint8Array(this.memory.buffer, ptr, this.length(t, ptr));
            case Type.I16:
                return new Int16Array(this.memory.buffer, ptr, this.length(t, ptr));
            case Type.U16:
                return new Uint16Array(this.memory.buffer, ptr, this.length(t, ptr));
            case Type.I32:
                return new Int32Array(this.memory.buffer, ptr, this.length(t, ptr));
            case Type.U32:
                return new Uint32Array(this.memory.buffer, ptr, this.length(t, ptr));
            case Type.I64:
                return new BigInt64Array(this.memory.buffer, ptr, this.length(t, ptr));
            case Type.U64:
                return new BigUint64Array(this.memory.buffer, ptr, this.length(t, ptr));
            case Type.F32:
                return new Float32Array(this.memory.buffer, ptr, this.length(t, ptr));
            case Type.F64:
                return new Float64Array(this.memory.buffer, ptr, this.length(t, ptr));
            default:
                return new Uint8Array(this.memory.buffer, ptr, this.length(t, ptr));
        }
    }

    public from = {
        string: (str: string): BufferPtr => {
            const len = str.length; // number of UTF-16 code units
            const ptr = this.alloc(Type.U16, len);

            const buf = this.slice(Type.U16, ptr) as Uint16Array;
            for (let i = 0; i < len; ++i) { buf[i] = str.charCodeAt(i); }

            return ptr;
        },
    };

    public to = {
        string: (ptr: BufferPtr): string => {
            const chars = this.slice(Type.U16, ptr) as Uint16Array;
            return String.fromCharCode(...chars);
        }
    };

}
