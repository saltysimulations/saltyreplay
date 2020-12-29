# <img src="https://cdn.discordapp.com/attachments/787690098497945600/787693736452423690/salty_big_nobg.png" placeholder="SaltyReplay" width="200"/>

[![Discord](https://img.shields.io/discord/698720578055700650?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2&style=flat-square)](https://discord.gg/S4PJDwk)

## About
SaltyReplay is a free and open source replay system for Microsoft Flight Simulator. Because replays are one of the most requested features for MSFS, we are developing a replay system that will be beneficial to the community at no cost.

## Disclaimer
SaltyReplay is a **work in progress** and should not be used outside of testing and development at the moment. This means that not all features are implemented and that there is still a lot of work to be done. Everything you see in this repository as well as in previews is not final, and is subject to change. We do not support use of SaltyReplay yet, and we would not recommend it if you are not a developer.

## Major Issues
### Replays
* The speed of replays is not correct. This is due to collecting data every frame instead of a set amount of time.
* Active pause needs to be activated for the replay to be smooth
* The only data that gets recorded for now is the position, bank angle, pitch, altitude and heading. Other data like gear, flaps, and other external animations will have to be added later. Although switches in the interior are planned in the future, this is one of the least prioritized things right now.
### UI
* The replay logic is not currently tied to the UI. 
* The CSS is very badly made with too much absolute and relative positioning used, which makes for a non-resizable window for now. Ideally, the CSS should be reworked to allow resizing.
* A way to properly show replays in the replays page has to be implemented. A placeholder JSON is currently used.
* Routing to different pages is sometimes broken.
* The content served by actix-web sometimes displays incorrectly.
### Other
* Some code cleanup is needed to improve readability.

## Setup
### Dependencies
* Node.js
* Rust stable
### Building
* Navigate to the web folder -  `cd src/web`
* Install Node dependencies and build React project - `npm install && npm run build`
* Navigate to the root folder - `cd..` `cd..`
* Build Rust project - `cargo build`
* Execute this command in an **administrative** command prompt - `checknetisolation LoopbackExempt -a -n=Microsoft.Win32WebViewHost_cw5n1h2txyewy` (This is required because Edge does not allow loopback by default, which is required by the local server. This is currently a workaround until a better solution can be found later.)

## FAQ
**Q: When will it release?**

A: There is no set date, and we will not set one anytime soon.

**Q: Can I use SaltyReplay at this time?**

A: Short answer is that the average simmer shouldn't. SaltyReplay is not in a well-working state right now. You are free to test it out, however we do not provide any support for use at this moment. 

**Q: I am interested in contributing. How do I get started?**

A: Firstly, join our Discord at the top of the readme and do the setup process mentioned above. The list of major issues is located at the top of this readme, and all issues, bugs and requests are handled through GitHub issues. 

## License
All contents of this repository are licensed under the GNU General Public License version 3. See [LICENSE](https://github.com/saltysimulations/saltyreplay/blob/master/LICENSE). For the contents of the "web-view" directory, [this copyright notice](https://github.com/saltysimulations/saltyreplay/blob/master/web-view/NOTICE) applies (Originally distributed under MIT). The use of the web-view library locally is temporary, as a slightly older version has to be used in order to avoid a bug.
