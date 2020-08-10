# verbum-counter

This is an implementation of word counting with `Rust` serving `Node.js`.

## The Why

I use `Node.js` on a regular basis. At work, at home, you name it.

`Node.js` is a fantastic platform to write server-side applications with the ubiquitous `JavaScript` language. But it does not count `Node.js` off flaws. In fact, most of us know that `Node.js` is not great for heavy computations that could potentially block its event loop. You block the event loop, you got f*d right away. So people try and bring heavy duty tools to power it up. That's when [`C++` addons](https://nodejs.org/api/addons.html) come into the playground.

`C++` being almighty as it is, it's not that easy to tame without a fair count of scars. Then `Rust` enters the playground too. With `Rust` you have **memory safe** and **fearless concurrency** with little to zero cost and no scars. And as you may guess by now, you can write native `Node.js` modules with `Rust` too. That's cool enough to me.

## The How

I just discovered an amazing tool named [`Neon`](https://github.com/neon-bindings/neon), which really pavels the road to write native `Node.js` modules without friction. So I could not help myself but try my hands on it as soon as I could.

The word count algorithm itself is fairly known by anybody that has got a little education on map-reduce. Nothing new here. However, to speed it up, I brought into the mix the awesome [Rayon](https://github.com/rayon-rs/rayon) crate to get data parallelism right out of the box.

## Test

Giving that you already have `Node.js` and `Rust` properly installed, testing this project alone cannot be simpler.

**1) Get Neon CLI:**

```
$ npm install -g neon-cli
```

**2) Get this project:**

```
$ git clone https://github.com/leandrosilva/verbum-counter.git
```
**3) Build this project:**

```
$ cd verbum-counter
$ neon build --release
```

**4) Try it out:**

```
$ node lib/wordcount_node.js
$ node lib/wordcount_node.js /path/to/a/small/textfile
$ node lib/wordcount_node.js /path/to/a/medium/textfile
$ node lib/wordcount_node.js /path/to/a/big/textfile
```

```
$ node lib/wordcount_rust.js
$ node lib/wordcount_rust.js /path/to/a/small/textfile
$ node lib/wordcount_rust.js /path/to/a/medium/textfile
$ node lib/wordcount_rust.js /path/to/a/big/textfile
```

As you may note, with small text files plain `Node.js` shines and things start to turn the other way around only when files get bigger and bigger. Therefore you should not rush into optimize unless you really need it. And, of course, bare in mind that the idea is always let the event loop runs free.

That's it. I hope you have enjoyed this tiny experiment as much as I did.
