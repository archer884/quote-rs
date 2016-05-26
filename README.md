QuoteRS
=======

[![Build Status](https://travis-ci.org/archer884/quoters.svg?branch=master)](https://travis-ci.org/archer884/quoters)

> Provides a bare bones wrapper for the quote API at [theysaidso.com][tss]

Right now I provide two API implementations, sort of... I mean, they aren't finished, but the idea is that one of them will provide access to the public API and the other will provide access to the private API. In the long run I may just combine those two and make the API key optional and then return an error from the private methods if you should happen to call one without an API key... Whatever.

## TLS

QuoteRS provides *no* TLS support as of right now; I have that feature turned off in [hyper][hyp] and I have no intention of turning it back on. This is because the API does not support secure connections. If you can think of a reason to have TLS support, I'm all ears.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE][APC] or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT][MIT] or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[tss]:https://theysaidso.com/api/
[hyp]:https://github.com/hyperium/hyper
[APC]:https://github.com/archer884/quoters/blob/master/LICENSE-APACHE
[MIT]:https://github.com/archer884/quoters/blob/master/LICENSE-MIT