---
title: Billard in a Ellipse
binary: ellipse_billard
summary: This animation allows to show the different behaviours of an elliptic billard.
---

# What is it ?

We start with a ray at an angle on the $y$ axis, making a customizable angle.
We then follow the reflections of this ray around the ellipse.
It is possible to observe two distinct regimes, if we start inside or outside the foci of the ellipse.

# Constructing the billard

An ellipse is defined by two constants $a$ and $b$. They represent the length in each direction.
In this animation $b$ is fixed to be $1$. We can change the eccentricity $e$, between 0 and 1.
We obtain $a$ from $e$ by the following: $a=\frac{1}{\sqrt{1-e^2}}$.

## Finding an intersection

We have a ray starting from $(x_0, y_0)$ at an angle $\theta$ from the origin.
The ray satisfies the following equation: $\cos(\theta)(y-y_0) = sin(\theta)(x-x_0)$.

The equation of an ellipse is $\frac{x^2}{a^2}+\frac{y^2}{b^2} = 1$.

Because we want a point on the line and the ellipse at the same time we are going to substitute the line equation in the ellipses'.

First isolate $y$. We get $y = \tan(\theta)(x-x_0) + y_0$. Note that this does not work for vertical lines, we will deal with them later.

In order to simply our calculations let's set $\Omega = y_0 - \tan(\theta)x_0$, we have $y = x\tan(\theta) + \Omega$.

Substituting in our ellipse equation we get the following:
$b^2x^2 + a^2(x\tan(\theta) + \Omega)^2 = a^2b^2$.
Expanding a bit leaves us with $x^2(b^2 + a^2\tan^2(\theta)) + x(2a^2\Omega\tan(\theta) + a^2(\Omega^2-b^2)$.

Let's set $\alpha = b^2 + a^2\tan^2(\theta), \beta = 2a^2\Omega\tan(\theta), \gamma = a^2(\Omega^2-b^2)$.

To solve this quadratic let's also set $\Delta = \beta^2 - 4\alpha\gamma$. Our two possible $x$ coordinates will be $\frac{-\beta \pm \sqrt(\Delta)}{2\alpha}$.

If we are not evaluating the first ray it's easy to choose the $y$ value, one of those is our current $x$, so we can pick the other one.

For the first ray we need to check with the initial angle which side of the ellipse our intersection should be in.

We can then recover the $y$ coordinate with the equation we set at the start.

Those equations are mostly symmetric in $x$ and $y$. To re-express the equations in terms of $y$ we can swap $a$ and $b$, and swap $\tan(\theta)$ with $\frac{1}{\tan(\theta)}$.

We choose the $x$ form or $y$ form in order to minimize the floating errors we could observe, meaning half the time we take one and the other half the other.

## Finding the reflected ray

Now that we have our intersection point $x_i, y_i$ we need to find the angle of the outgoing ray.

First we need the normal vector. We can easily get it through the tangent vector.

The equation of the tangent line is $\frac{x_ix}{a^2} + \frac{y_iy}{b^2} = 1$. We know $x_i,y_i$ is on the line.
Setting $x=0$ tells us that $0,\frac{b^2}{y_i}$ is on the line, giving a tangent vector $(x_i,y_i - \frac{b^2}{y_i})$.

There are two edge cases, when $y_i$ is zero the tangent vector is $(0,1)$ and when $x_i$ is zero it is $(1,0)$.

If the tangent vector is $(t_x,t_y)$ then the normal vector is $n'=(-t_y,t_x)$. Let's note $n = \frac{n'}{\|n'\|}$. We need the normal vector pointing to the center, so we can take the dot product of the vector and the vector from the origin to the intersection point to choose the correct one.

Because our calculations for the intersection were done with the angle between the origin and the ray we need to find the outgoing angle with the origin.

We can do this by finding the outgoing vector $o = i - 2 (n \cdot i) n$. We can then have the outgoing angle with $\tan^{-1}(\frac{o_y}{o_x})$, with the wrapping around for negative $o_x$. We can then plot the next ray!
