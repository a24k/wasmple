import type { FnConsoleSetPanicHook } from '../../target/bridge-console';

import { LogLevel } from '../../target/bridge-console';
export { LogLevel };

export class WasmConsole {

    private memory?: WebAssembly.Memory;

    public init(wasm: WebAssembly.Exports) {
        this.memory = wasm.memory as WebAssembly.Memory;
        (wasm.console_set_panic_hook as FnConsoleSetPanicHook)();
    }

    public imports = {
        console_message: (level: LogLevel, ptr: number, len: number) => {
            if (this.memory == null) return;

            const chars = new Uint16Array(this.memory.buffer, ptr, len);
            const msg = String.fromCharCode(...chars);

            switch (level) {
                case LogLevel.Error:
                    console.error(msg); break;
                case LogLevel.Warn:
                    console.warn(msg); break;
                case LogLevel.Info:
                    console.info(msg); break;
                case LogLevel.Debug:
                    console.debug(msg); break;
                case LogLevel.Log:
                default:
                    console.log(msg); break;
            }
        },
    };

}
