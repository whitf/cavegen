package caveui


import (
	"bufio"
	"cavegen"
	"fmt"
	"os"
	"image/png"
	"strconv"
	"strings"

	"github.com/veandco/go-sdl2/sdl"
)

type ui struct {
	winWidth int
	winHeight int
	window				*sdl.Window
	renderer 			*sdl.Renderer
	textureAtlas		*sdl.Texture
	textureIndex		map[int]sdl.Rect

}

func NewUI(c *cavegen.Cave) *ui {
	ui := &ui{}
	ui.winWidth = 1280
	ui.winHeight = 720

	window, err := sdl.CreateWindow("CaveGen!!", 1200, 400, int32(ui.winWidth), int32(ui.winHeight), sdl.WINDOW_SHOWN)
	if err != nil {
		panic(err)
	}

	ui.window = window
	ui.renderer, err = sdl.CreateRenderer(window, -1, sdl.RENDERER_ACCELERATED)
	if err != nil {
		panic(err)
	}

	sdl.SetHint(sdl.HINT_RENDER_SCALE_QUALITY, "1")

	ui.textureAtlas = ui.imgFileToTexture("ui/assets/tiles.png")
	ui.loadTextureIndex()

	return ui
}

func (ui *ui) loadTextureIndex() {
	ui.textureIndex = make(map[int]sdl.Rect)
	infile, err := os.Open("ui/assets/atlas-index.txt")
	if err != nil {
		panic(err)
	}

	scanner := bufio.NewScanner(infile)

	for scanner.Scan() {
		line := scanner.Text()
		line = strings.TrimSpace(line)

		var tileIndex int64
		tileIndex, err = strconv.ParseInt(line[:4], 10, 64)
		if err != nil {
			panic(err)
		}

		xy := line[4:]
		splitXY := strings.Split(xy, ",")

		x, err := strconv.ParseInt(strings.TrimSpace(splitXY[0]), 10, 64)
		if err != nil {
			panic(err)
		}

		y, err := strconv.ParseInt(strings.TrimSpace(splitXY[1]), 10, 64)
		if err != nil {
			panic(err)
		}

		rect := sdl.Rect{int32(x*32), int32(y*32), 32, 32}
		ui.textureIndex[int(tileIndex)] = rect
	}
}

func (ui *ui) imgFileToTexture(filename string) *sdl.Texture {
	infile, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer infile.Close()

	img, err := png.Decode(infile)
	if err != nil {
		panic(err)
	}

	w := img.Bounds().Max.X
	h := img.Bounds().Max.Y

	pixels := make([]byte, w*h*4)
	bIndex := 0
	for y := 0; y < h; y++ {
		for x := 0; x < w; x++ {
			r, g, b, a := img.At(x, y).RGBA()

			pixels[bIndex] = byte(r/256)
			bIndex++
			pixels[bIndex] = byte(g/256)
			bIndex++
			pixels[bIndex] = byte(b/256)
			bIndex++
			pixels[bIndex] = byte(a/256)
			bIndex++
		}
	}

	tex, err := ui.renderer.CreateTexture(sdl.PIXELFORMAT_ABGR8888, sdl.TEXTUREACCESS_STATIC, int32(w), int32(h))
	if err != nil {
		panic(err)
	}

	tex.Update(nil, pixels, w*4)

	err = tex.SetBlendMode(sdl.BLENDMODE_BLEND)
	if err != nil {
		panic(err)
	}

	return tex
}

func (ui *ui) Draw(f *cavegen.Floor) {

	ui.renderer.Clear()

	for y, row := range f.Layout {
		for x, i := range row {
			srcRect := ui.textureIndex[i]
			dstRect := sdl.Rect{int32(x*32), int32(y*32), 32, 32}

			//pos := cavegen.Pos{x, y}
			ui.textureAtlas.SetColorMod(255, 255, 255)
			ui.renderer.Copy(ui.textureAtlas, &srcRect, &dstRect)
		}
	}

	ui.renderer.Present()
}


func (ui *ui) Run(c *cavegen.Cave) {

	floor := c.Map[0]

	for {

		ui.Draw(floor)

		sdl.Delay(1000)
	}
}


func init() {
	fmt.Println("Initilize cavegen ui...")

	err := sdl.Init(sdl.INIT_EVERYTHING)
	if err != nil {
		panic(err)
	}
}