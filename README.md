cavegen - simple cave generation

cavegen is written in Rust and can be built with the "cargo" command.

Building/running should be as simple as
>cargo run

You might need some additional graphics libraries (ie, SDL) installed that cargo won't grab.

Press "Esc" to pull up/hide the main menu, and use the corresponding keys (ie, 'N', 'Q', etc) to select menu options.

I didn't spend a lot of time on them.  The texture files are in assets/.  You can add new ones, and update the filename in cavegen.conf.toml, src/config.rs, and/cave/event.rs. Then update the "find_texture" mmethod in src/cave/tile.rs. 

