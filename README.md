# Wanderlust to kill a Box

This project was generated using the [Bevy New 2D](https://github.com/TheBevyFlock/bevy_new_2d) template.

## Overview

Demo video, very rough game for now : D please dont be too disappointed, it is a work in progress. : D

[Demo video](https://github.com/DanBellman/Wandern-to-kill-a-Box/issues/1#issue-3316101367)

A directly playable version link here to my website using leptos incoming here. [LINK HERE]

## Controls:
- Right click to shoot
- Shoot the yellow box in the middle to spawn coins. Collect them. 
- Go behind the shops and buy weapons and upgrades.
- Use Q to toogle between bought weapons.


## Inspiration
This game is inspired by the rather old flash game called 'Coinbox Hero'. It was a really fun game that I played as a kid oftentimes. Check it out wherever it is available: https://armorgames.com/play/12247/coinbox-hero/ . The creator of that game is: John, https://armorgames.com/user/John , leave him a nice commecnt if you also played and enjoyed the game as a kid. While he doesnt seem to be active anymore, or have any other socials, maybe one day he will read your and my thanks. So: Thanks John for making a part of so many childhoods. 

Unfortunately, the performance of the game suffers the more you play - as more coins get spawned, and it just becomes unplayable. Thats why I wanted to create a similiar game so I can play "this game" without fps issues. And make my game open source and free. 

## Plan

I will work on this game as a hobby project, work on it over time, and update it regularly. When the first level is done, and the ground work for further logic is placed, art is finalized and polished, Version 1.0 will be released. 

#### My vision is a game like the above, 
- plus here you go on a very long adventure, 
- going through many different levels, 
- in different lands, 
- in different times, i
- t will be a very long adventure. 
- You could say that the main character is driven by a sort of Wanderlust, never wanting to stop exploring. 
- I dont want the game to ever end. That would mean that the character's Wanderlust would die, that is not my vision. 
- The main character will go through a very light story as he ages and roams the world - shown as environmental storytelling.


#### Technical vision: 
- While i dont have the appropriate hardware to test out the highest textures and quality yet. I still try to implement them and simply guess that it is working.
  - a minimum of 8k HDR assets. (And fallbacks to hardware that doesnt support it). The reason for that is that I would like to play this game later on a 8k HDR TV or monitors. So I will just implement it from the start.
- Maintain extremely high frame rates. (This is especially an important point as the inspiration game suffers from performance issues very heavily.) 

#### Name
Wanderlust because the player will travel a lot.
Lust means in german "wanting to". 
So double meaning: Having wanderlust just to kill a box, or just primarily wanting to kill a box. But it is also to discover many new levels with nice looking backgrounds and art. Thats the vision I have for this game.

## License 

This work is released under CC0 1.0 Universal (Public Domain). See more [[LICENSE]](

To the extent possible under law, Daniel Bellmann has waived all copyright and related or neighboring rights to Wandern. This work is published from: Germany.

You can copy, modify, distribute and perform the work, even for commercial purposes, all without asking permission.

[![CC0](https://licensebuttons.net/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)

## Contributions
I welcome any contributions : )  . But you dont have to contribute back at all.
Your contribution will fall under the same license CC-0 without any additional constraints or conditions. 

You can even just contribute by placing an idea and you will be credited among the implemters of that feature in the credits as the/one of the idea givers. If you are the first one and it is distinct enough for me.


   
# Working Features
- two shops
- One player
- Weapons
- Box
- You can shoot the box, hits will results in coins being spawned. 
- You can collect coins, while walking over them. 
- Coins get counted as money. 
- The buffer sets the amoung of coins you can collect in a certain time frame.

## Weapon shop
- You can buy weapons in the weapon shop with the money accumulated.

## Perks shop
- You can buy perks for yourself or other cool stuff. 

