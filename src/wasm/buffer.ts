export enum T {
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

type FnAlloc = (t: T, len: number) => number;
type FnLength = (t: T, ptr: number) => number;
type FnDealloc = (ptr: number) => void;
type FnClear = () => void;

export class Buffer {

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

    put_string(str: string): number {
        const len = str.length; // number of UTF-16 code units
        const ptr = this.alloc(T.U16, len);

        const buf = new Uint16Array(this.memory.buffer, ptr, len);
        for (let i = 0; i < len; ++i) { buf[i] = str.charCodeAt(i); }

        return ptr;
    }

    get_string(ptr: number): string {
        const len = this.length(T.U16, ptr);
        const chars = new Uint16Array(this.memory.buffer, ptr, len);
        return String.fromCharCode(...chars);
    }

}
