package main

import (
	"log"
)

type Cave struct {
	Entrance *Location
	Exit *Location

	World map[int8]*Floor

	WorldItem map[Location]*Item
	WorldOre map[Location]*Ore

	Name string

	PX int64
	NX int64
	PY int64
	NY int64
	PZ int8
	NZ int8
}

func (c *Cave) genfull() *Cave {

	c.World = make(map[int8]*Floor)

	for i := c.NZ; i <= c.PZ; i++ {
		f := Floor {
			Id: i,
		}
		c.World[i] = &f
		c.World[i].genfloor()
	}

	log.Println(c)

	return c
}



