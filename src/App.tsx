import { createSignal, createResource } from 'solid-js';
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

                <div class="absolute inset-0 transition duration-300 flex flex-col justify-center items-center"
                    classList={{ "opacity-100": wasmple() !== undefined, "opacity-0": !wasmple(), }}
                >
                    <div>
                        <button
                            class="mr-2 my-1 px-3 py-2 rounded-md font-medium text-sm text-white transition duration-300"
                        >
                            START
                        </button>
                        <button
                            class="mr-2 my-1 px-3 py-2 rounded-md font-medium text-sm text-white transision duration-300"
                        >
                            STOP
                        </button>
                    </div>
                </div>
            </div>
        </>
    );
};

export default App;
