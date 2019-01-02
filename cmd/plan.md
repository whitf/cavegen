cavegen project - a simple 2d sub-rpg

## Goals ##
- procedural generations of multi-level caves
- 2d pixel graphics (from a tileset)
..- rock walls
..- rock floors
..- "stairs" of some sort to indicate up/down transitions between levels
- the player should be able to move around the map
..- movement with arrow keys, wasd, and the mouse
- it should be possible to save and load the state of the map (including the player and locations)
- the overall architecture should be flexible enough to use as much as possible in a more complicated project in the "worldgen" family





## Terms ##
- "Cave" : The world as a whole. A cave consists of a player and 1 (or more) levels generated with rocks walls and floors.
- "Level" : A 2d "floor" of the cave.
- "Player" : The player-controlled entity with the ability to move around the cave.