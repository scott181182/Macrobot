

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



    public execute()
    {
        if(this.macro) {
            console.log(this.macro);
        } else {
            console.log("Configure!");
        }
    }
}
