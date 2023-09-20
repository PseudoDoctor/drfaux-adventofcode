<?php

namespace Onine;

require __DIR__ . '/../vendor/autoload.php';

use Onine\Utils;

// https://adventofcode.com/2022/day/1
// input list blocks separated by blank line (or eof)
// Part One - Sum each block, return Max sum
// Part Two - Sum each block, return sum of top 3 blocks



/**
 * Set manually, see {@link logger}
 */
$debug = true;
if ($debug) {
  var_dump($argv);
}
/**
 * Default operations for these tasks is to be noisy
 */
$verbose = true;
/** 
 * Default operations on small dataset
 */
define("DEFAULTDATASET", "small");
$dataset = DEFAULTDATASET;
/**
 * 
 */
foreach ($argv as $param) {
  if ($param == $argv[0]) {
    /*
     * skip $argv[0] which is the script calling string
     * i.e. $ php php/src/Day1.php
     * [DEBUG] skipping arg: php/src/Day1.ph
     */
    // 
    if ($debug) {
      logger("skipping first arg: {$param}");
    }
  } else {
    switch ($param) {
      case '-q':
      case 'q':
      case 'quiet':
        if ($debug) {
          logger("quiet requested");
        }
        $verbose = false;
        break;
      case 'everything':
      case 'full':
        if ($dataset == DEFAULTDATASET) {
          logger("full input.txt");
          $dataset = "";
        } else {
          logger("[WARNING] Please only provide one dataset, failing to set $param", true);
        }
        break;
      case 'tiny':
        if ($dataset == DEFAULTDATASET) {
          logger("smaller {$param}input.txt");
          $dataset = "{$param}";
        } else {
          logger("[WARNING] Please only provide one dataset, failing to set $param", true);
        }
        break;
      default:
        logger("unrecognized arg: '{$param}'", $verbose);
    }
    break;
  }
}

function partOne(bool $usePartTwo = false)
{
  if ($usePartTwo) {
    return partTwo(1);
  } else {
    global $dataset, $debug, $verbose;
    // https://adventofcode.com/2022/day/1
    // input list blocks separated by blank line (or eof)
    // Part One - Sum each block, return Max sum
    $inputObject = Utils::getinput(1, $dataset);
    if ($inputObject == false) {
      logger("FAIL", true);
    }
    $biggestElf = 0;
    $currentElf = 0;
    foreach ($inputObject as $k => $line) {
      // if ($debug || $verbose) {
      //   var_dump($k);
      //   var_dump($line);
      // }
      // logger("Parsing line: " . ($inputObject->key() + 1) . ': ' . $inputObject->current());
      logger("Parsing line: $line");
      logger("Biggest elf: $biggestElf");
      logger("Current Elf:$currentElf");
      # is empty string? compare and set biggest elf from current elf
      if($line == "\n"){
        logger("End of elf: $currentElf");
        if($currentElf >= $biggestElf){
          logger("Setting biggestElf");
          $biggestElf = $currentElf;
        }
        # reset current elf
        $currentElf = 0;
      }
      # otherwise add to currentelf
      $lineValue = intval($line);
      logger("adding $lineValue to $currentElf");
      $currentElf = $currentElf + $lineValue;
      
    }
    return $biggestElf;
  }
}
function partTwo(int $topCount = 3)
{ // https://adventofcode.com/2022/day/1
  // input list blocks separated by blank line (or eof)
  // Part Two - Sum each block, return sum of top 3 blocks
  global $dataset;
  $inputObject = Utils::getinput(1, $dataset);
}

/**
 * @param string $message
 * @param bool $always DEFAULT false - ignores $verbose
 * @return NULL
 */

function logger(string $message, bool $always = false)
{
  global $debug, $verbose;
  if ($debug) {
    Utils::logger("[DEBUG] {$message}");
  } else {
    if ($verbose) {
      Utils::logger($message, $verbose);
    } elseif ($always) {
      Utils::logger("[INFO] {$message}");
    }
  }
}

$input = Utils::getinput(1, $dataset);
echo "day1 $dataset";
var_dump($input);
echo "\n";
logger ("Biggest elf: " . partOne(),true);
