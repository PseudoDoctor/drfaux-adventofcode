<?php

namespace Onine;

require __DIR__ . '/../vendor/autoload.php';

// https://adventofcode.com/2022/day/1
// input list blocks separated by blank line (or eof)
// Part One - Sum each block, return Max sum
// Part Two - Sum each block, return sum of top 3 blocks


$utils = new Utils();

/**
 * Set manually. If true, bypasses VERBOSE and QUIET requests
 */
$debug = false;
if ($debug) {
    $utils->logState = LogLevel::DEBUG;
    var_dump($argv);
} else {
    $utils->logState = LogLevel::INFO;
}
/**
 * Default operations on small dataset
 */
define("DEFAULTDATASET", "small");
$dataset = DEFAULTDATASET;

class Day1
{

    public static function partOne(bool $usePartTwo = false)
    {
        if ($usePartTwo) {
            return self::partTwo(1);
        } else {
            global $dataset, $utils;
            // https://adventofcode.com/2022/day/1
            // input list blocks separated by blank line (or eof)
            // Part One - Sum each block, return Max sum
            $inputObject = Utils::getinput(1, $dataset);
            if ($inputObject == false) {
                $utils::logger("FAIL", LogLevel::WARNING);
            }
            $biggestElf = 0;
            $currentElf = 0;
            foreach ($inputObject as $k => $line) {
                // if ($debug || $verbose) {
                //   var_dump($k);
                //   var_dump($line);
                // }
                //$utils::logger("Parsing line: " . ($inputObject->key() + 1) . ': ' . $inputObject->current());
                $utils::logger("Parsing line: $line");
                $utils::logger("Biggest elf: $biggestElf");
                $utils::logger("Current Elf:$currentElf");
                # is empty string? compare and set biggest elf from current elf
                if ($line == "\n") {
                    $utils::logger("End of elf: $currentElf");
                    if ($currentElf >= $biggestElf) {
                        $utils::logger("Setting biggestElf");
                        $biggestElf = $currentElf;
                    }
                    # reset current elf
                    $currentElf = 0;
                }
                # otherwise add to currentelf
                $lineValue = intval($line);
                $utils::logger("adding $lineValue to $currentElf");
                $currentElf = $currentElf + $lineValue;
            }
            return $biggestElf;
        }
    }
    public static function partTwo(int $topCount = 3): int
    { // https://adventofcode.com/2022/day/1
        // input list blocks separated by blank line (or eof)
        // Part Two - Sum each block, return sum of top 3 blocks
        global $dataset, $utils;
        $inputObject = Utils::getinput(1, $dataset);
        $elfs = array();
        $currentElf = 0;
        foreach ($inputObject as $k => $line) {
            // if ($debug || $verbose) {
            //   var_dump($k);
            //   var_dump($line);
            // }
            //$utils::logger("Parsing line: " . ($inputObject->key() + 1) . ': ' . $inputObject->current());
            $utils::logger("Parsing line: $line");
            if ($line == "\n") {
                $utils::logger("End of elf: $currentElf");
                array_push($elfs, $currentElf);
                $currentElf = 0;
            } else {
                $lineValue = intval($line);
                $utils::logger("adding $lineValue to $currentElf");
                $currentElf = $currentElf + $lineValue;
            }
        }
        print_r($utils);
        $log = $utils::$logState;
        // if ($utils::$logState->value >= LogLevel::VERBOSE->value) {
        //   $utils::logger("All unsorted elfs: ", LogLevel::VERBOSE);
        //   print_r($elfs);
        // }
        rsort($elfs);
        // if ($verbose || $debug) {
        //  $utils::logger("All sorted elfs: ");
        //   print_r($elfs);
        // }
        $total = 0;
        $count = count($elfs);
        $counted = 0;
        $utils::logger("add");
        for ($i = 0; $i < $topCount; $i++) {
            $utils::logger("Adding {$elfs[$i]} to {$total}");
            $total = $total + $elfs[$i];
        }

        return $total;
    }
}

$input = Utils::getinput(1, $dataset);
echo "day1 $dataset";
var_dump($input);
echo "\n";
$utils::logger("Biggest elf: " . Day1::partOne());
$utils::logger("Top 3 elfs total: " . Day1::partTwo());
$utils::logger("Top 1 elf total: " . Day1::partOne(true));
