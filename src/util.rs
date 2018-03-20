use pwhash::scrypt::ScryptParams;

pub fn get_scrypt_params() -> ScryptParams {
    /* See: https://blog.filippo.io/the-scrypt-parameters/ for the choice of parameters
     *
     * Results on my machine:
     * n=12 bench:  12,347,749 ns/iter (+/- 434,208)
     * n=13 bench:  25,198,094 ns/iter (+/- 487,870)
     * n=14 bench:  51,083,006 ns/iter (+/- 1,295,275)
     * n=15 bench: 102,719,961 ns/iter (+/- 1,512,884) <--
     * n=16 bench: 209,729,930 ns/iter (+/- 75,439,669)
     * n=17 bench: 425,023,594 ns/iter (+/- 143,358,926)
     * n=18 bench: 847,250,736 ns/iter (+/- 230,782,327)
     * n=19 bench: 1,774,238,775 ns/iter (+/- 430,690,312)
     * n=20 bench: 3,584,148,475 ns/iter (+/- 467,359,726)
     */
    ScryptParams::new(15, 8, 1)
}