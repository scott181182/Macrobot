import MacroButton from "./MacroButton";



function makeGrid(rows: number, cols: number): MacroButton[][]
{
    const grid: MacroButton[][] = new Array(rows);
    for(let row = 0; row < rows; row++) {
        grid[row] = makeRow(cols);
    }
    return grid;
}
function makeRow(cols: number): MacroButton[]
{
    return new Array(cols).fill(0).map(() => MacroButton.default());
}



export default class ButtonPanel
{
    public constructor(
        public title: string,
        public buttons: MacroButton[][]
    ) {  }

    public static default(title?: string, rows = 3, columns = 4): ButtonPanel
    {
        return new ButtonPanel(title || "Default", makeGrid(rows, columns));
    }

    public reshape(rows: number, columns: number)
    {
        for(const index in this.buttons)
        {
            if(columns < this.buttons[index].length) {
                this.buttons[index] = this.buttons[index].slice(0, columns);
            } else if(columns > this.buttons[index].length) {
                this.buttons[index] = [ ...this.buttons[index], ...makeRow(columns - this.buttons[index].length) ];
            }
        }
        if(rows < this.buttons.length) {
            this.buttons = this.buttons.slice(0, rows);
        } else if(rows > this.buttons.length) {
            this.buttons = [ ...this.buttons, ...makeGrid(rows - this.buttons.length, columns) ];
        }
    }
}
