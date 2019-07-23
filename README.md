# Heneiken Music Player

Web-based music player as part of rust-learning-journal

![Screenshot](https://raw.githubusercontent.com/codenoid/heineken-music-player/master/assets/screenshot-1.png)

## Installation

1. Install Rust
2. run : 

```
git clone https://github.com/codenoid/heineken-music-player
cd heineken-music-player
cargo run # or cargo run --release
# access localhost:3003
```

## App Information

1. This application doesn't need a file-based database, i use lazy_static
2. currently only support .mp3 file, which are being loaded from [assets/music](https://github.com/codenoid/heineken-music-player/tree/master/assets/music)
3. you can refresh music my by accessing `/refresh` or press refresh button
4. automatically play next track

## Credits

* HTML Template are from [here](https://codepen.io/jinnrw/pen/ggpgVe)