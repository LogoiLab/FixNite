#Project FixNite

A fortnite replays parser and compatibility layer for older versions of Epic's `.replay` file format.

###This project is in infancy!

It currently only functions for upgrading 3.4 replays to 3.5 replays. The chances of anyone running these versions of the game is...nonexistent... But we're making progress.

##Compiling

You're gonna need the Rust toolchain you can get it here:

[rustup.rs](rustup.rs)
<hr>
Go ahead and clone the repo:

`git clone https://www.github.com/LogoiLab/FixNite.git`

For those of you who like real security:

`git clone git@github.com:LogoiLab/FixNite.git`
<hr>
Go ahead and cd into the directory:

`cd ./FixNite`
<hr>
And run the following command to build the project:

`cargo build --release`

Watch the pretty text and pray for no errors.
<hr>
Now you can find the binary at:

`./target/release/fixnite`

To run the program do the following:

`./fixnite <some_random_replay_file.replay>`

Where `<some_random_replay_file.replay>` is replaced with the path to a replay you want fixed.
