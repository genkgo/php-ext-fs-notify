<?php

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(-1);

try {
    $watcher = new FsNotify\RecommendedWatcher();
    $watcher->add(__DIR__ . '/unknown');
    $watcher->watch(fn () => null);
} catch (FsNotify\WatchException) {
    echo "caught exception";
}
