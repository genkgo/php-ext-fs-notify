<?php

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);

$watcher = new FsNotify\RecommendedWatcher();
$watcher->add(__DIR__, recursive: false);
$watcher->watch(function (FsNotify\Event $event) {
    var_dump($event->getKind());
    var_dump($event->getPaths());
    exit;
});
