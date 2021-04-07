# Specification

* This is an attempt at a 3D physics engine found at https://github.com/INDAPlus20/isaklar-ahanifi-maltebl-lervik-project
* Issues should be as short as possible, yet informative. 
* Branches shall be named akin to `bug/13_ground-friction` meaning: `<branch-type>/<issue#>_<shortened-issue-description>`
* Commits should be named according to the whims of the author.
* In case of fire: Use issue-tag `Catastrophic`
* Pullreq must be accepted by atleast two people.

## Project

We are going to try to make a 3D physics engine in 🦀 rust 🦀. Inspiration drawn from, among many things: https://nphysics.org/ & https://www.ncollide.org/ and it's examples. A finished product with all features is not expected but above all goals stated in `Enjoyment` are sought. 

### Outline

* Rendering (use library?)
    * Create baseline to show images
    * Render frames continously 
    * Simulate camera movement
* Vector-math using ultra-violet (https://github.com/termhn/ultraviolet)
* Define & render 3D objects
    * Cubics
    * Circles
    * Composite shapes
    * Contains_point(x,y,z)
* Basic movement
    * Coordinate position
    * Velocity
    * Acceleration
    * Friction
    * Gravity
    * Rotation
* Geometric data
    * Mass
    * Center of mass
    * Bounding volumes
    * Contact 
    * Proximity (check intersection if moving according to vector )
    * Time of impact
* Difficult movement
    * Inertia
    * Maximum velocity
    * Friction
* Advanced factors
    * Lighting and shading
    * Some sort of fluid dynamics
    * Air resistance
* If for some reason we have time
    * Create own OpenGL backend?
* More?

### Difficulties

* Coordination
* Math
* `git merge`
* Dividing up work
* Performance?

### Enjoyment
* Make lil'object go wooosh
