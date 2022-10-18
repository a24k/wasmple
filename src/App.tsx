import { createResource, Show } from 'solid-js';
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
            <div class="h-screen relative transition duration-300 flex flex-col justify-center items-center"
            >
                <div class="transision duration-300"
                    classList={{ "opacity-0": wasmple() !== undefined, "opacity-100": wasmple() === undefined, }}
                >
                    <div class="flex justify-center">
                        <span class="circle animate-loader"></span>
                        <span class="circle animate-loader animation-delay-200"></span>
                        <span class="circle animate-loader animation-delay-400"></span>
                    </div>
                </div>

                <div class="absolute inset-0 transition duration-300 flex flex-col justify-center items-center gap-4"
                    classList={{ "opacity-100": wasmple() !== undefined, "opacity-0": wasmple() === undefined, }}
                >
                    <Show when={wasmple()}>
                        <button
                            class="px-3 py-2 rounded-md bg-blue-500 hover:bg-blue-600 active:bg-blue-700 font-mono text-xl"
                            onclick={() => wasmple().prepare_string_buffer("🦀 💓 🦐")}
                        >
                            prepare string buffer
                        </button>
                    </Show>
                </div>
            </div>
        </>
    );
};

export default App;
