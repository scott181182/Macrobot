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
    public deletePanel(panel: ButtonPanel) {
        if(this.panels.length <= 1) { return console.log("Cannot remove the last button panel."); }
        const index = this.panels.indexOf(panel);
        if(this.active === panel) {
            this.active = index > 0 ? this.panels[index - 1] : this.panels[index + 1];
        }
        this.panels.splice(index, 1);
    }
}
