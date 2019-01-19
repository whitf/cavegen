package cavegen

import (
	"fmt"
	"math/rand"
	//"strconv"
	"strings"
	"time"
)

type Cave struct {
	Width, Height 					int
	Levels 							int
 	Map 							map[int]*Floor
 	Stairs 							map[StairKey]*Pos
	Entrance 						Pos
}

type Floor struct {
	Layout 				[][]int 
	Up 					Pos
	Down 				Pos
	Width 				int
	Height 				int
}

const (
	SolidStone		int = 1000
	CaveWall		int = 2000
	RockFloor		int = 3000
	Blank			int = 0
	StairDown		int = 8000
	StairUp			int = 8001
)

type Pos struct {
	X, Y int
}

type StairKey struct {
	Dir 			int 	// 8000 for Down, 8001 for Up (the same as the const for the UI).
	Floor 			int
}

func Create(width, height, levels int) *Cave {

	c := &Cave{}

	c.Width = width
	c.Height = height
	c.Levels = levels

	c.Map = make(map[int]*Floor, levels)
	c.Stairs = make(map[StairKey]*Pos, (levels-1)*2)

	for i := 0; i < levels; i ++ {
		c.Map[i] = &Floor{}
		c.Map[i].initialize(width, height)
	}

	return c
}

func (f *Floor) initialize(width, height int) {
	f.Width = width
	f.Height = height
	f.Layout = make([][]int, width)
	for i := 0; i < width; i++ {
		f.Layout[i] = make([]int, height)

		for j := 0; j < height; j++ {
			f.Layout[i][j] = Blank
		}
	}
}

func (f *Floor) generate(up, down *Pos) {
	// Add stairs
	if up.X >= 0 && up.Y >= 0 {
		f.Layout[up.X][up.Y] = StairUp
	}

	if down.X >= 0 && down.Y >= 0 {
		f.Layout[down.X][down.Y] = StairDown
	}

	// Start at stair down and generate to stair up.
}

func (f *Floor) rPos() *Pos {
	r := rand.New(rand.NewSource(time.Now().UnixNano()))

	return &Pos {
		X: r.Intn(f.Width),
		Y: r.Intn(f.Height),
	}
}

func (c *Cave) ToString() string {

	var cb strings.Builder
	fmt.Fprintf(&cb, "Cave: %d X %d with %d levels.\n", c.Width, c.Height, c.Levels)

	for l := 0; l < c.Levels; l++ {
		for x := 0; x < c.Width; x++ {
			for y := 0; y < c.Height; y++ {
				cb.WriteRune(rune(c.Map[l].Layout[x][y]))
				cb.WriteRune(' ')
			}
			cb.WriteRune('\n')
		}
	}

	return cb.String()
}

func (c *Cave) Generate() {
	// Generate a "list" of stairs "linking" floors together.
	c.Stairs[StairKey{0, StairDown}] = &Pos{-1, -1}
	c.Stairs[StairKey{0, StairUp}] = c.Map[0].rPos()
	
	for i := 1; i < c.Levels; i++ {

	}

	for i := 0; i < c.Levels; i++ {
		c.Map[i].generate(c.Stairs[StairKey{i, StairUp}], c.Stairs[StairKey{i, StairDown}])
	}
}

func (c *Cave) Load(id string) {}
func (c *Cave) Store() {}