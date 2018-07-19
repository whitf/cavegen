package main

import (
	"strconv"
)


type Location struct {
	X int64
	Y int64
	Z int8
}

func (l *Location) String() string {
	ts := ""
	ts = "(" + strconv.FormatInt(l.X, 10) + "," + strconv.FormatInt(l.Y, 10) + "," + strconv.Itoa(int(l.Z)) + ")"
	return ts
}