import type { WSClient } from "../api";


export default class MacroButton
{


    public constructor(
        public label: string,
        public macro: string
    ) {
        
    }

    public static default(): MacroButton
    {
        return new MacroButton("", "");
    }



    public execute(client: WSClient)
    {
        client.send(this.macro);
    }
}
