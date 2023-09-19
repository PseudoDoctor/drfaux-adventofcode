<?php
namespace Onine;

require __DIR__ . '/../vendor/autoload.php';
use Onine\Utils;

// https://adventofcode.com/2022/day/1
// input list blocks separated by blank line (or eof)
// Part One - Sum each block, return Max sum
// Part Two - Sum each block, return sum of top 3 blocks

// Arg dump for debugging
var_dump($argv);
if (strpos($argv[1], ' ') !== false) {
  $argw = explode(" ", $argv[1]);
  array_unshift($argw, $argv[2]);
  $argv = $argw;
}
var_dump($argv);

// override verbose
$debug = true;
// Default operations for these tasks is to be noisy and work on the small dataset
$verbose = true;
define("DEFAULTDATASET", "small");
$dataset = "small";
// Check for any argv with the string q or quiet
foreach ($argv as $param) {
  if ($param == $argv[0]) {
    if ($debug) {
      Utils::logger("[DEBUG] skipping arg: ${param}");
    }
  } else {
    switch ($param) {
      case '-q':
      case 'q':
      case 'quiet':
        if ($debug) {
          Utils::logger("[DEBUG] quiet requested");
        }
        $verbose = false;
        break;
      case 'everything':
      case 'full':
        if ($dataset == DEFAULTDATASET) {
          if ($debug) {
            Utils::logger("[DEBUG] full input.txt");
          }
          $dataset = "";
        } else {
          Utils::logger("[WARNING] dataset previous set, failing to set $param");
        }
        break;
      case 'tiny':
        if ($dataset == DEFAULTDATASET) {
          if ($debug) {
            Utils::logger("[DEBUG] smaller ${param}input.txt");
          }
          $dataset = "${param}";
        } else {
          Utils::logger("[WARNING] dataset previous set, failing to set $param");
        }
        break;
      default:
        if ($debug) {
          Utils::logger("[DEBUG] unrecognized arg: '${param}'");
        }
        # code...
        break;
    }
  }
}
if ($debug) {
  if ($dataset == "small") {
    Utils::logger("[DEBUG] smaller smallinput.txt");
  }
}


$input = Utils::getinput(1,$dataset);
echo $input;
?>