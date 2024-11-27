# Physics

## Momentum

Momentum is a way to numerically quantify how difficult something that moves is
to stop. It is the product of mass and velocity. More mass, means a moving
object is harder to stop. More velocity, also means a moving object is harder
to stop.

```
momentum = mass * velocity
M = m * V
```

## Conservation of Momentum

Imagine two metal balls shooting toward each other. Let’s simplify the physics
by making the following assumptions:

1. The masses of the balls stay constant.
2. We neglect external forces, like friction or gravity, meaning the system is
closed and no energy is added or removed.

Assumption 2 means that stopping both balls before the collision will be just
as difficult as stopping them after the collision. If this were not the case,
an external force would have changed the total “stopping difficulty,” meaning
energy would have been added or removed from the system. In other words, the
total momentum of the system before the collision is equal to the total
momentum after the collision.

This can be expressed mathematically:

```
// Expressed in terms of momentum.
M1 + M2 = M1' + M2'

// Expressed in terms of velocities and masses. 
m1 * V1 + m2 * V2 = m1 * V1' + m2 * V2'
```

## Levarage

Imagine a long heavy pole, say, 100cm long hinged to a wall. If you grab the
pole at the free end and move it say 1cm away from where you grabbed it, it is
relatively easy. However, if you now grab the pole 10cm from the hinge (90cm
from the free end) and move it 1cm away again from where you're grabbing
it becomes harder to move.

Why was it harder?

Because the second time, the free end actually moved 10cm and not 1cm, so you
did 10x the work than the first time!

## Work

Work is the product of force and distance, meaning it's the energy required
to move an object with a given force over a given distance. Work is energy
transferred to an object.

```
work = force * distance
W = F * d
2W = 2F * d
```

Notice that 0 work, effectively means that no energy is required because the
force is either 0 or the distance is 0.

## Torque

```
torque = force * radius
t = F * r // (N/m)
```

## Arc Length, Angular and Tangential Velocity

A unit circle has a radius of 1, giving it a circumference of  2*pi . When we
measure angles in radians—fractions of the full circle, ranging from 0 to 2*pi,
something elegant happens: the arc length on the unit circle is always equal to
the angle in radians between two points. This works because the unit circle has
a radius of 1. Now, if we scale this circle up or down by changing the radius,
the same idea applies, leading us to a simple and universal formula for the arc
length of any circle:

```
arc length = radius * angle
s = r * theta
```

Angular velocity is the rate of change of angle over time while linear or
tangential velocity is the rate of change of distance over time. We can use what
we know about arc length, to describe the relationship between angular velocity
and tangential velocity:

```
angular velocity = angle / time
arc length = radius * angle
tangential velocity = arc length / time
angular velocity = tangential velocity / radius
angular acceleration = tangential acceleration / radius
A = a/r
```

Angular acceleration = tangential acceleration / radius

F(angular) = mass * angular acceleration


## Linear and Angular Symmetry and Moment of Intertia

1. Linear (all-lowercase). 
`f = m*a`

2. Angular (all-uppercase).
`F = M*A`

3. Direct relationship between angular acceleration and tangential
acceleration.
`A = a/r`

4. Put 3 into 2. 
`F = M*(a/r)`

5. Direct: relationship between angular force and tangential force.
F = f*r

6. Put 5 into 4.
`f*r = M*(a/r)`

7. Solve for M.
`f*r*r = M*a`
`f/a*r*r = M`
`m*r*r = M`

Here we can see that M represents the angular mass and m the linear mass. M
can be expressed in terms of the linear mass. M is called the moment of
intertia, and is expressed:

`I = m*r*r`

Angular momentum is thus a product of the angular velocity and moment of
inertia, and the conservstion of momentum applies to rotations as well.

```
// Expressed in terms of angular momentum.
A1 + A2 = A1' + A2'

// Expressed in terms of velocities and masses. 
i1 * A1 + i2 * A2 = i1 * A1' + i2 * A2'
```

## Calculus

## Kinetic Energy

Need to understand calculus to derive this.

Energy is an intrinsic property of matter that we've observed in nature and
that we've abstacted with mathematics in order to quantify it.

Kinetic energy is energy an object has due to its motion.



If we apply a constant force to a moving or stationary object over a given
distance, this applies energy. If the object was statiionary, it had no energy
but if it was moving already, it had energy. Thus, the work here is the
differnce in kinetic energy before this constant force was applied and after
the given distance.

W = K1-K2.
F*d = K1-K2
m*a*d = K1-K2
m*(v/t)*d = K1-K2
m*(v/t)*(v*t) = K1-K2
m*v*v = K1-K2
W = m*v*v




a=v/t

v=d/t
d=v*t


Get the formula for work in terms of mass and velocity (this work is called kinetic energy).


W = F * d // work is the force applied over the distance

W = (m * a) * d; // work is the mass of the object, accelerated by `a` for some distance

W = (m * (v/t)) * d;

v*t = d //  velocity is always distance over time

0.5W = 0.5(m * v/t * v*t)
0.5W = 0.5(m * v*v) // half the work?

Why is half the work called kinetic energy?

F = m * a

W = F * d

## Dot Product

The dot product of two vectors is the sum of the products of their components. 

```
A dot B = B dot A = a1*b1 + a2*b2 + a3+b3, ...
```

1. Imagine we have two vector functions, A(x) and B(x), that each return a
random vector with fixed radii R_a and R_b, respectively. Note from the
definition that if the dot product is positive and at its maximum, the vectors
must be pointing in exactly the same direction. Conversely, if the dot product
is negative and at its minimum, the vectors must be pointing in exactly
opposite directions. The dot product gives us an indication of how aligned the
vectors are.

2. The absolute values of the maximum and minimum dot products for A(x) and
B(x) must be equal. The value exactly halfway between the maximum and minimum
is zero. Therefore, if the dot product between two vectors is zero, it means
the vectors are perpendicular to each other. This is because all possible
perpendicular vectors lie between the case where the vectors point in the same
direction (maximum dot product) and where they point in opposite directions
(minimum dot product).

3. A positive dot product must then mean that the angle between the vectors
is less than 90 degrees. A negative dot product must then mean that the angle
is greater than 90 degrees.

## Cross Product

```
C = A cross B = (Ay*Bz-Az*By), (Az*Bx-Ax*Bz), (Ax*By-Ay*Bx)
```

- Cx is the difference between the products of the y and z components of vectors
A and B.
- Cy is the difference between the products of the z and x components of vectors
A and B.
- Cz is the difference between the products of the x and y components of vectors
A and B.

Notice how Cx excludes the x component of A and B, Cy excludes the y component
of A and B, and Cz excludes the z component of A and B.

## Matrices

