**Disclaimer: This tool is not user friendly nor official. It's just a fun small pet project.**

# Setup

1. Download the [Latest Release](https://github.com/SiriusAshling/heros_spirit_utils/releases/latest). Choose the one that matches your operating system. There are no automatic updates, so check for updates if something doesn't work.
2. Locate your game installation folder. It should contain the `Hero's Spirit` executable and the `Roms/` folder.
3. Move the `heros_spirit_utils` executable into your game installation folder.
4. Move the `rando` folder containing `logic.json` into your game installation folder.

Now you can run `heros_spirit_utils` to do any of the below.
If something doesn't work, you may want to run it from a terminal to see any error messages.

# To use for randomization

## Generate seeds

Run `heros_spirit_utils` and choose `randomize` to create a new `Roms/randomizer.hsrom`.

To load the randomized rom, edit `rom.txt` in your game installation folder.
It should contain `main.hsrom` by default. Change its content to `randomizer.hsrom`.

To play vanilla again, change its content back to `main.hsrom`.

The game will read the rom when it starts, so you need to restart the game after changing the rom. Currently, only the "Heroic Hardcore" mode will be randomized.

# To use for ROM hacking

Run `heros_spirit_utils` and choose `export` or `export-rom` to export the rom. Files will be written into a folder `rom_files`.

Choose `import` or `import-rom` to import the rom. Your rom will be overwritten (best make a backup of the vanilla rom).

## Files you can edit

- `rom_files/Graphics/tile8.bmp`: Only use the four pixel colors already in the image, the game will color the tiles based on palette
- `rom_files/Maps/map*.tmx`: Use the Tiled editor to edit. More below
- `rom_files/Maps/Metadata/*`
- `rom_files/Audio/*`
- `rom_files/Shaders/*`
- `rom_files/Textures/*`
- `rom_files/Other/*`

## Files you can't edit

- `rom_files/Graphics/tile16`: These are just pieced together from the tile8 data for convenient viewing of what the tiles might look like in reality.
- `rom_files/Maps/images`: Rendered versions of the maps for convenience
- `rom_files/Maps/stats`: Collectible and Enemy statistics about the maps
- `completion/*`: Information about missing items from savefiles

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

The first item in the Sprites tileset, name `Other(0)`. Add 4 custom properties:

- `byte_1`: target map
- `byte_2`: target x
- `byte_3`: target y
- `byte_4`: face direction on exit (2, 4, 6 or 8)

#### **Silent in-map Warps**

The 16th item in the Sprites tileset, name `Other(15)`. Add 2 custom properties:

- `byte_1`: target x
- `byte_2`: target y

# To use for Save hacking

Put into a folder with your save files (in local app data).

Run `heros_spirit_utils` and choose `export` or `export-saves` to export the saves to json.

Choose `import` or `import-saves` to import the json with any edits, your saves will be overwritten.
