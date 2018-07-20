package main

/**
 * cavegen.go
 * General datastructures and drive code.
 *
 * https://felotter.com/jira/cavgen
 */

import (
 	"flag"
	"log"
	"os"
	"runtime"
	"runtime/pprof"
	//"time"
)

func cavegen(n string, x, y int, z int8) *Cave {
	c := Cave {
		//World := new(map[int8]*Floor),
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



var cpuprofile = flag.String("cpuprofile", "", "write cpu profile to `file`")
var memprofile = flag.String("memprofile", "", "write memory profile to `file`")

func main() {
	flag.Parse()
	if *cpuprofile != "" {
		f, err := os.Create(*cpuprofile)
		if err != nil {
			log.Fatal("could not create CPU profile: ", err)
		}
		if err := pprof.StartCPUProfile(f); err != nil {
			log.Fatal("could not start CPU profile: ", err)
		}
		defer pprof.StopCPUProfile()
	}

	log.Println("starting cave-gen...")

	c := cavegen("simpleCave1", 10000, 10000, 4)
	c.gen()

	if *memprofile != "" {
		f, err := os.Create(*memprofile)
		if err != nil {
			log.Fatal("could not create memory profile: ", err)
		}
		runtime.GC()
		if err := pprof.WriteHeapProfile(f); err != nil {
			log.Fatal("could not write memory profile: ", err)
		}
		f.Close()
	}

	//log.Println("napping...")
	//time.Sleep(time.Second * 10)
	log.Println(" done")
	
}