<?php

namespace Onine;

require __DIR__ . '/../vendor/autoload.php';


use Onine\Greetings;
use SplFileInfo;
use SplFileObject;

class Utils
{
    /**
     * if @link logger request is greater than this, log it
     * <pre>    case DEBUG = 1;
     *     case VERBOSE = 20;
     *     case INFO = 50; (DEFAULT)
     *     case QUIET = 75;
     *     case WARNING = 98;
     *     case ALWAYS = 99;</pre>
     * @var LogLevel
     */
    public LogLevel $logState = LogLevel::INFO;
    public static function hello()
    {
        echo Greetings::sayHelloWorld();
        echo "\n";
    }

    /**
     * @param int $day Day number, appends to "day" to become folder name. Literally `"day" . $day` i.e. day1, day12, day25
     * @param string $inputType OPTIONAL DEFAULT `""`. Prepends to "input.txt" to become filename. Literally `$inputType . "input.txt"` i.e. DEFAULT input.txt, smallinput.txt, tinyinput.txt
     * @param bool $verbose OPTIONAL DEFAULT `false`. Enable/Disable verbose output.
     * @return SplFileObject contents of requested input file
     */
    public static function getinput(int $day, string $inputType = "", bool $verbose = false): SplFileObject|bool
    {

        // This dir is ./php/src/
        // input dirs are ./php/../day/X
        // regular intput is day/X/input.txt
        // test(small) input is day/X/smallinput.txt
        // Some have tiny input day/X/tinyinput.txt
        // input:
        // $day - Day number
        // $inputType (optional) - default to regular, simple implementation would append to filename i.e. "${inputType}input.txt"

        /*
         * This is delicate, it assumes the tree looks like this:
         * .
         * ├── day/1
         * │   ├── input.txt
         * │   └── smallinput.txt
         * ├── day/2
         * │   ├── input.txt
         * │   └── smallinput.txt
         * ├── day/3
         * │   ├── input.txt
         * │   ├── smallinput.txt
         * │   └── tinyinput.txt
         * ├── ..
         * │   ├── composer.json
         * │   ├── README.md
         * │   ├── src
         * │   │   ├── Day1.php
         * │   │   ├── Greetings.php
         * │   │   └── Utils.php
         * │   ├── tests
         * │   │   └── test.php
         * │   └── vendor
         * │       ├── autoload.php
         * │       └── composer
         * │           ├── autoload_classmap.php
         * --snip--
         */
        $me = new SplFileInfo(__FILE__);
        $mom = $me->getPath();
        // $mom should be $project/src i.e. /home/mhill/personal/pseudodoctor-advent-2022/2023/src
        $project = new SplFileInfo($mom . "/../");
        // $project should have day1-25 folders i.e. /home/mhill/personal/pseudodoctor-advent-2022/day1
        $requestedfilename = "{$inputType}input.txt";
        $assumedfilepath = "{$project->getRealPath()}/day/{$day}/{$requestedfilename}";



        self::logger("Looking for: $assumedfilepath", LogLevel::VERBOSE);
        if (file_exists($assumedfilepath)) {
            self::logger("File exists", LogLevel::VERBOSE);
            $f = new SplFileInfo($assumedfilepath);
            return $f->openFile();
        }
        self::logger("Cannot find file for day{$day} {$inputType}input.txt", LogLevel::WARNING);
        return false;
    }


    /**
     * Compares $logState to $logLevel. If $logLevel >= $lotState, log it with prefix, unless ALWAYS then no prefix.
     *
     * @param string $message
     * @param LogLevel $logLevel
     * @param LogLevel $logState
     * @return void
     */
    public static function logger(
        string $message,
        LogLevel $logLevel = LogLevel::INFO,
        LogLevel $logState = LogLevel::INFO
    ): void {
        echo "[EXCESSIVE " . $logState->name . " " . $logLevel->name ."] " . $message . "\n";
    }
}



