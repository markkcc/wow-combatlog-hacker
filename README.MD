# WoW Combat Log Hacker 

Tested with COMBAT_LOG_VERSION: 9, BUILD_VERSION: 1.15.2 (WoW Classic: Season of Discovery)
This repo hosts Rust code that will look for heal events and increase healing by a multiplier.

[Warcraft Logs](https://sod.warcraftlogs.com/) is an unofficial (non-Blizzard affiliated) third party website that collects Advanced Combat Logs from World of Warcraft to display statistics and rankings of players according to their in-game performance. 

It works by importing the game's combat log and processing it to extract events and display statistics. 

Once Advanced Combat Logging is enabled from the game's menu (System > Network), typing `/combatlog` within the game client starts savings logs to `Program Files (x86)\World of Warcraft\_classic_era\Logs\WoWCombatLog-date-timestamp.txt`.

## Log Format

Here's a sample line from the logs for a healing event:

```
4/10 19:19:02.680  SPELL_HEAL,Player-1234-8FFFFFFF,"Charactername-ServerName",0x511,0x0,Player-1234-8FFFFFFF,"Charactername-ServerName",0x511,0x0,19278,"Devouring Plague",0x20,Player-1234-8FFFFFFF,0000000000000000,98,100,0,0,0,-1,0,0,0,-7665.44,-4634.82,1446,3.8602,50,70,70,0,0,nil
```
Description of each field:

| Field Description | Sample Value 			|
| ----------------- | --------------------------------- |
| Month / Day       | 4/10         			|
| Timestamp (ms)    | 19:19:02.680 			|
| Event name        | SPELL_HEAL or SPELL_PERIODIC_HEAL |
| Player ID         | Player-1234-8FFFFFFF              |
| Character name    | "Leeroyjenkins-LivingFlame"       |
| Hex ID            | 0x511 or 0x10511 etc              |
| Hex Zero?         | 0x0      				|
| Target player ID  | Player-1234-8FFFFFFF              |
| Target player name| "Leeroyjenkins-LivingFlame"       |
| Target Hex ID     | 0x511 or 0x10511                  |
| Hex Zero?         | 0x0 				|
| Spell ID          | 425275				|
| Spell name	    | "Renew"				|
| Spell type	    | 0x2 or 0x20 etc    		|
| Owner ID	    | Player-1234-8FFFFFFF		|
| Zeroes?	    | 0000000000000000			|
| Current HP	    | 98 (between 0 and 100)            |
| Max HP	    | 100				|
| unknown1	    | 0					|
| unknown2          | 0                                 |
| unknown3          | 0                                 |
| unknown4          | -1                                |
| unknown5          | 0                                 |
| unknown6          | 0                                 |
| unknown7          | 0                                 |
| Location: X coord | -7665.44				|
| Location: Y coord | -4634.82				|
| ZoneID	    | 1446				|
| Facing rotation   | 5.3675				|
| item level	    | 51				|
| Healing amount    | 250				|
| Raw healing amt   | 250				|
| Overhealing       | 72				|
| Resist/Absorb?    | 0					|
| Critical	    | nil or 1				|


