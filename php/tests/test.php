<?php

// Autoload files using the Composer autoloader.
require_once __DIR__ . '/../vendor/autoload.php';

use Onine\Greetings;

echo Greetings::sayHelloWorld();

$testPaths = array (
    "src/Utils.php",
    ".",
    "./../",
    __FILE__
);
// foreach ($testPaths as $file_name) {
//     if(file_exists($file_name)){
//         $splfileinfo = new \SplFileInfo($file_name);
//         // var_dump($splfileinfo->getFileInfo());
//         echo "file '{$splfileinfo->getRealPath()}'\n";
//         echo "parent '{$splfileinfo->getPath()}'\n";
//     } else {
//         echo "no find '{$file_name}'\n";
//     }
// }
$me = new SplFileInfo(__FILE__);
$mom = $me->getPath();
// $mom should be $project/php/src i.e. /home/mhill/personal/pseudodoctor-advent-2022/php/src
$project = new SplFileInfo( $mom . "/../../");

// $project should have day1-25 folders i.e. /home/mhill/personal/pseudodoctor-advent-2022/day1
    echo "file '{$project->getRealPath()}'\n";
    echo "parent '{$project->getPath()}'\n";
