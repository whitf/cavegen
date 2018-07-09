package main

import (
	"log"
)





type Cave struct {
	Entrance *Location
	Exit *Location

	World map[uint64]*Location

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



