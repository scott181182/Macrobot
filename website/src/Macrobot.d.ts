

declare namespace svelte.JSX {
    interface HTMLAttributes<T> {
        onshortclick?: (event: MouseEvent) => void;
        onlongclick?: (event: MouseEvent) => void;
    }
}