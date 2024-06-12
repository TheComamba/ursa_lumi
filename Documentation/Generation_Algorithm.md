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

Due to [branch prediction][branch-prediction], later steps will be faster on the CPU if chunks with similar properties are processed close to each other. Therefore, an algorithm that generates them sorted by their distance is used:

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

### Adding Stars

Stars in the milky way can be divided in several different populations. Each of these populations has different properties:
- The current age of stars in a population can either be a range, or a specific value that's basically the same for all. In case of white dwarfs, the age is irrelevant because of the vast timescales on which they develop.
- The local density $\rho_0$ can be consered a constant scale factor of the mass density of stars in a given population.
- The axis ratio $\epsilon$ is the ratio of _scale height_ and _scale length_: The stellar mass density for many populations in the milky way exponentially decreases on a characteristic height scale when moving above or below the galactic plane, and it decreases on a charactersitic length scale when moving outwards radially.
- The metallicity is defined as
  $$\langle[\text{Fe/H}]\rangle = \log_{10}\left(\frac{N_\text{Fe}}{N_\text{H}}\right)_\ast - \log_{10}\left(\frac{N_\text{Fe}}{N_\text{H}}\right)_\odot \,,$$
  the logarithmic ratio of the number of iron and hydrogen atoms as compared to their ratio in the sun. It is measured in dex.

  Note that in the thin disc, a radial metallcity dispersion of $-0.07$ dex/kpc is present.

[todo_metallicity]: <Are the metallcitiy values in the table refering to those at the center, at the position of the sun, or are they the mean over the whole of the milky way?>

Parameters for the different populations are given as follows (copied from [Robin2010][Robin2010], Table 1):
|Population|Age [Gyr]| $\rho_0$ [$M_\odot \text{pc}^{-3}$] | $\epsilon$ | $\langle[\text{Fe/H}]\rangle$ [dex] |
|---|---|---|---|---|
| Thin disc | $0-0.15$ | $4.0\cdot10^{-3}$ | $0.0140$ | $\phantom{-}0.01\pm0.010$ |
|| $0.15-1$ | $7.9\cdot10^{-3}$ | $0.0268$ | $\phantom{-}0.00\pm0.11$ |
|| $1-2$ | $6.2\cdot10^{-3}$ | $0.0375$ | $-0.02\pm0.12$ |
|| $2-3$ | $4.0\cdot10^{-3}$ | $0.0551$ | $-0.03\pm0.125$ |
|| $3-5$ | $4.0\cdot10^{-3}$ | $0.0696$ | $-0.05\pm0.135$ |
|| $5-7$ | $4.9\cdot10^{-3}$ | $0.0785$ | $-0.09\pm0.16$ |
|| $7-10$ | $6.6\cdot10^{-3}$ | $0.0791$ | $-0.12\pm0.18$ |
|| White dwarf | $3.96\cdot10^{-3}$ | - | - |
| Thick disc | $11$ | $1.34\cdot10^{-3}$ | - | $-0.50\pm0.30$ |
|| White dwarf | $3.04\cdot10^{-4}$ | - | - |
| Spheroid | $14$ | $9.32\cdot10^{-6}$ | $0.76$ | $-1.5\pm0.50$ |
| Bulge | $10$ | - | - | $\phantom{-}0.00\pm0.20$ |

According to Table 2 of [Robin2010][Robin2010], the mass densities for the different populations are given by the following forumlae:

#### Young Thin Disc Stars (age $\leq 0.15$ Gyr)

Given the radial distance from the galactic center $R^2 = x^2+y^2$, the density at a position is modelled as
$$\rho(R,z) = \frac{\rho_0}{d_0 k_\text{flare}}\left[\exp\left(-\frac{a^2}{h_+^2}\right) - \exp\left(-\frac{a^2}{h_-^2}\right)\right]\,,$$
where
- $d_0$ is a normalization factor to have a density of 1 at the solar position.
- $k_\text{flare} = 1 + (R-R_\text{flare})g_\text{flare}H(R-R_\text{flare})$ accounts for the increase of the thickness of the disc with galactocentric distance.
- $R_\text{flare} = 9500$ pc is the flare radius.
- $g_\text{flare} = 0.545\cdot10^{-6}\text{ pc}^{-1}$ is a flare parameter.
- $H(R-R_\text{flace})=\left\lbrace \begin{matrix} 1&, &R\geq R_\text{flare} \\ 0&, &R< R_\text{flare} \\ \end{matrix} \right.$ is the Heavyside step-function which assures that the flare factor only plays a role after a certain radius (compare [Robin2003][Robin2003], Sec. 2.1.3).
- $a^2 = R^2 + \left(z / \epsilon / k_\text{flare}\right)^2$ is a somewhat adjusted distance from the galactic center, where the scale height $\epsilon$ is scaled by the flare factor (compare [Robin2003][Robin2003], Sec. 2.1.3).
- $h_+ = 5000$ pc is a radial decay length dominating at larger distances.
- $h_- = 3000$ pc is another radial decay length dominating at smaller distances.
- $\rho_0$ and $\epsilon$ are taken from the table above. 

[todo_density]: <Determine normalization factor d_0>

#### Older Thin Disc Stars (age $> 0.15$ Gyr)

$$\rho(R,z) = \frac{\rho_0}{d_0 k_\text{flare}}\left[\exp\left(-\sqrt{0.25 + \frac{a^2}{h_+^2}}\right) - \exp\left[-\sqrt{0.25 + \frac{a^2}{h_-^2}}\right)\right]\,,$$
where
- $h_+ = 2530$ pc is a radial decay factor dominating at larger distances.
- $h_- = 1320$ pc is another radial decay factor dominating at smaller distances.
- the other parameters are the same as before.

#### Inner Thick Disc Stars ($|z| \leq x_l$)

$$\rho(R,z) = \frac{\rho_0}{d_0 k_\text{flare}} \exp \left( - \frac{R-R_\odot}{h_R} \right) \left( 1 - \frac{z^2}{h_z x_l (2 + x_l / h_z)}\right) \,,$$
where
- $x_l = 72$ pc is a height threshold.
- $R_\odot = 8.2$ kpc is the disctance of the sun from the galactic center.
- $h_R = 4000$ pc is a radial decay length.
- $h_z = k_\text{flare} \cdot 1200$ pc is a decay hight adjusted by the flare factor.
- the other parameters are the same as before.

[todo_scale_height]: <Have I correctly implemented the flare factor here?>

#### Outer Thick Disc Stars ($|z| > x_l$)

$$\rho(R,z) = \frac{\rho_0}{d_0 k_\text{flare}}\exp \left( - \frac{R-R_\odot}{h_R} - \frac{|z|}{h_z}\right) \frac{\exp \left( \frac{x_l}{h_z} \right)}{1 + \frac{x_l}{2h_z} } \,,$$
where all parameters are the same as before.

Note that [Robin2010][Robin2010] lists this formula without the factors $d_0$ and $k_\text{flare}$, but comparing to [Robin2003][Robin2003], I think they may have been forgotten here.

#### Inner Spheroid ($a \leq a_c$)

$$\rho(R,z) = \frac{\rho_0}{d_0} \left( \frac{a_c}{R_\odot}\right)^{-2.44} \,,$$
where
- $a_c = 500$ pc describes the boundary of a spheroid.
- the other parameters are the same as before.

Note that the density for the population in this regime is constant.

#### Outer Spheroid ($a > a_c$)

$$\rho(R,z) = \frac{\rho_0}{d_0} \left( \frac{a}{R_\odot}\right)^{-2.44} \,,$$
where all parameters are the same as before.

Note that [Robin2010][Robin2010] lists this formula without the normalisation factor $d_0$, but comparing to [Robin2003][Robin2003] and considering that the density should probably be continuous at $a = a_c$, it was probably forgotten.

#### Inner Bulge ($\sqrt{x_B^2+y_B^2} \leq R_c$)

$$\rho(x_B,y_B,z_B) = N \exp\left( - 0.5 r_s^2 \right) \,,$$
where
$$r_s^2 = \sqrt{\left[\left(\frac{x_B}{x_0}\right)^2 + \left(\frac{y_B}{y_0}\right)^2\right]^2 + \left(\frac{z_B}{z_0}\right)^4}$$
and (compare [Robin2003][Robin2003], Table 5)
- $x_B$, $y_B$ and $z_B$ are the cartesian coordinates in the bulge's frame of reference.
- $\alpha = 78.9^\circ$ is the angle between the bulge major axis and the line perpendicular to the sun - galactic center line.
- $\beta = 3.5^\circ$ is the tilt angle between the bulge plane and the galactic plane.
- $\gamma = 91.3^\circ$ is the roll angle around the bulge major axis.
- $x_0 = 1.59$ kpc is the scale length along the major axis.
- $y_0 = 0.424$ kpc is the scale length along one of the minor axes.
- $z_0 = 0.424$ kpc is the scale length along the other minor axis.
- $R_c = 2.54$ kpc is the cutoff distance
- $N = 13.70 \text{ stars pc}^{-3}$ is the star density at the center of the bulge.

#### Outer Bulge ($\sqrt{x_B^2+y_B^2} > R_c$)

$$\rho(x_B,y_B,z_B) = N \exp\left[ - 0.5 r_s^2 -0.2 \left( \sqrt{x_B^2 + y_B^2} - R_c\right)^2\right] \,,$$
where all parameters are the same as before.

Note that the formula in [Robin2010][Robin2010] contains a $e^{-5}$ inside the exponent. I am confident that this is a typo.


[Robin2003]: https://github.com/TheComamba/UrsaLumi/blob/dev/documenting-physics/Documentation/Literature/Robin2003.pdf
[Robin2010]: https://github.com/TheComamba/UrsaLumi/blob/dev/documenting-physics/Documentation/Literature/Robin2010.pdf

[branch-prediction]: https://www.educative.io/answers/what-is-branch-prediction
