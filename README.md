# homemade
homemade ECS engine developed in rust

This engine follows the general design cues of a [very popular keynote](https://www.youtube.com/watch?v=P9u8x13W7UE), by using a centralized 'world' struct with an AnyMap to store components.
For (a little bit of) data efficiency, components can choose to be stored in Vectors, HashMaps, or BTreeMaps. 
Systems, then, are just plain functions and/or closures that hook into the World's update() family of functions.

Sounds easy enough?

Not so.
This engine hits roadblock after roadblock as threadsafe game programming is incredibly hard when you have all your data in the same parent struct. 
For example, read()'ing a component while inside an update() block of the same component is impossible, even though they're the same data being accessed within the same context, Rust doesn't know that.
There's no way to guarantee to the compiler the data won't be invalidated, since it's all behind the black box of the World struct.

Feebly I tried to implement a macro-based 'scripting' system via the .rsc files, but it quickly dawned on me that no, Rust is not a scripting language and it never will be (and that's a good thing!).
However, for a timely development cycle something must be done, and this design is basically impossible to expose to Lua or any other scripting language (unless you're Catherine West)
