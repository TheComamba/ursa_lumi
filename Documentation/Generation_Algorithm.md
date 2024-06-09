# Generation Algorithm

Ursa Lumi is heavily based on [Robin2010][Robin2010].

At the moment, it only supports generating galactic objects.

## Generation Parameters

The following parameters need to be specified when starting a simulation:
- `observer_position`: This describes the `x`, `y` and `z` coordinate of an observer relative to the galactic center.

    This value cannot be changed in follow-up generations.
- `apparent_magnitude_threshold`: Together with the `observer_position` this restricts the minimal brightness a star must have to be included in the output.

    Currently this value can also not be changed in follow-up generations, so choose it wisely.
- `max_distance`: The maximal distance up to which new stars are generated in this generation run.

The generation parameters are stored together with the output.

## Initial Generation

### Chunking the Galaxy

For the initial generation of stars the local environment of the observer is separated in chunks. Each chunk is a cube with edges that span $S_C = 15 \text{ pc}$. $S_C$ is called the "chunksize". A chunk is therefore characerised by the three ordinates of one of its corners. The first chunk contains the observer at its center. Its corner is therefore at the position
$$C_0 = P_{\text{observer}} - \frac{1}{2}(S_C,S_C,S_C) \,.$$

> The sun is at a distance of `8.2 kpc` from the center of the milky way. Generating stars up to that distance results in
> $$\frac{4}{3} \pi \frac{{8200}^3}{{15}^3} \approx 7\cdot10^8$$
> chunks.

Due to [branch prediction][branch-prediction], later steps will be faster on the CPU if chunks with similar properties are evaluated close to each other. Therefore, an algorithm that generates them sorted by their distance is used:

- Calculate `N = floor(max_distance / CHUNKSIZE)`, the number of chunks between the origin and the maximum generation distance along an axis.
    > To speed up the calculation, allocate the memory for a vector with $\frac{1}{8}\frac{4}{3} \pi N^3$ three-tuples of integers once `N` is known.
- Fill the first quadrant: For $x\geq0$, $y\geq0$ and $z\geq0$ where
    $$x\leq N,$$
    $$y^2 \leq N^2 - x^2,$$
    $$z^2 \leq N^2 - x^2 - y^2 $$
    emplace $(x,y,z)$ in the vector.
- Sort the vector by distance.
- Create a new vector for all coordinates.
    > Once again, its size is known beforehand and it can be allocated.
- Add the zero-chunk containing the `observer_position` at its center.
- Loop through all entries in the quadrant and all sign combinations $\pm_{x,y,z}$ (in that order, working our way outwards).
  - We need to avoid duplicates coming from the fact that `+0 == -0`. Therefore,
    if `x == -0`, `y == -0` or `z == -0`, continue.`
  - Add a chunk at $C_0 + (\pm_x x,\pm_y y,\pm_z z) S_C$ to the vector, where $C_0$ is the position of the observer chunk.

[Robin2010]: https://github.com/TheComamba/UrsaLumi/blob/dev/documenting-physics/Documentation/Literature/2010-Robin.pdf

[branch-prediction]: https://www.educative.io/answers/what-is-branch-prediction