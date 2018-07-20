package main

import (
	"strconv"
)


type Location struct {
	X int
	Y int
	Z int8
}

func (l *Location) String() string {
	ts := ""
	ts = "(" + strconv.Itoa(l.X) + "," + strconv.Itoa(l.Y) + "," + strconv.Itoa(int(l.Z)) + ")"
	return ts
}