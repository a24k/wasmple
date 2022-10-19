import { createSignal, createResource, Show } from 'solid-js';
import type { Component } from 'solid-js';

import { Wasmple } from './wasmple';

const App: Component = () => {
    const [wasmple] = createResource(async () => {
        const wasmple = await Wasmple.prelude();
        await new Promise((r) => setTimeout(r, 1000));
        return wasmple;
    });

    const [message, setMessage] = createSignal("か🦀に は💓と え🦐び");
    const reverseMessage = () => message().length == 0 ? "" : wasmple().reverse_string(message());

    return (
        <>
            <div class="h-screen relative transition duration-300 flex flex-col justify-center items-center"
            >
                <div class="transision duration-300"
                    classList={{ "opacity-0": wasmple() !== undefined, "opacity-100": wasmple() === undefined, }}
                >
                    <div class="flex justify-center">
                        <span class="circle animate-loader" />
                        <span class="circle animate-loader animation-delay-200" />
                        <span class="circle animate-loader animation-delay-400" />
                    </div>
                </div>

                <div class="absolute inset-0 transition duration-300 flex flex-col justify-center items-center gap-4"
                    classList={{ "opacity-100": wasmple() !== undefined, "opacity-0": wasmple() === undefined, }}
                >
                    <Show when={wasmple()}>
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
                    </Show>
                </div>
            </div>
        </>
    );
};

export default App;
