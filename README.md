# rust-raytracer
![2 metallic and 1 matte sphere rendered by ray tracing](rimage.png?raw=true "Rendering of metallic and matte surfaces")
This ray tracer was built using the following C++ guide as a reference: https://raytracing.github.io/books/RayTracingInOneWeekend.html. It supports spheres, metallic (reflective) and matte surface types, shadows, and a positionable camera with adjustable viewport angle.

# `camera.rs`
This contains the camera struct and function to create a new one for rendering the scene

# `ray.rs`
This defines light rays (as vectors) and contains several functions that operate on them. The most important are the linear operations and the RGB vector calculator (which recursively determines the color of a ray as it repeatedly bounces off objects)

# `world.rs`
Defines the objects (spheres) the world can support, as well as surface types and light ray interactions with surfaces. Any object that implements the hittable trait defined here can be rendered by the ray tracing program.

# `main.rs`
The entry point for the program. Currently supports CLI (use the command `cargo run` to render the world of objects from the POV of the camera created here). The program uses anti-aliasing by averaging the color of 100 light rays passed through each pixel in the screen.


