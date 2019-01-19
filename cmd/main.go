package main

import (
	"cavegen"
	"caveui"
	"fmt"
)

func main() {
	fmt.Println("Starting cavegen...")

	c := cavegen.Create(20,20,1)
	c.Generate()

	fmt.Println(c.ToString())

	fmt.Println("Starting ui...")

	ui := caveui.NewUI(c)
	ui.Run(c)	
}