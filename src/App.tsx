import { createSignal, createResource, Show } from 'solid-js';
import type { Component } from 'solid-js';

import { Wasmple } from './wasmple';

const App: Component = () => {
    const [wasmple] = createResource(async () => {
        const wasmple = await Wasmple.prelude();
        await new Promise((r) => setTimeout(r, 1000));
        return wasmple;
    });

    return (
        <>
            <div class="h-screen relative flex flex-col justify-center items-center">
                <div class="flex justify-center" classList={{
                    "animate-fadeOut animate-faster": wasmple() !== undefined,
                    "opacity-100": wasmple() === undefined,
                }}>
                    <span class="circle animate-loader" />
                    <span class="circle animate-loader animate-delay-[200ms]" />
                    <span class="circle animate-loader animate-delay-[400ms]" />
                </div>

                <div class="absolute inset-0 flex flex-col justify-center items-center" classList={{
                    "animate-fadeIn animate-faster": wasmple() !== undefined,
                    "opacity-0": wasmple() === undefined,
                }}>
                    <Show when={wasmple()}>
                        <button
                            class="px-3 py-2 rounded-md bg-blue-500 hover:bg-blue-600 active:bg-blue-700 font-mono text-xl"
                            onclick={() => wasmple().hello()}
                        >
                            say hello
                        </button>
                    </Show>
                </div>
            </div>
        </>
    );
};

export default App;
