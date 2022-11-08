import type { Component } from 'solid-js';
import { createSignal, createMemo } from 'solid-js';

import type { FnConvertParameters, FnConvertReturns } from '../wasm/wasmple';
import { Wasmple } from '../wasm/wasmple';

export const Main: Component<{
    wasmple?: Wasmple,
}> = (props) => {
    const [inputA, setInputA] = createSignal("パトカー🚔");
    const [inputB, setInputB] = createSignal("タクシー🚖");

    const converted = createMemo<FnConvertReturns>(() =>
        props.wasmple === undefined
            ? { interleaved: "", reversed: "" } as FnConvertReturns
            : props.wasmple.convert_string({ a: inputA(), b: inputB() } as FnConvertParameters)
    );

    return (
        <div class="absolute inset-0 flex flex-col justify-center items-center gap-2">
            <div>
                <label for="input-a" class="block mb-1 text-md">input a</label>
                <input id="input-a" type="text" value={inputA()} onInput={(e) => setInputA(e.currentTarget.value)}
                    class="block w-96 p-2 bg-zinc-700 border border-zinc-500 rounded-md text-xl text-center focus:border-blue-500"
                />
            </div>
            <div>
                <label for="input-b" class="block mb-1 text-md">input b</label>
                <input id="input-b" type="text" value={inputB()} onInput={(e) => setInputB(e.currentTarget.value)}
                    class="block w-96 p-2 bg-zinc-700 border border-zinc-500 rounded-md text-xl text-center focus:border-blue-500"
                />
            </div>
            <div>
                <label for="output-interleaved" class="block mb-1 text-md">output (interleaved)</label>
                <input id="output-interleaved" type="text" value={converted().interleaved} disabled
                    class="block w-96 p-2 bg-zinc-700 border-none rounded-md text-xl text-center"
                />
            </div>
            <div>
                <label for="output-reversed" class="block mb-1 text-md">output (reversed)</label>
                <input id="output-reversed" type="text" value={converted().reversed} disabled
                    class="block w-96 p-2 bg-zinc-700 border-none rounded-md text-xl text-center"
                />
            </div>
        </div>
    );
};
