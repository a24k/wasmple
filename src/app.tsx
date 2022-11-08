import type { Component } from 'solid-js';
import { createResource } from 'solid-js';

import { Wasmple } from './wasm/wasmple';

import { Fader, Loading } from './components/animate';
import { Main } from './components/main';

export const App: Component = () => {
    const [wasmple] = createResource(async () => {
        const wasmple = await Wasmple.init();
        await new Promise((r) => setTimeout(r, 1000));
        return wasmple;
    });

    return (
        <div class="h-screen relative flex flex-col justify-center items-center">
            <Fader visible={wasmple() === undefined} >
                <Loading />
            </Fader>

            <Fader visible={wasmple() !== undefined}>
                <Main wasmple={wasmple()} />
            </Fader>
        </div>
    );
};
