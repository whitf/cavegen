package main

import (
	"log"
	"math/rand"
	"time"
)

type Cave struct {
	Entrance *Location
	Exit *Location

	World map[int8]*Floor

	WorldItem map[Location]*Item
	WorldOre map[Location]*Ore

	Name string

	PX int
	NX int
	PY int
	NY int
	PZ int8
	NZ int8
}

func (c *Cave) gen() int {

	c.World = make(map[int8]*Floor)

	for i := c.NZ; i <= c.PZ; i++ {
		f := Floor {
			Id: i,
		}
		c.World[i] = &f
	}

	c.entranceGen()
	log.Println("cave entrance at ", c.Entrance)

	c.World[c.Entrance.Z].Entrance = c.Entrance
	c.World[c.Entrance.Z].Up = make([]*Location, 1)
	c.World[c.Entrance.Z].Down = make([]*Location, 1)

	log.Println("linking cave floors...")
	c.link(c.Entrance.Z, 1)
	c.link(c.Entrance.Z, -1)
	log.Println(" done")

	for i := c.NZ; i <= c.PZ; i++ {
		c.World[i].gen()
	}

	log.Println(c)

	return 0
}

func (c *Cave) entranceGen() int {
	var x, y int = 0, 0
	r := rand.New(rand.NewSource(time.Now().UnixNano()))

	x = r.Intn(c.PX)
	y = c.NY

	if r.Float32() <= 0.5 {
		x = -1 * x
	}

	l := Location {
		X: x,
		Y: y,
		Z: int8(r.Intn(int(c.PZ))),
	}

	c.Entrance = &l
	return 0
}

func (c *Cave) link(z int8, i int8) int {
	if z >= c.PZ || z <= c.NZ {
		return 0
	}

	r := rand.New(rand.NewSource(time.Now().UnixNano()))
	x := r.Intn(c.PX)
	y := r.Intn(c.PY)

	if r.Float32() <= 0.5 {
		x = -1 * x
	}

	if r.Float32() <= 0.5 {
		y = -1 * y
	}

	p := Location{
		X: x,
		Y: y,
		Z: z,
	}

	pLink := Location{
		X: x,
		Y: y,
		Z: z + i,
	}

	if i < 0 {
		c.World[z].Down[0] = &p
		
		c.World[z+i].Up = make([]*Location, 1)
		c.World[z+i].Down = make([]*Location, 1)
		
		c.World[z+i].Up[0] = &pLink
	} else {
		c.World[z].Up[0] = &p
		
		c.World[z+1].Up = make([]*Location, 1)
		c.World[z+1].Down = make([]*Location, 1)

		c.World[z+i].Down[0] = &pLink
	}

	return c.link(z + i, i)
}


