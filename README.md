# <img src="https://user-images.githubusercontent.com/26278705/120645097-55d3de00-c478-11eb-8b9b-6ec866f6c882.png" width="300" />

[![Discord](https://img.shields.io/discord/698720578055700650?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2&style=flat-square)](https://discord.gg/S4PJDwk)

## About
SaltyReplay is a free and open source replay system for Microsoft Flight Simulator. Because replays are one of the most requested features for MSFS, we are developing a replay system that will be beneficial to the community at no cost.

## Disclaimer
SaltyReplay is a **work in progress** and should not be used outside of testing and development at the moment. This means that not all features are implemented and that there is still a lot of work to be done. Everything you see in this repository as well as in previews is not final, and is subject to change. We do not support use of SaltyReplay yet, and we would not recommend it if you are not a developer.

## Major Issues
### Replays
* Active pause needs to be activated for the replay to be smooth
* The only data that gets recorded for now is the position, bank angle, pitch, altitude and heading. Other data like gear, flaps, and other external animations will have to be added later. Although switches in the interior are planned in the future, this is one of the least prioritized things right now.
### UI
* The replay logic is not currently tied to the UI. 
* The CSS is very badly made with too much absolute and relative positioning used, which makes for a non-resizable window for now. Ideally, the CSS should be reworked to allow resizing.
* Routing to different pages is sometimes broken.

## Setup for development
### Dependencies
* Node.js
* Rust stable
### Building
* Execute this command in an **administrative** command prompt - `checknetisolation LoopbackExempt -a -n=Microsoft.Win32WebViewHost_cw5n1h2txyewy` (This is required because Edge does not allow loopback by default, which is required by the local development server.)
* Install Node dependencies and start React dev server - `npm install` `npm start`
* Start Tauri application  - `npm run tauri dev`

## FAQ
**Q: When will it release?**

A: There is no set date, and we will not set one anytime soon.

**Q: Can I use SaltyReplay at this time?**

A: Short answer is that the average simmer shouldn't. SaltyReplay is not in a well-working state right now. You are free to test it out, however we do not provide any support for use at this moment. 

**Q: I am interested in contributing. How do I get started?**

A: Firstly, join our Discord at the top of the readme and do the setup process mentioned above. The list of major issues is located at the top of this readme, and all issues, bugs and requests are handled through GitHub issues. 

## License
All contents of this repository are licensed under the GNU General Public License version 3. See [LICENSE](https://github.com/saltysimulations/saltyreplay/blob/master/LICENSE).
