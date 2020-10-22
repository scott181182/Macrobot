


export class WSClient
{
    public constructor(
        private readonly socket: WebSocket
    ) {
        socket.onmessage = (event) => { this.handleMessage(event); }
        socket.onerror = (event) => {
            console.error("Error on connected socket:");
            console.error(event);
        }
    }

    public static connect(endpoint: string): Promise<WSClient>
    {
        console.log("Connecting to WebSocket endpoint...");

        return new Promise((resolve, reject) => {
            const socket = new WebSocket(endpoint);

            socket.onerror = (event) => {
                console.error("WebSocket connection failed!");
                console.error(event);
                reject(event);
            };
            socket.onopen = () => {
                console.log("WebSocket connected!");
                socket.onerror = undefined;
                resolve(new WSClient(socket));
            };
        });
    }



    public send(message: string)
    {
        this.socket.send(message);
    }

    private handleMessage(event: MessageEvent<any>)
    {
        console.log(event.data);
    }
}
