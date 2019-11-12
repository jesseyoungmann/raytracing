# Raytracing in Rust

This is the majority of the excellent [Raytracing in One Weekend, The Next Week, and The Rest of Your Life](https://raytracing.github.io/) books, written in Rust instead of C++.

It was pretty easy to add multi-threading. Felt like Rust's `Result`s really cleaned up a lot of the functions that take an uninitialized result object and return a boolean indicating if it was successfully created. I'd like to add in a denoiser at some point, as well as display live results using `piston` or similar just for fun.

# Example results
![spheres](/examples/spheres.png)

![Cornell Box](/examples/cornell_box.png)

# TODO, because they seemed less interesting
- Motion blur
- Image textures
