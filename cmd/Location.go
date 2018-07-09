package main


type Location struct {
	Id uint64

	Open bool
	Tile string

	X int64
	Y int64
	Z int64

	n *Location
	e *Location
	s *Location
	w *Location
	u *Location
	d *Location
}

type Loc struct {
	X int64
	Y int64
	Z int64
}

