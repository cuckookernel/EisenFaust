import os
from textual.app import App, ComposeResult, Widget
from textual.widgets import Header, Input, Label, Static, Footer, \
    ListView, ListItem
from textual.containers import Container, Vertical, Horizontal
from textual.events import Event, Key, Hide, Show

CMD_GROUPS = [
    "shell",
    "git",
    "eks",
    "aws"
]


# Stolen from textual/src/textual/demo.py
class Sidebar0(Container):
    def compose(self) -> ComposeResult:
        yield Static("Textual Demo")
        #  yield OptionGroup(Message(MESSAGE), Version())
        # yield DarkSwitch()


class CmdGroupsSidebar(ListView):
    def __init__(self, *labels):
        self.labels = labels
        self.choices = ([Static("cmd-groups cmd-groups cmd-groups")]
                        + [ListItem(Label(label)) for label in labels])

        super().__init__(*self.choices, id="cmd-groups")
        self.add_class('-hidden')

    # def on_key(self, event: Key):
    #     match event.key:
    #         case "enter":
    #             print("Cmd Group", self.labels[self.index])
    #             st1: Static = self.parent.query_one("#first")
    #             st1.update(self.labels[self.index])
    #     print(f"CmdGroupsSelector: {event} (type {type(event)})")


class CmdTmplsSidebar(ListView):
    def __init__(self, *tmpls, **kwargs):
        self.tmpls = tmpls
        self.buttons = [ListItem(Label(tmpls)) for tmpls in tmpls]

        super().__init__(*self.buttons, id="cmd-tmpls")
        self.add_class('-hidden')

    # def on_key(self, event: Key):
    #    match event.key:
    #        case "enter":
    #            print("Cmd Tmpl", self.tmpls[self.index])
    #            st1: Static = self.parent.query_one("#first")
    #            st1.update(self.tmpls[self.index])
    #    print(f"CmdTmplsSideBar: {event} (type {type(event)})")


class EisenFaust(App):
    TITLE = "EisenFaust"
    CSS_PATH = "main.css"

    BINDINGS = [("f1", "toggle_cmd_groups_sidebar", "Cmd Groups"),
                ("f2", "toggle_cmd_tmpls_sidebar", "Cmd Tmpls")]

    def on_key(self, event) -> None:
        # self.title = "after KEY"
        print(f"pressed: {event.key}")

    def compose(self) -> ComposeResult:
        # yield Header()
        cmd_group_labels = [f"{i:2d} [b]{cmd_group}[/b]"
                            for i, cmd_group in enumerate(CMD_GROUPS)]

        cmd_tmpls = ["ls -l", "mkdir ...", "rm ..."]

        print("In App compose")

        yield Horizontal(
            # Vertical(
            #    *[Static(f"{i:2d} [b]{cmd_group}[/b]") for i, cmd_group in enumerate(CMD_GROUPS)],
            #    id="left-pane",
            # ),
            # ListView(
            #    *[ListItem(Label(f"{i:2d} [b]{cmd_group}[/b]", classes="gmd_group_label"))
            #      for i, cmd_group in enumerate(CMD_GROUPS)],
            #    id="left-pane"
            # ),
            Vertical(
                Static(os.getcwd(), id="top-info-bar"),
                Static("Horizontally", id="output"),
                Static("template here - template here - template here", id="template-bar"),
                id="right-pane",
            ),
            CmdGroupsSidebar(*cmd_group_labels),
            CmdTmplsSidebar(*cmd_tmpls),
            id="app-grid",
        )

        yield Footer()

        # self.input1.on_change = lambda value: self.label1.update(self.input1.value)

    def on_button_pressed(self) -> None:
        self.exit()

    def action_toggle_cmd_groups_sidebar(self) -> None:
        sidebar = self.query_one(CmdGroupsSidebar)
        self.toggle_visibility(sidebar)

    def action_toggle_cmd_tmpls_sidebar(self) -> None:
        sidebar = self.query_one(CmdTmplsSidebar)
        self.toggle_visibility(sidebar)

    def on_list_view_selected(self, event: Event):
        sender: Widget = event.sender
        print(f"List view selected: {event}, id={sender.id}")
        match sender.id:
            case "cmd-groups":
                # sender.set_focus(None)
                self.hide_and_defocus(sender)
                cmd_tmpls = self.query_one("#cmd-tmpls")
                self.show_and_focus(cmd_tmpls)

            case "cmd-tmpls":
                # sender.set_focus(None)
                tmpl_bar = self.query_one("#template-bar")
                tmpl_bar.update( sender.tmpls[sender.index] )
                self.hide_and_defocus(sender)
                # self.hide_and_defocus(tmpl_bar)

    def show_and_focus(self, widget: Widget, verbose=True):
        self.set_focus(None)
        widget.remove_class("-hidden")
        widget.focus()
        if verbose:
            print(f"show_and_focus: {widget}\n{widget.styles}")

    def hide_and_defocus(self, widget: Widget, verbose=True):
        self.set_focus(None)
        if widget.query("*:focus"):
            self.screen.set_focus(None)
        widget.add_class("-hidden")
        if verbose:
            print(f"hide_and_defocus: {widget}\n{widget.styles}")

    def toggle_visibility(self, widget: Widget):
        self.set_focus(None)
        if widget.has_class("-hidden"):
            self.show_and_focus(widget, verbose=False)
            print(f"toggle_visibility hidden->show widget: {widget}\n{widget.styles}")

        else:
            self.hide_and_defocus(widget, verbose=False)
            print(f"toggle_visibility hidden->show widget: {widget}")


if __name__ == "__main__":
    app = EisenFaust()
    app.run()
