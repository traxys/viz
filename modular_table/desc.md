---
title: Modular Tables
binary: modular_table
summary: This animation is about modular tables, and the nice pattern you can get.
---

# What is it ?

Pick two integers $n$ and $i$. You can calculate the times table for $i$: $1 \times i$, $2 \times i$, ... $k \times i$.

Then for each $k$ draw an edge from $k$ to $k \mod n$. This generates the "modular" times table of $i$ modulo $n$.

If $n$ is prime then all the points are reached & form a single path. If $n$ is not prime then there can be multiple components, you can color them using the checkbox.

# Interesting values

At large modulus (like 300) you start seeing interesting patterns for different modifiers.

You should try 2 to around 15, they show the same number of lobes as the multiplier. Then there are nice geometric patterns like 21, 44, 59 or 61. 

# Drawing the diagram

Drawing the diagram is quite simple. You just need to iterate of the $n$ moduli points, create a graph that has an edge from $k$ to $k \mod n$, then find draw the graph. If choosing to color the connected components then we just need to iterate over the unique pairs $a,b$, and check if $a$ connects to $b$ and choose a sufficiently different color for the components.

This can be done by searching for spaced out colors in a space like YCbCr or HSV, then converting to RGB.

# More information

If you want to see more resources on modular tables I recommend the following Mathologer youtube videos:

 - [Tesla’s 3-6-9 and Vortex Math: Is this really the key to the universe?](https://www.youtube.com/watch?v=6ZrO90AI0c8)
 - [Times Tables, Mandelbrot and the Heart of Mathematics](https://www.youtube.com/watch?v=qhbuKbxJsk8)
