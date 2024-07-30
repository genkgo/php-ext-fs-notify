# PHP Extension for cross-platform filesystem notifications

Uses [PHPER framework](https://github.com/phper-framework/phper) and [notify-rs](https://github.com/notify-rs/notify) 
to build the extension. Supports PHP 8.1, 8.2 and 8.3 for Linux and macOS.

## Build from source

```shell
# For Debian/Ubuntu
sudo apt install gcc make llvm-13-dev libclang-13-dev protobuf-c-compiler protobuf-compiler

# For Alpine Linux
apk add gcc make musl-dev llvm15-dev clang15-dev protobuf-c-compiler

# For MacOS
brew install llvm@13
export LIBCLANG_PATH=$(brew --prefix llvm@13)/lib

# recommended rust setup, see https://www.rust-lang.org/tools/install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# build using cargo
cargo build --release

# move .so (Linux) or .so (MacOS) file to your extension dir
PHP_EXTENSION_DIR=`php -r "echo ini_get('extension_dir');"`
mv target/release/libphp_ext_fs_notify.so ${PHP_EXTENSION_DIR}/fs_notify.so
mv target/release/libphp_ext_fs_notify.dylib ${PHP_EXTENSION_DIR}/fs_notify.dylib

# enable the extension but putting in the ini file
echo 'extension=fs_notify.so' | tee -a /etc/php/${PHP_VERSION}/cli/conf.d/20-fs_notify.ini > /dev/null
```

## Install pre-created .so and .dylib

In the release download the .so (Ubuntu) or .dylib (macOS) file for your PHP version. Lookup the value of your
exension_dir, move the extension into that directory. Enable the extension by putting an ini-file in the conf.d folder 
of your php version. It might look as follows.

```shell
PHP_VERSION=`php -r "echo PHP_MAJOR_VERSION . '.' . PHP_MINOR_VERSION;"`
PHP_EXTENSION_DIR=`php -r "echo ini_get('extension_dir');"`

# download the extension file into the extension dir 
curl -L -o "${PHP_EXTENSION_DIR}/fs_notify.so" "https://github.com/genkgo/php-ext-fs-notify/releases/latest/download/linux-php${PHP_VERSION}-fs_notify.so"

# enable the extension
echo 'extension=fs_notify.so' | tee -a /etc/php/${PHP_VERSION}/cli/conf.d/20-fs_notify.ini > /dev/null
```

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
        return true; // return false if you do not want to continue
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
    
    public function remove(string $path): void;
    
    /**
     * @param callable(Event): bool $handle
     * @throws WatchException
     */
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

class WatchException extends \Exception
{
}
```

## Why?

Why this extension? With the introduction of [native attributes in PHP 8.0](https://www.php.net/manual/en/language.attributes.overview.php),
attributes can be placed in many locations. These attributes might influence which actions are available within your
application. So the application needs to know these attributes before starting the application. Hence, classes need to be scanned
for these attributes.

In our case, the list of folders to scan to see if attribute caches needed to be invalidated, became so large that our
development experience suffered from it. Rather than scanning directories on each request, we decided to invalidate
caches by watching the folders, and invalidate files that actually changed manually. 

## Compile

Make sure you have Rust installed. See the [PHPER introduction](https://docs.rs/phper-doc/latest/phper_doc/_02_quick_start/_01_write_your_first_extension/index.html)
for the required build dependencies.

```shell
# for debug purposes
cargo build
# for production
cargo build --release
# run tests
cargo test
```
