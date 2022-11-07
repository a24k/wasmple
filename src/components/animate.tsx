import type { Component, ParentComponent } from 'solid-js';

export const Fader: ParentComponent<{
    visible: boolean,
    class?: string,
}> = (props) => {
    let initial = props.visible;

    return (
        <div {...props} classList={{
            "opacity-100": initial && props.visible,
            "animate-fadeOut animate-faster": initial && !props.visible,
            "opacity-0": !initial && !props.visible,
            "animate-fadeIn animate-faster": !initial && props.visible,
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
