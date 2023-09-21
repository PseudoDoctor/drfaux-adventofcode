<?php
namespace Onine;

require __DIR__ . '/../vendor/autoload.php';
use Onine\Greetings;
use SplFileInfo;
use SplFileObject;

class Utils
{
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
        // input dirs are ./php/../dayX
        // regular intput is dayX/input.txt
        // test(small) input is dayX/smallinput.txt
        // Some have tiny input dayX/tinyinput.txt
        // input:
        // $day - Day number 
        // $inputType (optional) - default to regular, simple implementation would append to filename i.e. "${inputType}input.txt"

        /*
         * This is delicate, it assumes the tree looks like this:
         * .
         * ├── day1
         * │   ├── input.txt
         * │   └── smallinput.txt
         * ├── day2
         * │   ├── input.txt
         * │   └── smallinput.txt
         * ├── day3
         * │   ├── input.txt
         * │   ├── smallinput.txt
         * │   └── tinyinput.txt
         * ├── php
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
        // $mom should be $project/php/src i.e. /home/mhill/personal/pseudodoctor-advent-2022/php/src
        $project = new SplFileInfo( $mom . "/../../");
        // $project should have day1-25 folders i.e. /home/mhill/personal/pseudodoctor-advent-2022/day1
        $requestedfilename = "{$inputType}input.txt";
        $assumedfilepath = "{$project->getRealPath()}/day{$day}/{$requestedfilename}";

        
        self::logger("Looking for: $assumedfilepath",true);
        if (file_exists($assumedfilepath)) {
            self::logger("File exists",true);
            $f = new SplFileInfo($assumedfilepath);
            return $f->openFile();
        }
        self::logger("Cannot find file for day{$day} {$inputType}input.txt", true);
        return false;
    }
    /**
     * @param string $message
     * @param bool $isverbose
     * @return NULL (outputs based on isverbose)
     */

    public static function logger(string $message, bool $isverbose = false)
    {
        if ($isverbose) {
            echo "[VERBOSE]" . $message;
        } else {
            echo $message;
        }
        echo "\n";
    }

}
