About
=====
I created this because I thought it would be nice to have a program where I could swap between different mod packages. I also don't have much expierience with Rust, and I wanted to make something useful with it.

Windows
=======
- As of January 31st 2024, this will work on Windows, however, instead of backslashes for the path to the ready or not folder in the ini, you must use forward slashes. However, note that windows support could break at any time. There is planned support for Windows in the future.

Building
========
- Run cargo build to build without optimizations and to get debugging symbols
- Run cargo build --release to build with optimizations

Using
=====
- Make sure there is a modswap.ini file in the directory that you run the executable in, replace the ron_dir option with the actual path to your Ready or Not folder:
```ini
[directories]
ron_dir="/home/yourusername/.steam/steam/steamapps/common/Ready Or Not"
```

- Now create a folder with the mods (.pak) files you want to install/swap to, and then run this command
- IMPORTANT: Any mods in the ron_dir/ReadyOrNot/Content/Paks will be deleted, so save them if you want to save them
- Now run this command:
```bash
    ./ron-mod-swapper /path/to/mods
```

Contributing
============
- If you want to contribute, feel free to do so! I will take a look at it whenever I can. However, there are some guidelines

- 4 spaces for indentation
- Use LF, not CRLF

Issues
======
- If you have any issues, please create one and I will take a look at it once I can
