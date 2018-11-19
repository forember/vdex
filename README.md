# Parallel Battles in Rust - Core/Host

This is a server (with accompanying client libraries) for simulating Pokémon battles. The goal is to support all *features* of Gen IV, with the stats/moves/Pokémon/etc. of Gen V. The server can also provide basic Pokédex information (base stats/movesets/etc.), but no art assets, display names, or flavortext. These are considered outside the scope of a simulator core, and are more likely to attract the ire of Nintendo.

On the topic of the ire of Nintendo, this repository contains data from [veekun](https://github.com/veekun/pokedex). Here is their caveat regarding copyright:

> The software is licensed under the MIT license.  See the `LICENSE` file for full copyright and license text.  The short version is that you can do what you like with the code, as long as you say where you got it.
>
> This repository includes data extracted from the Pokémon series of video games.  All of it is the intellectual property of Nintendo, Creatures, inc., and GAME FREAK, inc. and is protected by various copyrights and trademarks.  The author believes that the use of this intellectual property for a fan reference is covered by fair use — the use is inherently educational, and the software would be severely impaired without the copyrighted material.
>
> That said, any use of this library and its included data is **at your own legal risk**.

Obviously a simulator goes a fair bit beyond a fan reference, but I do not believe that this project harms the market for the Pokémon games. On the contrary, at least in my personal experience, projects like this have kept engagement in the franchise, actually driving sales. But who am I to know. No lifeguard on duty, swim at your own risk.
