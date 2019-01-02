package cavegen

import (
	"fmt"
)




type Cave struct {
	Width, Height, Levels int
	Map map[int]*Floor
}

type Floor struct {
	Width, Height int
	Layout [][]int
}

type Tile rune
const (
	SolidStone		Tile = 1000
	CaveWall		Tile = 2000
	DirtFloor		Tile = 3000
	Blank			Tile = 0
)


func Create(width int, height int, levels int) *Cave {

	c := &Cave{}

	c.Width = width
	c.Height = height
	c.Levels = levels

	c.Map = make(map[int]*Floor, levels)

	for i := 0; i < levels; i ++ {
		c.Map[i] = &Floor{}
		c.Map[i].Width = width
		c.Map[i].Height = height
		c.Map[i].createFloor()
	}

	return c
}

func (c *Cave) Generate() {


}

func (f *Floor) createFloor() {
	f.Layout = make([][]int, f.Width)
	for i := 0; i < f.Width; i++ {
		f.Layout[i] = make([]int, f.Height)

		for j := 0; j < f.Height; j++ {
			f.Layout[i][j] = 0
		}
	}

}

func (c *Cave) Draw() {
	fmt.Println("Cave: ", c.Width, " x ", c.Height, "(", c.Levels, " levels)")

	for l := 0; l < c.Levels; l++ {
		for x := 0; x < c.Width; x++ {
			for y := 0; y < c.Height; y++ {
				fmt.Print(c.Map[l].Layout[x][y], " ")
			}
			fmt.Println()
		}
	}

}

