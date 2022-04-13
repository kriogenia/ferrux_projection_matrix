# FerruX Projection Matrix
Tool to ease the generation of perspective projection matrices to convert 3D vectors into their 2D projection. 

## Building the matrix
To build the matrix just create a new Builder and set the properties that will define the projection matrix:
* `near`: Position of the near clip in the z-axis. By default `0.0`.
* `far`: Position of the far clip in the z-axis. By default `1000.0`.
* `fov`: Field of view in degrees. By default `90.0`.
* `width`: Frame width. By default `1280`.
* `height`: Frame height. By default `720`.

```rust
let matrix = ferrux_projection_matrix::ProjectionMatrixBuilder::new()
               .set_width(1920)
               .set_height(1080)
               .set_fov(100.0)
               .set_far(2000.0)
               .set_near(1.0)
               .build();
println!("{matrix:?}");
```