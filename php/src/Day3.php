<?php

namespace Onine;

require __DIR__ . '/../vendor/autoload.php';

use SplFileObject;


$dataset = "small";
$verbose = true;
$logState = LogLevel::INFO;
if ($verbose) {
    $logState = LogLevel::VERBOSE;
}
/*
    vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw
    
    The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items vJrwpWtwJgWr,
     while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
    The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both 
     compartments is uppercase L.
    The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
    The fourth rucksack's compartments only share item type v.
    The fifth rucksack's compartments only share item type t.
    The sixth rucksack's compartments only share item type s.

    Lowercase item types a through z have priorities 1 through 26.
    Uppercase item types A through Z have priorities 27 through 52.

    In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L)
      , 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.
 */
class Day3
{
    /** partOne
     * 
     */
    public static function partOne(SplFileObject $in, bool $verbose = false): int
    {
        $sum = 0;
        foreach ($in as $k => $line) {
        }
        Utils::logger($sum, LogLevel::ALWAYS);
        Utils::logger($sum, LogLevel::ALWAYS);
        return $sum;
    }
    /** partTwo
     *
     */
    public static function partTwo(SplFileObject $in, bool $verbose = false): int
    {
        foreach ($in as $k => $line) {
        }

        return 0;
    }

    public static function logger(string $message, $logLevel = LogLevel::VERBOSE)
    {
        global $logState;
        Utils::logger($message, $logLevel, $logState);
    }

    public static function alphaOrdList()
    {

        $alphabet = array_merge(range('a', 'z'), range('A', 'Z'));
        foreach ($alphabet as $key => $letter) {
            self::logger($letter." ord ". ord($letter) . " priority ". self::day3Priority($letter));
        }
    }
    /**
     * Lowercase item types a through z have priorities 1 through 26.
     * Uppercase item types A through Z have priorities 27 through 52.
     * @param string $char 
     * @return int
     */
    public static function day3Priority(string $char): int
    {
        /*
         *
         * ord(a) = 97 return 1
         * ord(z) = 122 return 26
         * ord(A) = 65 return 27
         * ord(Z) = 90 return 52
         */
        if (strlen($char) > 1) {
            self::logger("too many chars '$char'", LogLevel::WARNING);
        }
        # Keep processing even on error, ord($char) will only return on the first char
        $ord = ord($char);

        // TODO: Valid ranges. This should be invalid, [EXCESSIVE VERBOSE DEBUG] ; ord 59 priority 21

        // ord(a) 97 - 96 = 1
        // ord(Z) 90 - 96 = -6
        $priority = $ord - 96;
        if ($priority < 1) {
            // A-Z need to be shifted up.
            // + 32 puts A at 1
            // + 26 puts A at 27
            // ord(Z) + 32 + 26 = 52
            $priority = $priority + 32 + 26;
        }
        self::logger("$char ord $ord priority $priority", LogLevel::DEBUG);
        return $priority;
    }
}

Day3::alphaOrdList();
Day3::day3Priority("asdf");
Day3::day3Priority(";lkj");
Day3::day3Priority(";");
Day3::day3Priority("jkl;");
Day3::day3Priority("D");



// $inputObject = Utils::getinput(3, $dataset);
// Day3::logger(Day3::partOne($inputObject), LogLevel::INFO);
// Day3::logger(Day3::partTwo($inputObject), LogLevel::INFO);
