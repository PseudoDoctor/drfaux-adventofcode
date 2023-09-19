<?php
namespace Onine;

require __DIR__ . '/../vendor/autoload.php';
use Onine\Greetings;

class Utils
{
    public static function hello()
    {
        echo Greetings::sayHelloWorld();
        echo "\n";
    }

    /**
     * @param int $day Day number, either Integer 1-25 
     * @param string $inputType OPTIONAL appends to input.txt i.e. "small" retrieves smallinput.txt , "" retrieves input.txt
     * @param bool $verbose OPTIONAL Verbose output
     * @return string contents of requested input file
     */
    public static function getinput(int $day, string $inputType = "", bool $verbose = false): string|bool
    {
        // This dir is ./php/src/
        // input dirs are ../../dayX
        // regular intput is ../../dayX/input.txt
        // test(small) input is ../../dayX/smallinput.txt
        // Some have tiny input ../../dayX/tinyinput.txt
        // input:
        // $day - Day number, either "day1" or "1"
        // $inputType (optional) - default to regular, simple implementation would append to filename i.e. "${inputType}input.txt"

        Utils::logger("Cannot find file for day${day} ${inputType}input.txt", true);
        return false;
    }
    /**
     * @param string $message
     * @param bool $isverbose
     * @return NULL (outputs based on isverbose)
     */

    public static function logger(string $message, bool $isverbose = false)
    {
        if (!$isverbose) {
            echo "[VERBOSE]" . $message;
        } else {
            echo $message;
        }
        echo "\n";
    }
}

?>