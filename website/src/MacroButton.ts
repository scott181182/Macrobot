import type { WSClient } from "./api";


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
        if(this.macro) {
            console.log(this.macro);
            client.send(this.macro);
        } else {
            console.log("Configure!");
        }
    }
}
