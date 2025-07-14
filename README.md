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