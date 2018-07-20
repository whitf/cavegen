package main

import (
	"log"
)

type Floor struct {
	Id int8

	Entrance *Location
	Exit *Location

	Up []*Location
	Down []*Location

	Wall *[]Location

	PX int64
	NX int64
	PY int64
	NY int64
	PZ int64
	NZ int64

}

func (f *Floor) gen() int {
	log.Println("starting floor generation for level ", f.Id)

	// Create a "tree" starting at Entrance (if set) or from each "Down".
	// Start from each "Down" as that will correspond to an incoming "Up".
	// Any floor to be generated should have an Entrance and/or a Down value set.

	if nil != f.Entrance {
		log.Println("starting from f.Entrance ", f.Entrance)
	} else if nil != f.Down {
		log.Println("starting cavegen from f.Down")
	} else if nil != f.Up {
		log.Println("generating bottom floor")
	} else {
		//log.Println("no valid starting points for floor level ", f.Id)
	}


	log.Println(" done")
	return 0
} 