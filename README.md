# Rust :heart: mg

A community-driven port of [mg](https://man.openbsd.org/mg.1) to [Rust](https://www.rust-lang.org).

<!-- markdown-toc start - Don't edit this section. Run M-x markdown-toc-refresh-toc -->
**Table of Contents**

- [Why mg?](#why-mg)
- [Why Rust?](#why-rust)
- [Why A Fork?](#why-a-fork)
- [Contributing](#contributing)

<!-- markdown-toc end -->

# Why mg?

mg is intended to be a small, fast, and portable editor for people who
can't (or don't want to) run Emacs for one reason or another, or are
not familiar with the [vi](https://man.openbsd.org/vi.1) editor.  It
is compatible with Emacs because there shouldn't be any reason to
learn more editor types than Emacs or
[vi](https://man.openbsd.org/vi.1).

# Why Rust?

Rust is a great alternative to C.

Rust has **a fantastic learning curve**. [The documentation is superb](https://doc.rust-lang.org/),
and the community is very helpful if you get stuck.

Rust has **excellent tooling**. The compiler makes great suggestions,
the unit test framework is good, and `rustfmt` helps ensure formatting
is beautiful and consistent.

The Rust **packaging story is excellent**. It's easy to reuse
the great libraries available, and just as easy to factor out code for
the benefit of others. We can replace entire C files in Emacs with
well-maintained Rust libraries.

Code written in Rust **easily interoperates with C**. This means we
can **port to Rust incrementally**, and having a working Emacs at each
step of the process.

Rust provides **many compile-time checks**, making it much easier to write
fast, correct code (even when using multithreading). This also makes
it much easier for newcomers to contribute.

Give it a try. We think you'll like it.

# Why A Fork?

OK, this is not really a fork.  To be honest, this is not even a
re-implementation of mg in Rust: it is just mg compiled with cargo and
some Rust glue.

So what is this all about?  Maybe I'm just mocking
[REmacs](https://github.com/remacs/remacs) or I'm playing with the
`cc` crate and Rust's ffi.  You should stop reading here, it is just a
really bad joke.

But the code runs the latest mg from OpenBSD.  Which is by far the
best editor: small, fast, Emacs-like but without all the Emacs.

# Contributing

If you really want to contribute, go and get the
[mg sources](http://cvsweb.openbsd.org/src/usr.bin/mg/) from OpenBSD.
You can always send mg patches to [tech@openbsd.org](tech@openbsd.org).
