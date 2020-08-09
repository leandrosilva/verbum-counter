# verbum-counter

This is an implementation of word counting with `Rust` serving `Node.js`.

## The Why

I use `Node.js` on a regular basis. At work, at home, and whatnot.

`Node.js` is a fantastic platform to write server-side applications with the ubiquitous `JavaScript` language. But it does not count `Node.js` off flaws. In fact, most of us know that `Node.js` is not great for heavy computations that could potentially block its event loop. You block the event loop, you got f*d right away. So people try and bring heavy duty tools to power it up. That's when [`C++` addons](https://nodejs.org/api/addons.html) come into the playground.

`C++` being almighty as it is, it's not that simple to tame without a fair count of scars. Then `Rust` enters the playground too. With `Rust` you have **memory safe** and **fearless concurrency** with little to zero cost and no scars. And as you may guess by now, you can write native `Node.js` modules with `Rust` too. That's cool enough to me.

## The How

I just discovered an amazing tool named [`Neon`](https://github.com/neon-bindings/neon), which really pavels the road to write native `Node.js` modules without friction. So I could not help myself but try my hands on it as soon as I could.

The word count algorithm itself is fairly know by anybody that has got a little education on map-reduce. Nothing new here. However, to speed it up, I brought into the mix the awesome [Rayon](https://github.com/rayon-rs/rayon) crate to get data parallelism right out of the box.

## Test

Giving that you already have `Node.js` and `Rust` properly installed, testing this project alone cannot be simpler.

**1) Neon CLI:**

```
$ npm install -g neon-cli
```

**2) This project:**

```
$ git clone https://github.com/leandrosilva/verbum-counter.git
$ cd verbum-counter
$ neon build --release
$ node lib/index.js
```

That's it. I hope you enjoy it as much as I did.