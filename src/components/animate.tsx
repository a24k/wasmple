import type { Component, ParentComponent } from 'solid-js';

export const Fader: ParentComponent<{
    visible: boolean,
    class?: string,
}> = (props) => {
    return (
        <div {...props} classList={{
            "animate-fadeIn animate-faster": props.visible,
            "animate-fadeOut animate-faster": !props.visible,
        }}>
            {props.children}
        </div>
    );
};

export const Loading: Component = () => {
    return (
        <div class="flex justify-center">
            <span class="circle animate-loader" />
            <span class="circle animate-loader animate-delay-[200ms]" />
            <span class="circle animate-loader animate-delay-[400ms]" />
        </div>
    );
};
