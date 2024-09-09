use cursive::Cursive;
use cursive::views::{TextView, Dialog};

fn main() {
    let mut siv = cursive::default();
    cursive::logger::init();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(Dialog::text("This is a survey!")
        .title("title")
        .button("Next", show_next));

    siv.run();

    println!("Cursive done");
}

fn show_next(s: &mut Cursive) {
    // s.pop_layer();
    s.add_layer(
        Dialog::text("A question")
        .title("Question 1")
        .button("Yes!", |s| show_answer(s, "WD"))
        .button("No!", |s| show_answer(s, "Meh"))
        .button("No!", |s| s.add_layer(Dialog::info("Try again")))
    )
}

fn show_answer(s: &mut Cursive, msg: &str) {
    // s.pop_layer();
    s.add_layer(Dialog::text(msg)
    .title("Results")
    .button("Finish", |s| s.quit())
    );
}