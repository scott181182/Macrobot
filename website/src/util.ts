

export function clicker(node: HTMLElement, threshold = 500)
{
    const handle_mousedown = () => {
        let didLong = false;

        const timeout = setTimeout(() => {
            node.dispatchEvent(new CustomEvent("longclick"));
            didLong = true;
        }, threshold);

        const cancel = () => {
            clearTimeout(timeout);
            node.removeEventListener("mousemove", cancel);
            node.removeEventListener("mouseup", cancel);
            if(!didLong) {
                node.dispatchEvent(new CustomEvent("shortclick"));
            }
        };

        node.addEventListener("mousemove", cancel);
        node.addEventListener("mouseup", cancel);
    }

    node.addEventListener("mousedown", handle_mousedown);

    return {
        destroy() {
            node.removeEventListener("mousedown", handle_mousedown);
        }
    };
}
