cavegen - simple cave generation

Building/running should be as simple as
>cargo run

Press "Esc" to pull up/hide the main menu, and use the corresponding keys (ie, 'N', 'Q', etc) to select menu options.

Q: Why are the textures so simple/plain?
A: I didn't spend a lot of time on them.  The texture files are in assets/.  You can add new ones, and update the filename in cavegen.conf.toml, src/config.rs, and/cave/event.rs. Then update the "find_texture" mmethod in src/cave/tile.rs. 

