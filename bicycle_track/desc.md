---
title: Mono-Bicycle Track
binary: bicycle_track
summary: This animation is about a special bicycle track. This is a track where the front wheel and the rear wheel follow exactly the same track. In some sense it is a path that is not distinguishable from a monocycle.
---

# What is it ?

A bicycle track is _not_ two random tracks (for the rear & front wheels).
Because the bicycle is rigid there is always the same distance between the wheels.
The rear wheel is also always tangent to the track it is drawing (this is due to the fact that it is "pulled" in some sense).

So given a rear track it is straightforward to reconstruct the matching front track.

Now is it possible to have a bicycle track such that the rear wheel runs exactly over the front wheel's track ?

The answer is yes! This animation demonstrates some of these tracks.

## Warning

When increasing the segment count computing the track can become very expensive. In addition the curve will likely become very noisy. To compensate for the noise you should increase the smoothing factor, but this also increases the cost of a segment.

# Constructing the track

## Front wheel tracks

Before trying to construct the "mono-bicycle track" we need to show how we construct an arbitrary front wheel track from a rear wheel track.

So if we are give a rear wheel track $f(t) = (x(t), y(t))$ we can calculate the derivative $f'(t) = (x'(t), y'(t))$.

The vector $T(t) = (x'(t), y'(t))$ is tangent to the curve at the point $(x,y)$. In order to find the front wheel's position we just need to follow the vector for the length of the bike. For simplicity let's assume the bike is of length 1. Then the front wheel is at position $(x(t),y(t)) + \frac{T(t)}{\lVert T(t) \rVert}$.

## Initial rear wheel track

Because we are able to derive a front wheel track from a rear wheel track we only need to provide the rear wheel track for the interval $[0, 1]$. Indeed once we have found this track we will have caught up to the front wheel, and we can do our calculation with the next segment.

The initial track needs to satisfy a pretty strong condition: It must be a [flat function](https://en.wikipedia.org/wiki/Flat_function) at $0$ and $1$. This means that the function and all of it's derivatives must be $0$ at those points.

The family of functions that were chosen in this animation where $A \mapsto (x \mapsto A \exp(\frac{-1}{x^2})\exp(\frac{-1}{(1-x)^2}))$.

## Calculating the derivative

Calculating the derivative numerically can be done quite easily, but it introduces quite some error.

Let's say we have equidistantly spread out points $(t_i)_{i\in [0..n]}$. Then the derivative at the point $i$ can be calculated to be $f(t_{i+1}) - f(t_i)$. We can then calculate the front wheel position $n(t_i)$ using this information.

In order to smooth out the front wheel track we can replace $n(t_i)$ with the average of $n(t_{i-w}) ... n(t_{i+w})$ for some w.

## Further Reading

I saw about this in the following video: [A Curious Track, or What Bikes Are Hiding From Us](https://www.youtube.com/watch?v=l7bYY2U5ld8). I highly recommend checking it out, it is explained in a much nicer and intuitive way than my short explanation.

I used the paper [Notes on the Unicycle Puzzle](http://stanwagon.com/public/1248UnicycleSolution.pdf) to help me clear up some doubts on the subject.
