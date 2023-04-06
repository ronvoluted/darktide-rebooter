# Darktide Rebooter 🥾↪️🥾

Automatically start the game again (and bypass the launcher) whenever it crashes. There's heresy to be smote.

## Installation

Copy [`DarktideRebooter.exe`](https://github.com/ronvoluted/darktide-rebooter/releases/latest) to your Darktide folder. By default:

**Steam**
```
C:\Program Files (x86)\Steam\steamapps\common\Warhammer 40,000 DARKTIDE
```
**Game Pass**
```
C:\XboxGames\Warhammer 40,000- Darktide\Content
```

## Monitoring
Running Dark Rebooter will monitor for crashes and 


## Complimentary mods
This pairs very niceley with [Log Me In] by raindish. Combined, it means you'll be taken back to the hub after crashing.

### For modders
This was originally made to alleviate the inefficiencies of frequent crashes during mod development, so there's one extra skip you can do for modding. Grab [Psych Ward] by [Fractality] and modify the `mod:hook(StateMainMenu, "update", ...)` block so that the check for `if _go_to_shooting_range then` is ignored/removed.

 I personally keep a separate, stripped down copy of the mod installed so that updates don't wipe the changes.
