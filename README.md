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
| Amstrad CPC                 | ❌        |
| Apple II                    | ❌        |
| Arcade                      | ❌        |
| Arcadia 2001                | ❌        |
| Arduboy                     | ❌        |
| Atari 2600                  | ❌        |
| Atari 7800                  | ❌        |
| Atari Jaguar                | ❌        |
| Atari Jaguar CD             | ❌        |
| Atari Lynx                  | ❌        |
| ColecoVision                | ❌        |
| Dreamcast                   | ❌        |
| Elektor TV Games Computer   | ❌        |
| Fairchild Channel F         | ❌        |
| Famicom Disk System         | ❌        |
| Game Boy                    | ✅        |
| Game Boy Advance            | ✅        |
| Game Boy Color              | ✅        |
| Game Gear                   | ❌        |
| GameCube                    | ❌        |
| Genesis/Mega Drive          | ❌        |
| Intellivision               | ❌        |
| Interton VC 4000            | ❌        |
| Magnavox Odyssey 2          | ❌        |
| Master System               | ❌        |
| Mega Duck                   | ❌        |
| MSX                         | ❌        |
| NES/Famicom                 | ❌        |
| Neo Geo CD                  | ❌        |
| Neo Geo Pocket              | ❌        |
| Nintendo 64                 | ❌        |
| Nintendo DS                 | ✅        |
| Nintendo DSi                | ❌        |
| PC Engine CD/TurboGrafx-CD  | ❌        |
| PC Engine/TurboGrafx-16     | ❌        |
| PC-8000/8800                | ❌        |
| PC-FX                       | ❌        |
| PlayStation                 | ❌        |
| PlayStation 2               | ❌        |
| PlayStation Portable        | ❌        |
| Pokemon Mini                | ❌        |
| Saturn                      | ❌        |
| Sega CD                     | ❌        |
| SG-1000                     | ❌        |
| SNES/Super Famicom          | ✅        |
| Standalone                  | ❌        |
| Uzebox                      | ❌        |
| Vectrex                     | ❌        |
| Virtual Boy                 | ❌        |
| WASM-4                      | ❌        |
| Watara Supervision          | ❌        |
| WonderSwan                  | ✅        |