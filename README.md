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
- Merge 5 or more bubbles together and they pop!
- You lose if you run out of swaps.

# History

I started working on this game as a hobby project way back in 2004. I always meant to get it working well enough to publish it and even got it into beta, but I never got it 100% right. I went through many iterations before I hit on the current system of approximating the bubble walls with cubic Bézier curves and using a full Euler-Lagrange technique. You can read more about the underlying physics and math in [bubblemath.pdf](docs/bubblemath.pdf).

In 2018, Stu Denman at Pine Street Codeworks independently developed a similar idea and published [Tiny Bubbles](https://play.google.com/store/apps/details?id=com.pinestreetcodeworks.TinyBubbles&hl=en_US) which won many well-deserved awards. Honestly, it's a lot better than my game ever was, and I feel a tiny bit vindicated that he proved the idea was a good one, even if I never found time to properly execute it.

Ultimately, I'm pretty sure the reason it took so long was that I made the classic programmer mistake of using the tools I was familiar with (Java) instead of the right tools for the job (C++, which I have always hated). With [Gemini CLI](https://github.com/google/gemini-cli) (which [I also worked on](https://github.com/google-gemini/gemini-cli/graphs/contributors)!), I was able to port this to Rust where it does not suffer from the performance issues that dogged the previous versions.

## Gameplay

Previous iterations have had different rules. Most allowed you to swap any two bubbles. Swapping any adjacent items is a popular mechanic in lots of games (like Bejeweled) but it doesn't work well in a bubble graph since by default they form hex grids which are much more connected than square ones. Joining like-colored bubbles and popping has been a feature almost from the beginning because it looks cool.
