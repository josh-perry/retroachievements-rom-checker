# retroachievements-rom-checker
A utility for scanning a folder of game ROMs and checks against the [RetroAchievements](https://retroachievements.org/) API.

This lets you know if a) a particular ROM has an achievement set and b) if the ROM you have is compatible by checking the RA hash.

## Config
To use the retroachievements API you must provide a web API key (found in your [settings](https://retroachievements.org/settings) whilst logged in). 

Create a config.toml with the following contents and fill in the API key and path:
```toml
# config.toml
api_key = "[your api key here]"
roms_folder = "/path/to/roms/root"
```

## Supported systems
| Name                        | Supported |
|-----------------------------|-----------|
| 32X                         | ❌        |
| 3DO Interactive Multiplayer | ❌        |
| Amiga                       | ❌        |
| Amstrad CPC                 | ❌        |
| Apple II                    | ❌        |
| Arcade                      | ❌        |
| Arcadia 2001                | ❌        |
| Arduboy                     | ❌        |
| Atari 2600                  | ❌        |
| Atari 5200                  | ❌        |
| Atari 7800                  | ❌        |
| Atari Jaguar                | ❌        |
| Atari Jaguar CD             | ❌        |
| Atari Lynx                  | ❌        |
| Atari ST                    | ❌        |
| Cassette Vision             | ❌        |
| ColecoVision                | ❌        |
| Commodore 64                | ❌        |
| DOS                         | ❌        |
| Dreamcast                   | ❌        |
| Elektor TV Games Computer   | ❌        |
| Events                      | ❌        |
| Fairchild Channel F         | ❌        |
| Famicom Disk System         | ❌        |
| FM Towns                    | ❌        |
| Game & Watch                | ❌        |
| Game Boy                    | ✅        |
| Game Boy Advance            | ✅        |
| Game Boy Color              | ✅        |
| Game Gear                   | ❌        |
| GameCube                    | ❌        |
| Genesis/Mega Drive          | ❌        |
| Hubs                        | ❌        |
| Intellivision               | ❌        |
| Interton VC 4000            | ❌        |
| Magnavox Odyssey 2          | ❌        |
| Master System               | ❌        |
| Mega Duck                   | ❌        |
| MSX                         | ❌        |
| NES/Famicom                 | ❌        |
| Neo Geo CD                  | ❌        |
| Neo Geo Pocket              | ❌        |
| Nintendo 3DS                | ❌        |
| Nintendo 64                 | ❌        |
| Nintendo DS                 | ✅        |
| Nintendo DSi                | ❌        |
| Nokia N-Gage                | ❌        |
| Oric                        | ❌        |
| PC Engine CD/TurboGrafx-CD  | ❌        |
| PC Engine/TurboGrafx-16     | ❌        |
| PC-6000                     | ❌        |
| PC-8000/8800                | ❌        |
| PC-9800                     | ❌        |
| PC-FX                       | ❌        |
| Philips CD-i                | ❌        |
| PlayStation                 | ❌        |
| PlayStation 2               | ❌        |
| PlayStation Portable        | ❌        |
| Pokemon Mini                | ❌        |
| Saturn                      | ❌        |
| Sega CD                     | ❌        |
| Sega Pico                   | ❌        |
| SG-1000                     | ❌        |
| Sharp X1                    | ❌        |
| Sharp X68000                | ❌        |
| SNES/Super Famicom          | ❌        |
| Standalone                  | ❌        |
| Super Cassette Vision       | ❌        |
| Thomson TO8                 | ❌        |
| TI-83                       | ❌        |
| TIC-80                      | ❌        |
| Uzebox                      | ❌        |
| Vectrex                     | ❌        |
| VIC-20                      | ❌        |
| Virtual Boy                 | ❌        |
| WASM-4                      | ❌        |
| Watara Supervision          | ❌        |
| Wii                         | ❌        |
| Wii U                       | ❌        |
| WonderSwan                  | ✅        |
| Xbox                        | ❌        |
| Zeebo                       | ❌        |
| ZX Spectrum                 | ❌        |
| ZX81                        | ❌        |
