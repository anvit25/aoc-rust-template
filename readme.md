Place a file called AOC_SESSION_COOKIE.pvt in the main directory containing the session cookie. This allows input files to be automatically downloaded. 

The download functionality is implemented in [`aoc-utils-repository`](https://github.com/anvit25/aoc-utils-rust), the path of which should be specified in the `Cargo.toml` file.

Everytime you add a day, you should modify the mod.rs file with `pub mod day{n};` and also update the run_day funcion. I was trying to turn that into a macro but couldn't figure it out 🤷.