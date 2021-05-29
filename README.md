# Raymarcher-rs

A Rust implementation of the raymarching algorithm.

Comes with a few primitives and implementors.

# Features
1. Basic primitives (cube, sphere, half plane)
1. Union, intersect, difference
1. Translation, uniform scaling
1. Phong shading
1. Reflections
1. Point lights
1. Rotations

# TODO
1. Materials with the current point as input
1. Refraction
1. Figure out where the color banding/reflection waves come from
1. More light types

# Sources/inspiration
* https://raytracing.github.io/books/RayTracingInOneWeekend.html (correct camera code)
* http://jamie-wong.com/2016/07/15/ray-marching-signed-distance-functions/ (basic principles)
* https://iquilezles.org/ (primitives, combinators, soft shadows)
* https://en.wikipedia.org/ (Phong shading, vector laws)
* Random forum/Reddit posts I forgot about.
