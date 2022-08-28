# To use for ROM hacking

**Disclaimer: This tool is not user friendly nor official. It's just a fun small pet project.**

Put into a folder with your rom (in the game files).
Pass "export" to export the rom. Files will be written into a folder "rom_files".

Pass "import" to import the rom. Your rom will be overwritten (best make a backup of the vanilla rom).

## Files you can edit
- rom_files/graphics/tile8/all.bmp: Only use the four pixel colors already in the image, the game will color the tiles based on palette
- rom_files/graphics/meow.bmp
- rom_files/graphics/rawr.bmp
- rom_files/graphics/winter.bmp: Used for subareas in the Underworld
- rom_files/maps/map\*\*.tmx: Use the Tiled editor to edit. More below
- rom_files/music/track\*\*.ogg
- rom_files/sounds/sfx\*\*.ogg
- rom_files/sounds/shaders/retrofx: OpenGL shader

## Files you can't edit
- rom_files/tile16: These are just pieced together from the tile8 data for convenient viewing of what the tiles might look like in reality.
- rom_files/maps/images: Just for viewing

## Editing maps

Only use the predefined Layers, Tilesets etc. Anything else cannot be imported.

### The Tiles Layer

Only use the Tiles tileset to draw on the Tiles layer. It includes everything that can be on the Tiles layer.

### The Sprites Layer

Only use the Sprites tileset to place on the Sprites layer. It includes everything that can be on the Sprites layer.

Some useful shortcuts in Tiled:
- T to place Sprites
- S to select and edit Sprites
- Ctrl to snap Sprites to the grid when placing them, otherwise they will be snapped to the grid on import.

Many sprites are invisible, you can export the vanilla rom to see what sprite does what.

For simple sprites like Swords or Wind Routes you can just place them. More complex sprites like Warps and Map transfers have data attached. The data format is individual for each sprite, you can export the vanilla rom to get ideas on how they work. Some useful ones:

#### **Map Transfers**

The first item in the Sprites tileset, name "Other(0)". Add 4 custom properties:
- "byte_1": target map
- "byte_2": target x
- "byte_3": target y
- "byte_4": face direction on exit (2, 4, 6 or 8)

#### **Silent in-map Warps**

The 16th item in the Sprites tileset, name "Other(15)". Add 2 custom properties:
- "byte_1": target x
- "byte_2": target y

# To use for Save hacking

Put into a folder with your save files (in local app data).

Pass "export" to export the saves to json.

Pass "import" to import the json with any edits, your saves will be overwritten.
