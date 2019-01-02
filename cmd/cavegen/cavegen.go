package cavegen

import (
	"fmt"
	//"strconv"
	"strings"
)

type Cave struct {
	Width, Height, Levels int
	Map map[int]*Floor
}

type Floor struct {
	Layout [][]int
}

const (
	SolidStone		int = 1000
	CaveWall		int = 2000
	RockFloor		int = 3000
	Blank			int = 0
)

type Pos struct {
	X, Y int
}

func Create(width, height, levels int) *Cave {

	c := &Cave{}

	c.Width = width
	c.Height = height
	c.Levels = levels

	c.Map = make(map[int]*Floor, levels)

	for i := 0; i < levels; i ++ {
		c.Map[i] = &Floor{}
		c.Map[i].createFloor(width, height)
	}

	return c
}

// initiate a "blank" floor layout
func (f *Floor) createFloor(width, height int) {
	f.Layout = make([][]int, width)
	for i := 0; i < width; i++ {
		f.Layout[i] = make([]int, height)

		for j := 0; j < height; j++ {
			f.Layout[i][j] = RockFloor
		}
	}
}

func (f *Floor) generate(p Pos) {

	for i := 0; i < len(f.Layout); i++ {
		f.Layout[0][i] = CaveWall
		f.Layout[len(f.Layout)-1][i] = CaveWall
	}

	for i := 0; i < len(f.Layout[0]); i++ {
		f.Layout[i][0] = CaveWall
		f.Layout[i][len(f.Layout[0])-1] = CaveWall
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
	for i := 0; i < c.Levels; i++ {
		c.Map[i].generate(Pos{1,1})
	}
}

func (c *Cave) Load(id string) {}
func (c *Cave) Store() {}