import { createSignal, createResource, Show } from 'solid-js';
import type { Component } from 'solid-js';

import { Wasmple } from './wasmple';

const App: Component = () => {
    const [wasmple] = createResource(async () => {
        const wasmple = await Wasmple.prelude();
        await new Promise((r) => setTimeout(r, 1000));
        return wasmple;
    });

    const [left, setLeft] = createSignal(0);
    const [right, setRight] = createSignal(0);

    setInterval(() => setLeft(left() + 1), 110);
    setInterval(() => setRight(right() + 1), 190);

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

                <div class="absolute inset-0 transition duration-300 flex flex-col justify-center items-center"
                    classList={{ "opacity-100": wasmple() !== undefined, "opacity-0": wasmple() === undefined, }}
                >
                    <Show when={wasmple()}>
                        <p class="font-mono text-xl">
                            {left()} + {right()} = {wasmple().add(left(), right())}
                        </p>
                    </Show>
                </div>
            </div>
        </>
    );
};

export default App;
