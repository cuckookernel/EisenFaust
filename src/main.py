import os
from subprocess import PIPE, Popen
from typing import Dict, Optional

from textual.app import App, ComposeResult, List, Widget
from textual.containers import Horizontal, Vertical
from textual.events import Event
from textual.reactive import reactive
from textual.widgets import Footer, Header, Input, Label, ListItem, ListView, Static

# TODO / Ideas:
# Directory navigation functionality
# File preview


CMD_GROUPS = [
    ("shell", ["ls -l", "cd ..", "rm {arg1}", "mkdir {arg1}"]),
    ("git", ["git status", "git branch", "git add {arg1}"]),
    ("eks", ["kubectl get pods"]),
    ("aws", ["aws sso login {arg1}"]),
]


def show_and_focus(self, widget: Widget, verbose=True):
    if hasattr(self, 'set_focus'):
        self.set_focus(None)
    widget.remove_class("-hidden")
    widget.focus()
    if verbose:
        print(f"show_and_focus: {widget}")


def hide_and_defocus(self, widget: Widget, verbose=True):
    if hasattr(self, 'set_focus'):
        self.set_focus(None)

    if widget.query("*:focus"):
        self.screen.set_focus(None)

    widget.add_class("-hidden")
    if verbose:
        print(f"hide_and_defocus: {widget}")


def show_and_hide(self, to_show: List[Widget], to_hide: List[Widget]):
    for widget in to_show:
        show_and_focus(self, widget)

    for widget in to_hide:
        hide_and_defocus(self, widget)


def toggle_visibility(self, widget: Widget):
    if widget.has_class("-hidden"):
        show_and_focus(self, widget)
    else:
        hide_and_defocus(self, widget)


class SidebarContainer(Horizontal):

    def __init__(self):
        super().__init__(classes='-hidden')

        self.show_state = "hidden"  # hidden , "show-groups",
        cmd_group_names = [t[0] for t in CMD_GROUPS]
        self.cmd_group_labels = [f"{i:2d} [b]{cmd_group}[/b]"
                                 for i, cmd_group in enumerate(cmd_group_names)]

    def compose(self) -> ComposeResult:
        yield CmdGroupsSidebar(*self.cmd_group_labels)
        yield CmdTmplsPicker()

    def toggle_visibility_cmd_groups(self):
        if self.show_state == "hidden":
            show_and_hide(self.parent,
                          to_show=[self, self.query_one(CmdGroupsSidebar)],
                          to_hide=[self.query_one(CmdTmplsPicker)])
            self.show_state = "show-groups"
        else:
            self.hide()

    def toggle_visibility_cmd_tmpls(self):
        if self.show_state == "hidden":
            self.show_cmd_tmpls()
        else:
            self.hide()

    def show_cmd_tmpls(self):
        show_and_hide(self.parent,
                      to_show=[self, self.query_one(CmdTmplsPicker)],
                      to_hide=[self.query_one(CmdGroupsSidebar)])
        self.show_state = "show-tmpls"

    def hide(self):
        show_and_hide(self.parent, to_show=[], to_hide=[self])
        self.show_state = "hidden"


class ArgsForm(Vertical):
    def __init__(self):
        super().__init__(classes="-hidden")
        self.args: List[Optional[str]] = [None, None, None]

    def compose(self):
        for i, value in enumerate(self.args):
            yield Horizontal( Label(f"arg{i + 1}"), Input(value) )

    def as_dict(self) -> Dict[str, str]:
        return {f"arg{i + 1}": self.args[i] for i, val in enumerate(self.args)}


class CmdGroupsSidebar(ListView):
    def __init__(self, *labels):
        self.labels = labels
        self.choices = ([Header("Command Groups", id="cmd-groups-header")]
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


class CmdTmplsPicker(ListView):
    cmd_tmpls = reactive([])

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.add_class('-hidden')

    def watch_cmd_tmpls(self, old_tmpls: List[str], new_tmpls: List[str]):
        print(f"CmdTmplsPicker - watch_cmd_tmpls: old: {old_tmpls} new: {new_tmpls} "
              f"tmpls: {self.cmd_tmpls}")

        list_view = self.make_list()
        self.contents = list_view
        # self.update(self.make_list())

    # def render(self):
    #    return self.make_list().render()

    # def compose(self):
    #    yield self.make_list()

    def make_list(self):
        choices = [ListItem(Label(tmpl)) for tmpl in self.cmd_tmpls]
        return ListView(*choices)
    # def on_key(self, event: Key):
    #    match event.key:
    #        case "enter":
    #            print("Cmd Tmpl", self.tmpls[self.index])
    #            st1: Static = self.parent.query_one("#first")
    #            st1.update(self.tmpls[self.index])
    #    print(f"CmdTmplsSideBar: {event} (type {type(event)})")


class EisenFaust(App):
    TITLE = "EisenFaust"
    CSS_PATH = "main2.css"

    BINDINGS = [("f1", "toggle_cmd_groups_sidebar", "Cmd Groups"),
                ("f2", "toggle_cmd_tmpls_sidebar", "Cmd Tmpls"),
                ("f3", "toggle_args_form", "Args"),
                ("ctrl+f5", "run_command", "Run")]

    def compose(self) -> ComposeResult:
        # yield Header()

        print("In App compose")

        yield Horizontal(
            SidebarContainer(),
            Vertical(
                Vertical(
                    Static(os.getcwd()),
                    Static("tmpl-tmpl-tmpl", id="template-bar"),
                    id="top-info-bar",
                ),
                Static("\n" * 50, id="output"),
                id="right-pane",
            ),
            ArgsForm(),
        )

        yield Footer()

        # self.input1.on_change = lambda value: self.label1.update(self.input1.value)

    def on_key(self, event) -> None:
        # self.title = "after KEY"
        print(f"pressed: {event.key} event.sender={event.sender}")

        # if event.key == "enter" and event.sender == "asds":
    def action_run_command(self):
        # print(f"action_run_command - pressed: {event.key} event.sender={event.sender}")

        tmpl_widget = self.query_one(CmdTmplsPicker)
        tmpl = tmpl_widget.tmpls[tmpl_widget.index]
        print(f"value = {tmpl}")
        args_widget = self.query_one(ArgsForm)
        cmd = tmpl.format(**args_widget.as_dict())

        out_err = self.run_command_get_output(cmd)
        output_pane = self.query_one("#output")
        out_bytes = b"\n".join(out_err["out"])
        output_pane.update(out_bytes.decode("utf8"))

    def run_command_get_output(self, cmd: str) -> Dict[str, List[bytes]]:
        proc = Popen([cmd], stdout=PIPE, stderr=PIPE, shell=True)
        stdout, stderr = proc.communicate()
        print(stdout)
        return {"out": stdout.splitlines(), "err": stderr.splitlines()}

    def on_button_pressed(self) -> None:
        self.exit()

    def action_toggle_cmd_groups_sidebar(self) -> None:
        sbcontainer = self.query_one(SidebarContainer)
        sbcontainer.toggle_visibility_cmd_groups()

    def action_toggle_cmd_tmpls_sidebar(self) -> None:
        sbcontainer = self.query_one(SidebarContainer)
        sbcontainer.toggle_visibility_cmd_tmpls()

    def action_toggle_args_form(self) -> None:
        form = self.query_one(ArgsForm)
        toggle_visibility(self, form)

    def on_list_view_selected(self, event: Event):
        sender: Widget = event.sender
        print(f"List view selected: {event}, id={sender.id}")
        match sender.id:
            case "cmd-groups":
                cmd_tmpls_widget = self.query_one(CmdTmplsPicker)
                selected_tmpls = CMD_GROUPS[sender.index][1]
                cmd_tmpls_widget.cmd_tmpls = selected_tmpls
                print(f"selected group: {CMD_GROUPS[sender.index]}")
                print(f"cmd_tmpls_widget.cmd_tmpls: {cmd_tmpls_widget.cmd_tmpls}")

                # sbcontainer.render()
                sbcontainer = self.query_one(SidebarContainer)
                sbcontainer.show_cmd_tmpls()
            case "cmd-tmpls":
                tmpl_bar = self.query_one("#template-bar")
                tmpl_bar.update( sender.tmpls[sender.index] )
                sbcontainer = self.query_one(SidebarContainer)
                sbcontainer.hide()
                # self.hide_and_defocus(tmpl_bar)

    # def toggle_visibility(self, widget: Widget):
    #     self.set_focus(None)
    #     if widget.has_class("-hidden"):
    #         self.show_and_focus(widget, verbose=False)
    #         print(f"toggle_visibility hidden->show widget: {widget}\n{widget.styles}")
    #
    #     else:
    #         self.hide_and_defocus(widget, verbose=False)
    #         print(f"after toggle_visibility show->hidden: {widget}")


# if __name__ == "__main__":
app = EisenFaust()
#    app.run()
