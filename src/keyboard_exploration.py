import curses

from rich.console import Console
from rich.text import Text

# %%


def curses_example():
    stdscr = curses.initscr()
    curses.cbreak()
    stdscr.keypad(True)

    key = ''
    while key not in [ord('q'), 27]:  # 'q' key or Escape key
        key = stdscr.getch()
        stdscr.addch(0, 0, key)
        stdscr.refresh()

    curses.endwin()


def main(stdscr):
    # Set up the curses screen
    loop(stdscr)
    # %%


def main1():
    # Set up the Rich console
    # %%
    console = Console(record=True, force_terminal=True)

    # Define some rich text
    rich_text = Text("Hello, [bold magenta]Rich[/bold magenta] library "
                     "with [underline]curses[/underline]!")

    console.render(rich_text)
    a_str = console.export_text(styles=True)
    print(repr(a_str))
    # %%

K_ESC = 27


def loop(stdscr):
    # Main loop
    # curses.echo(True)
    curses.curs_set(1)

    stdscr.clear()

    count = 0

    with open("log.txt", "w") as f:
        while True:
            # Clear the screen
            stdscr.clear()

            # Get the dimensions of the terminal window
            height, width = stdscr.getmaxyx()

            # Render the rich text to a string
            # rendered_text = console.render_str(rich_text)  # , width=width - 2)
            # rendered_text_str = ''.join(rendered_text)

            # Print the rendered text to the curses screen
            # stdscr.addstr(1, 1, "Hello", curses.A_NORMAL)

            # Get a key press
            key = stdscr.getkey()
            count += 1
            f.write(f"{count}: {key!r} {type(key).__name__}\n")
            stdscr.addstr(2, 1, f"Key: {type(key).__name__}", curses.A_NORMAL)
            # Refresh the screen
            # stdscr.refresh()

            # Exit the loop on 'q' key
            if key == K_ESC:
                break


# Run the application
curses.wrapper(main)
# main1()
