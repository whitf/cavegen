package main

import (
	"log"
)





type Cave struct {
	Entrance *Location
	Exit *Location

	World map[Loc]*Location

	WorldItem map[Loc]*Item
	WorldOre map[Loc]*Ore


	Name string

	PX int64
	NX int64
	PY int64
	NY int64
	PZ int64
	NZ int64
}

func (c *Cave) genfull() *Cave {
	log.Println("creating basic location matrix...")

	c.World = make(map[Loc]*Location)

	// Create the "full" cave of potential "rooms".
	i := c.NZ
	for i <= c.PZ {
		j := c.NX
		for j <= c.PX {
			k := c.NY
			for k <= c.PY {
				l := Loc {
					Z: i,
					X: j,
					Y: k,
				}

				c.World[l] = &Location{
					Open: false,
					Tile: "closedroom.png",
					X: j,
					Y: k,
					Z: i,
				}

				k++
			}
			j++
		}
		i++
	}

	log.Println(" done")

	log.Println("generating cave structure...")
	log.Println(" done")

	log.Println("generating and loading items...")
	log.Println(" done")

	log.Println("loading npcs...")
	log.Println(" done")

	log.Println("initial ore spawn...")
	log.Println(" done")

	log.Println("returning a cave...")
	return c
}



