<script lang="ts">
import * as svelte from "svelte";
import { fade } from "svelte/transition";

const baseSetContext = svelte.setContext;
const SvelteComponent = svelte.SvelteComponent;

export let key = "macromodal";
export let closeButton = true;
export let closeOnEsc = true;
export let closeOnOuterClick = true;
export let styleBg = { top: 0, left: 0 };
export let styleWindow = {};
export let styleContent = {};
export let styleCloseButton = {};
export let setContext = baseSetContext;
export let transitionBg = fade;
export let transitionBgProps = { duration: 250 };
export let transitionWindow = transitionBg;
export let transitionWindowProps = transitionBgProps;

const defaultState = {
    closeButton,
    closeOnEsc,
    closeOnOuterClick,
    styleBg,
    styleWindow,
    styleContent,
    styleCloseButton,
    transitionBg,
    transitionBgProps,
    transitionWindow,
    transitionWindowProps,
};

let state = { ...defaultState };
let Component: svelte.SvelteComponent | null = null;
let props = null;
let background: HTMLElement;
let wrap: HTMLElement;
let modalWindow: HTMLElement;
let clickTimeout: number;
let canOuterClick = false;

const camelCaseToDash = (str: string) => str.replace(/([a-zA-Z])(?=[A-Z])/g, "$1-").toLowerCase();
const toCssString = (props: Record<string, any>) => Object.keys(props).reduce((str, key) => `${str}; ${camelCaseToDash(key)}: ${props[key]}`, "");

// eslint-disable-next-line no-prototype-builtins
const isSvelteComponent = (component: any): component is svelte.SvelteComponent =>
    SvelteComponent &&
    SvelteComponent.isPrototypeOf &&
    SvelteComponent.isPrototypeOf(component);

$: cssBg                   = toCssString(state.styleBg);
$: cssWindow               = toCssString(state.styleWindow);
$: cssContent              = toCssString(state.styleContent);
$: cssCloseButton          = toCssString(state.styleCloseButton);
$: currentTransitionBg     = state.transitionBg;
$: currentTransitionWindow = state.transitionWindow;

type ModalCallback = () => void;
interface ModalCallbackOptions {
    onOpen?:   ModalCallback;
    onClose?:  ModalCallback;
    onOpened?: ModalCallback;
    onClosed?: ModalCallback;
};

const toVoid = () => {};
let onOpen   = toVoid;
let onClose  = toVoid;
let onOpened = toVoid;
let onClosed = toVoid;

const open = (NewComponent: svelte.SvelteComponent, newProps: any = {}, options: any = {}, callback: ModalCallbackOptions = {}) => {
    Component = NewComponent;
    props     = newProps;
    state     = { ...defaultState, ...options };
    onOpen    = callback.onOpen   || toVoid;
    onClose   = callback.onClose  || toVoid;
    onOpened  = callback.onOpened || toVoid;
    onClosed  = callback.onClosed || toVoid;

    // Avoid touch stutter
    clickTimeout = setTimeout(() => {
        canOuterClick = true;
        clickTimeout = undefined;
    }, 50);
};
const close = (callback: ModalCallbackOptions = {}) => {
    onClose   = callback.onClose  || onClose;
    onClosed  = callback.onClosed || onClosed;
    Component = null;
    props     = null;

    if(clickTimeout) { clearTimeout(clickTimeout); }
    clickTimeout = undefined;
    canOuterClick = false;
};
const handleKeydown = (event: KeyboardEvent) => {
    if (state.closeOnEsc && Component && event.key === "Escape") {
        event.preventDefault();
        close();
    }
    if (Component && event.key === "Tab") {
        // trap focus
        const nodes = modalWindow.querySelectorAll("*");
        const tabbable = Array.from(nodes).filter((node) => (node as HTMLElement).tabIndex >= 0);
        let index = tabbable.indexOf(document.activeElement);
        if (index === -1 && event.shiftKey) index = 0;
        index += tabbable.length + (event.shiftKey ? -1 : 1);
        index %= tabbable.length;
        (tabbable[index] as HTMLElement).focus();
        event.preventDefault();
    }
};
const handleOuterClick = (event: MouseEvent) => {
    if(state.closeOnOuterClick && canOuterClick && (event.target === background || event.target === wrap)) {
        event.preventDefault();
        close();
    }
};
setContext(key, { open, close });
</script>



<svelte:window on:keydown={handleKeydown} />

{#if Component}
    <div
            class="modal"
            role="dialog"
            tabindex="-1"
            on:click={handleOuterClick}
            bind:this={background}
            transition:currentTransitionBg={state.transitionBgProps}
            style="display: flex; background-color: #0008;">
        <!-- <div class="window-wrap" bind:this={wrap}> -->
            <div
                    class="modal-dialog"
                    style="margin: auto"
                    role="document"
                    aria-modal="true"
                    bind:this={modalWindow}
                    transition:currentTransitionWindow={state.transitionWindowProps}
                    on:introstart={onOpen}
                    on:outrostart={onClose}
                    on:introend={onOpened}
                    on:outroend={onClosed}>
                <!-- {#if state.closeButton}
                    {#if isSvelteComponent(state.closeButton)}
                        <svelte:component
                            this={state.closeButton}
                            onClose={close} />
                    {:else}
                        <button
                            on:click={close}
                            class="close"
                            style={cssCloseButton} />
                    {/if}
                {/if} -->
                <div class="modal-content" style={cssContent}>
                    <div class="modal-body">
                        <svelte:component this={Component} {...props}/>
                    </div>
                </div>
            </div>
        <!-- </div> -->
    </div>
{/if}
<slot />
