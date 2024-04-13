# Lightscrape-rs

This is a cli that downloads books from websites like lightnovelpub and compiles them into an epub. You should use this with a vpn.

# Install
```sh
cargo install --git https://github.com/Tnixc/lightscrape-rs
```

# Compatible sites

| Website         | Works     |
|--------------|-----------|
| [topnovelupdates.com](https://topnovelupdates.com) | **Yes (fast)** |
| [webnovelpub.pro](https://www.webnovelpub.pro) | **Yes**|
| [webnovelpub.co](https://webnovelpub.co) | **Yes**      |
| [webnoveworld.org](https://www.webnovelworld.org)| **Yes**  | 
| [lightnovelpub.vip](https://lightnovelpub.vip)| **Yes** |

# Features

- [x] Retries download after small delay if site responds with busy
- [x] Async download or chapter by chapter download
- [x] Sources cover image and attaches it to epub
