package main

/**
 * cavegen.go
 * General datastructures and drive code.
 *
 * https://felotter.com/jira/cavgen
 */

import (
	"log"
)

func cavegen(n string, x int64, y int64, z int64) *Cave {
	c := Cave {
		Name: n,
		PX: x,
		NX: -1 * x,
		PY: y,
		NY: -1 * y,
		PZ: z,
		NZ: -1 * z,
	}

	return &c
}




func main() {
	log.Println("starting cave-gen...")

	c := cavegen("simpleCave1", 1000, 10000, 25)
	c = c.genfull()

	l := Loc{
		X: 1,
		Y: 1,
		Z: 1,
	}

	//log.Println(c)
	log.Println("len(c.World)")
	log.Println(len(c.World))
	log.Println(c.World[l])
	
}