import { Button } from "sveltestrap";
import ButtonPanel from "./ButtonPanel";



export default class PanelController
{
    public get activePanel() { return this.active; }
    public readonly panels: ButtonPanel[];

    private active: ButtonPanel;

    public constructor(...panels: ButtonPanel[])
    {
        if(panels.length < 1) {
            throw new Error("Cannot create a PanelController with 0 ButtonPanels");
        }
        this.panels = panels;
        this.active = panels[0];
    }

    public static default(): PanelController { return new PanelController(ButtonPanel.default()); }



    public changePanel(panel: ButtonPanel) {
        if(!this.panels.includes(panel)) { this.panels.push(panel); }
        this.active = panel;
    }
}
