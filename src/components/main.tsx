import type { Component } from 'solid-js';
import { createSignal } from 'solid-js';

import { Wasmple } from '../wasm/wasmple';

export const Main: Component<{
    wasmple?: Wasmple,
}> = (props) => {
    const [message, setMessage] = createSignal("か🦀に は💓と え🦐び");
    const reverseMessage = () => props.wasmple === undefined ? "" : props.wasmple.reverse_string(message());

    return (
        <div class="absolute inset-0 flex flex-col justify-center items-center gap-2">
            <div>
                <label for="message" class="block mb-1 text-md">input</label>
                <input id="message" type="text" value={message()} onInput={(e) => setMessage(e.currentTarget.value)}
                    class="block w-96 p-2 bg-zinc-700 border border-zinc-500 rounded-md text-xl text-center focus:border-blue-500"
                />
            </div>
            <div>
                <label for="message" class="block mb-1 text-md">output (reversed)</label>
                <input id="message" type="text" value={reverseMessage()} disabled
                    class="block w-96 p-2 bg-zinc-700 border-none rounded-md text-xl text-center"
                />
            </div>
        </div>
    );
};
