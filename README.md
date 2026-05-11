# PipPop

A 2-D bubble-swapping puzzle game without a grid! Join 5 bubbles of the same color to make them pop!

<div align="center">

[![PipPop Gameplay Demo](demo.gif)](https://scidomino.github.io/pippop/)

</div>

# How to play

<p align="center">
  <a href="https://scidomino.github.io/pippop/">
    <strong>Click here to play!</strong>
  </a>
</p>

- Click a bubble bordering the empty space to swap it in.
- If the newly swapped bubble touches a like-colored bubble, they merge.
- **Every merge grants you an extra swap (up to a maximum of 5)!**
- Merge 5 or more bubbles together and they pop!
- String together multiple merges or pops to trigger chain bonuses.
- You lose if you run out of swaps.

# History

I started working on this game as a hobby project way back in 2004. I always meant to get it working well enough to publish it and even got it into beta, but I never got it 100% right. I went through many iterations before I hit on the current system of approximating the bubble walls with cubic Bézier curves and using a full Euler-Lagrange technique. You can read more about the underlying physics and math in [bubblemath.pdf](docs/bubblemath.pdf).

In 2018, Stu Denman at Pine Street Codeworks independently developed a similar idea and published [Tiny Bubbles](https://play.google.com/store/apps/details?id=com.pinestreetcodeworks.TinyBubbles&hl=en_US) which won many well-deserved awards. Honestly, it's a lot better than my game ever was, and I feel a tiny bit vindicated that he proved the idea was a good one, even if I never found time to properly execute it.

Ultimately, I'm pretty sure the reason it took so long was that I made the classic programmer mistake of using the tools I was familiar with (Java) instead of the right tools for the job (C++, which I have always hated). With [Gemini CLI](https://github.com/google/gemini-cli) (which [I also worked on](https://github.com/google-gemini/gemini-cli/graphs/contributors)!), I was able to port this to Rust where it does not suffer from the performance issues that dogged the previous versions.

## Gameplay

Previous iterations have had different rules. Most allowed you to swap any two bubbles. Swapping any adjacent items is a popular mechanic in lots of games (like Bejeweled) but it doesn't work well in a bubble graph since by default they form hex grids which are much more connected than square ones. Joining like-colored bubbles and popping has been a feature almost from the beginning because it looks cool.

# Building

## Prerequisites
Before building for any platform, ensure you have the Rust toolchain installed (version 1.91+ is recommended). You can install it via [rustup](https://rustup.rs/).

## Running Locally (Desktop)
To run the game natively on macOS, Windows, or Linux, run the following command from the root of the repository:
```bash
cargo run --manifest-path rust/Cargo.toml --release
```
*(Using `--release` ensures the physics and rendering run at maximum performance).*

## Building for Android
Building the installable `.apk` requires setting up the Android build environment.

**1. Install Rust Mobile Targets**
Add the cross-compilation targets for Android:
```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

**2. Install the Android SDK & NDK**
You must install the Android SDK and the **NDK (Side by side)**. The easiest way to get these is by downloading Android Studio and using the built-in SDK Manager.

**3. Configure Environment Variables**
Export the paths to your SDK and NDK in your terminal. (The paths below are typical for macOS; adjust the NDK version to match your installation):
```bash
export ANDROID_HOME=$HOME/Library/Android/sdk
export NDK_HOME=$ANDROID_HOME/ndk/YOUR_NDK_VERSION
```

**4. Install the Packaging Tool**
Install the latest version of `cargo-quad-apk` directly from GitHub to ensure compatibility with modern Cargo lockfiles:
```bash
cargo install --git https://github.com/not-fl3/cargo-quad-apk --force
```

**5. Build the APK**
Navigate into the `rust` directory and run the build:
```bash
cd rust
cargo quad-apk build --release
```
The final `.apk` will be located at `rust/target/android-artifacts/release/apk/rust.apk`.
