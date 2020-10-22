import MacroButton from "./MacroButton";



function makeGrid(rows: number, cols: number): MacroButton[][]
{
    const grid: MacroButton[][] = new Array(rows);
    for(let row = 0; row < rows; row++) {
        grid[row] = new Array(cols);
        for(let col = 0; col < cols; col++) {
            grid[row][col] = MacroButton.default();
        }
    }
    return grid;
}



export default class ButtonPanel
{
    public constructor(
        public readonly title: string,
        public readonly buttons: MacroButton[][]
    ) {  }


    public static default(title?: string): ButtonPanel
    {
        return new ButtonPanel(title || "Default", makeGrid(3, 4));
    }
}
