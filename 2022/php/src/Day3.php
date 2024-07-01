<?php

namespace Onine;

require __DIR__ . '/../vendor/autoload.php';

use SplFileObject;

// $dataset = 'small';
$verbose = true;
$dataset = '';
// $verbose = false;

$logState = LogLevel::INFO;
if ($verbose) {
    $logState = LogLevel::VERBOSE;
}

/*
 * vJrwpWtwJgWrhcsFMMfFFhFp
 * jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
 * PmmdzqPrVvPwwTWBwg
 * wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
 * ttgJtRGJQctTZtZT
 * CrZsJsPPZsGzwwsLwLmpwMDw
 *
 * The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp,
 *     which means its first compartment contains the items vJrwpWtwJgWr,
 *     while the second compartment contains the items hcsFMMfFFhFp.
 *     The only item type that appears in both compartments is lowercase p.
 * The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL.
 *     The only item type that appears in both compartments is uppercase L.
 * The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg;
 *     the only common item type is uppercase P.
 * The fourth rucksack's compartments only share item type v.
 * The fifth rucksack's compartments only share item type t.
 * The sixth rucksack's compartments only share item type s.
 *
 * Lowercase item types a through z have priorities 1 through 26.
 * Uppercase item types A through Z have priorities 27 through 52.
 *
 * In the above example, the priority of the item type that appears in
 *     both compartments of each rucksack is
 *     16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s);
 *     the sum of these is 157.
 */
class Day3
{
    /**
     * partOne
     */
    public static function partOne(SplFileObject $in, bool $verbose = false): int
    {
        $sum = 0;
        foreach ($in as $k => $line) {
            $input = trim($line);
            $len = strlen($input);
            if ($len >= 2) {
                $char = self::duplicatedCharFromHalfs($input);
                $sum = $sum + self::day3Priority($char);
            }
        }
        Utils::logger($sum, LogLevel::ALWAYS);
        return $sum;
    }

    /**
     * partTwo
     * Every set of three lines in your list corresponds to a single group, but each group
     *  can have a different badge item type. So, in the above example, the first group's
     *  rucksacks are the first three lines:
     *
     * vJrwpWtwJgWrhcsFMMfFFhFp
     * jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
     * PmmdzqPrVvPwwTWBwg
     * And the second group's rucksacks are the next three lines:
     *
     * wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
     * ttgJtRGJQctTZtZT
     * CrZsJsPPZsGzwwsLwLmpwMDw
     * In the first group, the only item type that appears in all three rucksacks is lowercase r;
     *  this must be their badges. In the second group, their badge item type must be Z.
     *
     * Sum of the priority of r and Z is 18 + 52 = 70
     */
    public static function partTwo(SplFileObject $in, bool $verbose = false): int
    {
        $sum = 0;
        $elfArray = array();
        foreach ($in as $k => $line) {
            $input = trim($line);
            $elfArray[] = $input;
            // self::logger('elfs: ' . count($elfArray));
            if (count($elfArray) >= 3) {
                // process
                $char = self::duplicatedCharsInStrings($elfArray);
                self::logger(" Dupes = '$char' from elfs $elfArray[0] $elfArray[1] $elfArray[2]");
                $sum += self::day3Priority($char);
                // TODO
                // reset
                $elfArray = array();
            }
        }

        return $sum;
    }

    public static function logger(string $message, $logLevel = LogLevel::VERBOSE)
    {
        global $logState;

        Utils::logger($message, $logLevel, $logState);
    }

    /**
     * Uses #day3Priority to print all alphabet A-Z a-z and their associated "Priority"
     */
    public static function alphaOrdList()
    {
        $alphabet = array_merge(range('a', 'z'), range('A', 'Z'));
        foreach ($alphabet as $key => $letter) {
            self::logger($letter . ' ord ' . ord($letter) . ' priority ' . self::day3Priority($letter));
        }
    }

    /**
     * Lowercase item types a through z have priorities 1 through 26.
     * Uppercase item types A through Z have priorities 27 through 52.
     * @param string $char
     * @return int 1-52
     */
    public static function day3Priority(string $char): int
    {
        /*
         * ord(a) = 97  return  1
         * ord(z) = 122 return 26
         * ord(A) = 65  return 27
         * ord(Z) = 90  return 52
         */
        if (strlen($char) > 1) {
            self::logger("too many chars '$char' , only looking at " . $char[0], LogLevel::WARNING);
        }
        // Keep processing even on error, ord($char) will only return on the first char
        $ord = ord($char);

        // Valid range check.
        if (!(($ord >= 65 && $ord <= 90) || ($ord >= 97 && $ord <= 122))) {
            self::logger("OUT OF RANGE '$char[0]'");
        }

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
        self::logger("$char[0] ord $ord priority $priority", LogLevel::DEBUG);
        return $priority;
    }

    /**
     * Takes string, splits in half, compares halfs, returns the char that's duplicated between halfs
     * aabcbc returns b
     * WantWIST returns W
     */
    public static function duplicatedCharFromHalfs(string $input): string
    {
        $half = strlen($input) / 2;
        $char = '';
        self::logger("String '$input' len $half");
        $firstHalf = substr($input, 0, $half);
        $secondHalf = substr($input, $half);
        self::logger("1 $firstHalf");
        self::logger("2 $secondHalf");
        for ($i = 0; $i < $half; $i++) {
            for ($j = 0; $j < $half; $j++)
                if (ord($firstHalf[$i]) == ord($secondHalf[$j])) {
                    $char = $firstHalf[$i];
                };
        }
        self::logger("Dupe $char");
        return ($char);
    }

    public static function duplicatedCharsInStrings(array $input): string
    {
        $output = '';
        $temp = '';
        if (count($input) > 2) {
            self::logger('Nesting ');
            $temp = self::duplicatedCharsInStrings(array_slice($input, 1));
            $output = self::duplicatedCharsInStrings(array($temp,$input[0]));
        } else {
            self::logger("array $input[0] $input[1]");
            for ($i = 0; $i < strlen($input[0]); $i++) {
                for ($j = 0; $j < strlen($input[1]); $j++) {
                    if (ord($input[0][$i]) == ord($input[1][$j])) {
                        $output = $output . $input[0][$i];
                    }
                }
            }
        }
        return $output;
    }
}

// Day3::alphaOrdList();
// Day3::day3Priority("asdf");
// Day3::day3Priority(".lkj");
// Day3::day3Priority(",");
// Day3::day3Priority("jkl'");
// Day3::day3Priority("D");

$inputObject = Utils::getinput(3, $dataset);
// Day3::logger(Day3::partOne($inputObject), LogLevel::INFO);
Day3::logger(Day3::partTwo($inputObject), LogLevel::INFO);
