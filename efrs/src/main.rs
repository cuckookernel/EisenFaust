use cursive::views::{TextView, LinearLayout, SelectView, ResizedView};
use cursive::view::{Scrollable, SizeConstraint};
use cursive::event::Event;
use cursive::theme::{Theme, Palette};
use cursive::traits::*;

fn main() {

    let theme = Theme{shadow: false, ..Theme::default()};

    let mut siv = cursive::default();

    // You can load a theme from a file at runtime for fast development.
    // siv.load_theme_file("examples/assets/style.toml").unwrap();

    // Or you can directly load it from a string for easy deployment.
    // siv.load_toml(include_str!("assets/style.toml")).unwrap();

    let mut toolbar = make_toolbar().with_name("toolbar");

    let output_pane =
        TextView::new("output\ncomes\nhere")
        .with_name("output-pane")
        .fixed_size((10i32, 5i32)).scrollable();

    let main_screen = LinearLayout::horizontal()
                .child(toolbar)
                .child(output_pane);

    siv.add_layer(main_screen);

    siv.add_global_callback('~', cursive::Cursive::toggle_debug_console);

    siv.add_global_callback(Event::CtrlChar('g'), |s|{
        s.call_on_name(
            "toolbar",
        |vw: &mut ResizedView<SelectView>| {
            let rsz =  vw.with_view_size();
            if  > 0 {
                vw.set_width(SizeConstraint::AtMost(0))
            } else {

            }
        });
    });

    /*
       .title("Themed dialog")
       .button("Oh rly?", |_| ())
       .button("Quit", |s| s.quit()),
    ) */

    siv.run();
}


fn make_toolbar() -> ResizedView<SelectView> {
    let named = SelectView::new()
    // .with_name("picker")
    ;

    named
    .item("git", "git".to_string())
    .item("shell", "shell".to_string())
    .fixed_size((16, 50))
}