

export function clicker(node: HTMLElement, threshold = 500)
{
    let inClick = false;

    const handle_clickstart = () => {
        if(inClick) { return; }
        inClick = true;

        let didLong = false;

        const timeout = setTimeout(() => {
            node.dispatchEvent(new CustomEvent("longclick"));
            didLong = true;
        }, threshold);

        const cancel = () => {
            clearTimeout(timeout);
            node.removeEventListener("mousemove", cancel);
            node.removeEventListener("mouseup",   cancel);

            node.removeEventListener("touchmove",   cancel);
            node.removeEventListener("touchend",    cancel);
            node.removeEventListener("touchcancel", cancel);

            if(!didLong) {
                node.dispatchEvent(new CustomEvent("shortclick"));
            }
            inClick = false;
        };

        node.addEventListener("mousemove", cancel);
        node.addEventListener("mouseup",   cancel);

        node.addEventListener("touchmove",   cancel);
        node.addEventListener("touchend",    cancel);
        node.addEventListener("touchcancel", cancel);
    }

    node.addEventListener("mousedown",  () => handle_clickstart());
    node.addEventListener("touchstart", () => handle_clickstart());

    return {
        destroy() {
            node.removeEventListener("mousedown",  handle_clickstart);
            node.removeEventListener("touchstart", handle_clickstart);
        }
    };
}
