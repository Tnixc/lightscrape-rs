# Lightscrape-rs

This is a cli that asynchronously downloads books from websites like lightnovelpub and compiles them into an epub. You should use this with a vpn.

The output will be at `./res/title.epub`

# Install

```sh
cargo install --git https://github.com/Tnixc/lightscrape-rs
```



https://github.com/Tnixc/lightscrape-rs/assets/85466117/8ea73f86-6d49-4d6c-8858-daadb18a853a



# Compatible sites

| Website                                            | Works          |
| -------------------------------------------------- | -------------- |
| [topnovelupdates.com](https://topnovelupdates.com) | **Yes (fast)** |
| [webnovelpub.pro](https://www.webnovelpub.pro)     | **Yes**        |
| [webnovelpub.co](https://webnovelpub.co)           | **Yes**        |
| [webnoveworld.org](https://www.webnovelworld.org)  | **Yes**        |
| [lightnovelpub.vip](https://lightnovelpub.vip)     | **Yes**        |

# Features

- [x] Retries download after small delay if site responds with busy
- [x] Async download or chapter by chapter download
- [x] Sources cover image and attaches it to epub

# Credits

- Uses [mdbook-epub](https://github.com/Michael-F-Bryan/mdbook-epub) for epub writing

---

This is a more feature complete rust rewrite of [lightscrape](https://github.com/tnixc/lightscrape) which I wrote in typescript a few months ago.
