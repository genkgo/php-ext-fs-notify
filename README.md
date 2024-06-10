# PHP Extension for cross-platform filesystem notifications

Uses [PHPER framework](https://github.com/phper-framework/phper) and [notify-rs](https://github.com/notify-rs/notify) 
to build the extension.

## Usage

Create watcher, add paths to watch and start watching with a callback.

```php
$watcher = new FsNotify\RecommendedWatcher();
$watcher->add(__DIR__);
$watcher->add(__DIR__, recursive: false);
$watcher->watch(function (FsNotify\Event $event) {
    var_dump($event); // dumps instance of FsNotify\Event
});
```

## Compile

Make sure you have Rust installed. See the [PHPER introduction](https://docs.rs/phper-doc/latest/phper_doc/_02_quick_start/_01_write_your_first_extension/index.html)
for the required build dependencies.

```shell
# for debug purposes
cargo build
# for production
cargo build --release
```

Copy `target/{debug|release}/libphp_ext_fs_notify.so` into your .so into your extension directory and add it to your php.ini.

```ini
extension=libphp_ext_fs_notify.so
```
