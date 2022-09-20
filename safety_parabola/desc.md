---
title: Safety Parabola
binary: safety_parabola
summary: This is an animation about ballistic trajectories & how not to get hit.
---

# What is it ?

Suppose we have a cannon at the origin. This cannon has a defined power, that translates to a defined speed of the projectile $v_0$.

The envelope of all trajectories form a parabola that is called the "safety parabola" because it delimits the space in two regions, safe and not safe.

The equation of this parabola is $s(x) = \frac{v_0^2}{2g} - \frac{g}{2v_0^2}x^2$.

# Derivation

In order to derive the parabola we are only going to need some basic physics & algebra.

Let's have the initial speed a vector $v_0$ with an angle $\theta$ from the $x$ axis.

## Deriving the trajectory

We then have the following projections of $v_0$:

- $v_{0,x} = v_0 \cos(\theta)$
- $v_{0,y} = v_0 \sin(\theta)$

From the second law of Newton we have:

- $a_x = 0$, so $v_x = v_{0,x}$ and finally $x(t) = v_{0,x}t$.
- $a_y = -g$, so $v_y = -gt + v_{0,y}$ and finally $y(t) = -\frac{1}{2}gt^2 + v_{0,y}t$

Using the projections of $v_0$ we arrive at:

- $x(t) = v_0\cos(\theta)t$
- $y(t) = -\frac{1}{2}gt+v_0\sin(\theta)t$

## Calculating the height of the trajectory

Now we are going to need to find $y_{max}$ the highest point reached. We can calculate $v_y(t) = 0$ to find this.

$v_y(t) = 0 \Leftrightarrow -gt + v_0\sin(\theta) = 0 \Leftrightarrow t_{max} = \frac{v_0\sin(\theta)}{g}$

We can then subsitute in $y(t)$, this gives $y_{max}(\theta) = -\frac{1}{2}g\left(\frac{v_0\sin(\theta)}{g}\right)^2 + \frac{v_0^2\sin^2(\theta)}{g}$.

After some algebra we derive $y_{max}(\theta)=\frac{v_0^2\sin^2(\theta)}{2g}$.

## Calculating the range of the trajectory

In order to find the range of the trajectory we can calculate $t_r$ such that $y(t_r) = 0$.

Because we are not interested in $t = 0$ we can divide the equation by $t$ to obtain: $-\frac{1}{2}gt_r + v_0\sin(\theta)=0$.

By isolating $t_r$ we get $t_r=\frac{2v_0\sin(\theta)}{g}$. In order to find the range we need to check $x(t_r) = \frac{2v_0^2\cos(\theta)\sin(\theta)}{g}$.

We can simplify a bit by noting that $2\cos(\theta)\sin(\theta) = \sin(2\theta)$. We obtain: $x_{max}(\theta) = \frac{v_0^2}{g}\sin(2\theta)$.

## Calculating the ballistic trajectory

We have $y(t)$ and $x(t)$, but to find our trajectory we are going to need to find $y(x)$.

This is easily done by setting $t=\frac{x}{v_0\cos(\theta)}$, and substituting in $y(t)$.
We obtain $y(x) = \frac{-g}{2v_0^2\cos^2(\theta)}x^2 + \tan(\theta)x$

Using this formula with can plot a number of trajectories for different values of $\theta$.

### Equally spread out parabolas

There are two main approaches to creating "spread out" parabolas:

- Equally spaced out $\theta$ between 0 and $\pi$
- Equal intersections on the $x$ axis. This is a bit more involved, because we need to find the angle that gives an $x_{max}$.

We have $x_{max}(\theta) = \frac{v_0^2}{g}\sin(2\theta)$, so $\sin(2\theta)=\frac{gx_{max}}{v_0^2}$.

This leads to $\theta = \frac{\arcsin\left(\frac{gx_{max}}{v_0^2}\right)}{2}$

So if we take equally spaced out $x$ intersections we get the corresponding $\theta$ for our ballistic trajectory.

## Maximum range of any trajectory

We know that the safety parabola is going to intersect the $x$ axis at furthest point reachable by the cannon. We can find that point using $x_{max}$.

Indeed we have $\frac{\mathrm{d}{x_{max}}}{\mathrm{d}\theta} = \frac{2v_0^2}{g}\cos(2\theta)$. This derivative is zero when $cos(2\theta) = 0$.

This means that $2\theta = \frac{\pi}{2}$ or that $\theta=\frac{\pi}{4}$.

By substituting in the formula for $x_{max}$ we get $x_{range} = \frac{2v_0^2}{g} \times \frac{\sqrt{2}}{2} \times \frac{\sqrt{2}}{2} = \frac{v_0^2}{g}$

## Maximum height of any trajectory

We could derive $y(\theta)$ to find the angle giving the best hight, but we can see quite intuitively that we need all the energy to go in the $y$ axis, giving $\theta=\frac{\pi}{2}$ as the best angle for height.

By looking at our formula for the height of a parabola we can get the maximum height: $y_{summit} = \frac{v_0^2\sin^2(\theta)}{2g} = \frac{v_0^2}{2g}$ because for our angle $sin(\theta)=1$.

## Deriving the safety parabola

We know that our safety parabola is going to intersect the $x$ axis at $x_{range}$ and $-x_{range}$. As such the safety parabola $s(x)$ is of the form: $a\left(x+\frac{v_0^2}{g}\right)\left(x-\frac{v_0^2}{g}\right)$.
We can distribute a bit to have $s(x) = a\left(x^2 - \frac{v_0^4}{g^2}\right)$.

We can find $a$ using the fact that $s(0) = y_{summit}$. This means $-a\frac{v_0^4}{g^2}=\frac{v_0^2}{2g}$, solving for $a$ we get $a=\frac{-g}{2v_0^2}$. We can distribute in $s(x)$ to find:

$s(x) = \frac{v_0^2}{2g} - \frac{g}{2v_0^2}x^2$
