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

func (l *Location) String() string {
	ts := ""

	if l.Open {
		ts = "\r\n"
		ts += ".---N---.\r\n"
		ts += "'       '\r\n"
		ts += "W       E\r\n"
		ts += "'       '\r\n"
		ts += ".---S---.\r\n"
		ts += "\r\n"
	} else {
		ts = "Location " + "id will go here " + "is not open." 
	}

	return ts
}