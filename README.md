# DAGNALOK - Multiplayer MMO with focus on PVP

~~**1. Setup Core - i.e., camera, lights, asset loading, terrain, character**~~

~~**2. Setup animations - Character and Animations will be set up closely to one another to allow for better modularity and providing different character meshes with correct animations**~~

**2.5. Set up Player - Setup the player so that we have a controller for our character.  We must also have a player in order for a UI to exist.**

**3. Movement system - Once terrain, characters, and animations are setup, create a movement system for the player to control the character, incporporating animations for fluidity**

**4. Basic UI - Work to create a basic UI that will keep track of player's name, stats, etc**


**5. SpacetimeDB Integration - Setup server and connect client to server.**

**6. STDB player/character integration - Setup for server to keep track of player location, direction, speed, health, target, combat state.**

**7. Basic combat - create system that allow connected players attack other players based on combat state.**

**8. Expand UI Functionality - Create UI for player to select play/connect. If play, take player to character screen to create new or use character to connect to world**

**Features to be added: Character inventory, stats, class, basic weapons, basic armor, ranged and magic combat, better terrain, account creation/0auth2 login, characater subclasses**


More features will be added as the game continues to take shape and/or as ideas come to fruition.


This is a step by step process of what I believe the game should do upon startup.  This is a working and living document to help keep track of everything going on.

1. The player double clicks the game icon and is presented with a login screen.
2. Player provides account information then clicks "log in"
3. Server verifies the the account information and connects.  If the account does not exist, player will be required to create one.  If PW is wrong, player will be required to re-enter the password up to 4 times before being timed out.  There will also ben an option for the player to change their password or connect through other means such as O2Auth.

Once the player is logged in, the launcher will request certain informatino from the server regarding the player's account. such as the player's characters or game settings.
The player is then presnted with the option to launch the game.

Once clicked, the client will begin loading in assets.  Such as the menu, background, sounds, as well as the assets required for the player's character(s).  The player will be presented with a menu: (Play, Settings, or Quit).  If the player clicks play, they will be taken to the character selection page, where they will have the options to either select a character to continue playing with, or to create a new character.

When a player makes their selection and clicks (Connect) the client will request the character's (name, health, position, class, subclass, attributes, inventory, etc), as well as other player's around them.  The player's client will load in all of the other player's characters around them, and any interactions taking place.  Movement, character position, if combat is going on, the terrain, etc.

From this point on, it'll all be gameplay
The player will have the option to do many things throughout the world.  Explore the world, combat with NPCs, or other players.  They will also have the option to gather resources, fish, or craft.

Players will also be able to build bases, cities, or towns in a very similar fashion to Shadowbane.

Characters will have a name, race, class, health, crafting professions, subclass, attributes, position, direction, speed, in_combat, targetable, is_targetted, has_target, is_moving, is_attacking, is_attacked,
