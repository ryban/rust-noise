# rust-noise


## Building


```sh
cargo build
```

## Examples


```cargo test``` will build the examples, then you can run them individually

```
cargo test
./target/simplex
./target/fbm
./target/ridged
./target/billow
./target/voronoi
```

## Issues


3D simplex noise does not currently work. Since Simplex is the noise source for every other generator, none of them generate 3D noise.
