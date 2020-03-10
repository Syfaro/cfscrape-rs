# cfscrape-rs

A questionable wrapper around [cfscrape](https://github.com/Anorov/cloudflare-scrape).

It requires running Rust in such a way that cfscrape is available to the active Python interpreter. You may source a virtualenv or install it globally.

I am using this successfully, but am unsure what improvements are possible.

```rust
use cfscrape::get_cookie_string;

let (cookies, user_agent) = get_cookie_string("https://www.google.com", None)?;
// Now you can use the cookie header string and user agent to make requests.
```
