# Leak? with wee-alloc, yew-wasm-pack-template **and yew 14.x**

The problem remported here is not demonstrated by this repository and
_may_ no longer exist.  After upgrading crates and packages to the
latest, except leaving yew pinned to 0.14 the bug was present on March
26th.  However, by then yew 0.19 is current and once I upgraded to
that, the bug doesn't appear.

## Background

This is the smallest demo I could come up with "quickly" that shows what
appears to be a leak when using wee-alloc in a project configured per
yew-wasm-pack-template.  It is the result of me tracking down a memory leak
in a client that I had written using std-web and converted to web-sys using
yew-wasm-pack-template as a template.

FWIW, from my POV, this same program, but configured to run with wasm-pack
did not show the leak, even with it using wee-alloc.  I have not looked into
what is different between the two environments, because _my guess_ is there
are others who would know off the top of their head.  I don't mind spending
more time trying to find out more about this bug, but I don't want to spend
hours of my time doing something someone else can do in seconds.

### with wee-alloc

The source code has wee-alloc set as the global allocator at the end
of src/lib.rs.  If you use `yarn start:dev` and go to
`http://localhost:8000` your browser will start using a lot of WASM
memory.  If you use Chrome, you can use the `Task Manager` window to
see how much memory is used.  It may take a few tens of seconds, but
you'll probably get an `Uncaught (in promise) RuntimeError` coming
from the `rust_oom`, and if you're using `Task Manager`, you'll see
about 2 GB of memory consumed.

You'll also get around 1,500 lines of "When in the course of human
events" on your web page.

### without wee-alloc

If you comment out the `#[global_allocator]` line, close the previous
run (if it's still open), open a new window or tab and go to
`http://localhost:8000` you'll still get a big pause and perhaps your
fan will run (it's a debug build; this same problem manifests in a
release build; it's just that using them requires some sort of server),
but you'll get 1,500 lines of "When in the course...", there will be no
error on the JavaScript Console and the amount of memory you'll be using
once you get to steady state will be on the order of 50 MB.

## Questions? Ask

I realize there's not a lot of info here. I also realize that
wee-alloc never shrinks.  However, I don't think this is simply a case
of wee-alloc never shrinking, because although the log_data constantly
grows upward, it only grows to 1,500 lines.  I recognize that if every
time you allocate a little more memory than you had before, you
potentially leave a block that's not going to ever be usable again for
that particular data structure (since it continues to grow) but as
those left behind pieces get larger and larger, I would think they
could be used for smaller objects. But beyond that, since this doesn't
happen with wasm-pack, I think something more is going on.

Anyway, I hope the information I'm providing is useful.  I'm happy to
provide more information and I feel bad submitting PRs for things like
this, because I really don't think it's a yew issue, per-se, and
wee-alloc isn't enabled by default, but I also don't want to spam a
bunch of different repositories with this info, so I figure making a
sample program and reporting it once to _someone_ is _probably_ the
right thing to do.





