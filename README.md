# PHP Extension for cross-platform filesystem notifications

Uses [PHPER framework](https://github.com/phper-framework/phper) and [notify-rs](https://github.com/notify-rs/notify) 
to build the extension.

## Usage

Create watcher, add paths to watch and start watching with a callback.

```php
$watcher = new FsNotify\RecommendedWatcher();
$watcher->add(__DIR__);
$watcher->add(__DIR__, recursive: false);
$watcher->watch(
    function (FsNotify\Event $event) {
        var_dump($event->getKind()); // kind of file/folder event
        var_dump($event->getPaths()); // paths 
    }
);
```

## Classes

```php
namespace FsNotify;

class RecommendedWatcher
{
    public function __construct();
    
    public function add(string $path, bool $recursive = true): void;
    
    public function watch(callable $handle): void;
}

class Event
{
    private function __construct();
    
    public function getKind(): string;
    
    /**
      * @return array<int, string>
      */
    public function getPaths(): array;
}
```

## Why?

Why this extension? With the introduction of [native attributes in PHP 8.0](https://www.php.net/manual/en/language.attributes.overview.php),
attributes can be placed in many locations. These attributes might influence which actions are available within your
application. So the application needs to know these attributes before starting the application. Hence, classes need to be scanned
for these attributes.

In our case, the list of folders to scan to see if attribute caches needed to be invalidated, became so large that our
development experience suffered from it. Rather than scanning directories on each request, we decided to invalidate
caches by watching the folders.

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