use std::collections::HashMap;

use bitflags::bitflags;
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers as KM};
use crate::{app_util::StatefulList, exec_cell::ExecCell};

pub enum Screen {
    Commander,
    LogMessages,
    // TODO: Help,
}

static CMDS: &[(&str, &[&str]) ] = &[
    ("shell", &[
        "ls",
        "pwd"
        ]),
    ("git", &[
        "git status"
        ]),
    ("aws", &[
        "s3 ls $ARG1",
        "s3 cp $ARG1 $ARG2"
    ])
];

bitflags! {
    pub struct WhatToShow:u8 {
        const CmdGroups = 0b0000_0001;
        const CmdPicker = 0b0000_0010;
        const ArgSetter = 0b0000_0100;
    }
}

pub enum FocusElem {
    CmdGroups,
    CmdPicker,
    CmdRunner,
    ArgSetter,
    ExecPanel,
    ExecCell(usize),
}


pub struct App<'a> {
    pub title: &'a str,
    pub active_screen: Screen,
    pub to_show: WhatToShow,
    pub focus_elem: FocusElem,
    pub cmd_groups: StatefulList<String>,
    pub cmd_groups_map: HashMap<String, Vec<String>>,
    pub cmd_picker: StatefulList<String>,
    // pub tabs: TabsState<'a>,
    pub exec_cells: Vec<ExecCell>,
    pub msgs: StatefulList<String>,
    // pub progress: f64,
    // pub sparkline: Signal<RandomSignal>,
    // pub tasks: StatefulList<&'a str>,
    // pub logs: StatefulList<(&'a str, &'a str)>,
    // pub signals: Signals,
    // pub barchart: Vec<(&'a str, u64)>,
    // pub servers: Vec<Server<'a>>,
    pub enhanced_graphics: bool,
    pub should_quit: bool
}


impl<'a> App<'a> {
  pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {

    let cmd_groups0: Vec<_> = CMDS.iter().map(|(x, _)| {(*x).to_owned()}).collect();
    let cmd_groups_map: HashMap<String, Vec<String>> =
        CMDS.iter().map( |(x, cmds)| {
            let key = (*x).to_owned();
            let v0 = *cmds;
            let v: Vec<String> = v0.iter().map(|elem| { (*elem).to_owned()}).collect();

            (key, v)
        }).collect();

    App {
        title,
        to_show: WhatToShow::CmdGroups,
        focus_elem: FocusElem::CmdGroups,
        active_screen: Screen::Commander,
        cmd_groups: StatefulList::with_items(cmd_groups0),
        cmd_groups_map,
        cmd_picker: StatefulList::with_items(vec![]),
        exec_cells: vec![],
        enhanced_graphics,
        msgs: StatefulList::with_items([].to_vec()),
        should_quit: false
    }
  }

  pub fn on_key(&mut self, kev: KeyEvent) {
    if (kev.modifiers == KM::CONTROL) && (kev.code == KeyCode::Char('w')) {
        self.should_quit = true;
        return
    }

    if (kev.modifiers == KM::ALT) && (kev.code == KeyCode::Char('m')) {
        self.active_screen = Screen::LogMessages;
        return
    }

    if (kev.modifiers == KM::ALT) && (kev.code == KeyCode::Char('c')) {
        self.active_screen = Screen::Commander;
        return
    }

    match self.focus_elem {
        FocusElem::CmdGroups => {
            self.cmd_groups_on_key(kev)
        }
        FocusElem::CmdPicker => {
            self.cmd_picker_on_key(kev)
        }
        FocusElem::CmdRunner => {
            self.push_msg(&"!!Unimplemented on_key for cmd_runner")
        }
        FocusElem::ArgSetter => {
            self.push_msg(&"!!Unimplemented on_key for arg_setter")
        }
        FocusElem::ExecPanel => {
            self.push_msg(&"!!Unimplemented on_key for exec_panel")
        }
        FocusElem::ExecCell(_i) => {
            self.push_msg(&"!!Unimplemented on_key for exec_cell")
        }
    }
  }

  pub fn cmd_groups_on_key(&mut self, kev: KeyEvent) {
    match kev.code {
        KeyCode::Up => {
            self.cmd_groups.previous()
        },
        KeyCode::Down => {
            self.cmd_groups.next()
        },
        KeyCode::Enter => {
            if let Some(grp_idx) = self.cmd_groups.state.selected() {
                self.focus_elem = FocusElem::CmdPicker;
                self.push_msg(&"Cmd Picker gained focus");
                let cmd_group = &self.cmd_groups.items[grp_idx];
                self.cmd_picker.items = self.cmd_groups_map[cmd_group].clone();
                self.to_show.remove(WhatToShow::CmdGroups);
                self.to_show.insert(WhatToShow::CmdPicker);
            } else {
                self.push_msg(&"No grp_idx");
            }
        },
        _ => self.push_msg(&format!("Unhandled key event on StatefulList {:?}", kev))
    }
  }

  pub fn cmd_picker_on_key(&mut self, kev: KeyEvent) {
    match kev.code {
        KeyCode::Up => {
            self.cmd_picker.previous()
        },
        KeyCode::Down => {
            self.cmd_picker.next()
        },
        KeyCode::Enter => {
            if let Some(cmd_idx) = self.cmd_picker.state.selected() {
                self.focus_elem = FocusElem::ExecPanel;
                self.push_msg(&"Exec Panel gained focus");
                self.to_show.remove(WhatToShow::CmdPicker);
            } else {
                self.push_msg(&"No cmd_idx");
            }
        },
        _ => self.push_msg(&format!("Unhandled key event on StatefulList {:?}", kev))
    }
  }

  pub fn push_msg(&mut self, msg: &dyn AsRef<str>)
  {
    self.msgs.items.push(msg.as_ref().to_owned())
  }

  /*
  pub fn on_up(&mut self) {
    self.push_msg(&"Unhandled event: on_up");
  }
  pub fn on_right(&mut self) {
    // self.push_msg("Unhandled event: on_right");
    self.tabs.next()
  }

  pub fn on_down(&mut self) {
    let a = &"asd";
    self.push_msg(&"Unhandled event: on_down");
  }
    */


  pub fn on_tick(&mut self) {
    // eprintln!("Unhandled event: on_tick");
  }

}