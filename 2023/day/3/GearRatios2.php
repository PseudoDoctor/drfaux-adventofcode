<?php

$inputFile = "input.txt";
$smallInputFile = "smallinput.txt";
$mediumInputFile = "mediuminput.txt";
$tinyInputFile = "tinyinput.txt";

//$handle = fopen("$smallInputFile", "r");
//$handle = fopen("$mediumInputFile", "r");
$handle = fopen("$inputFile", "r");
//$handle = fopen("$tinyInputFile", "r");

$allLines = array();
//print_r($handle);
if ($handle) {
    while (($line = fgets($handle)) !== false) {
        // dump all lines into memory
//        print_r($line);//debug
        $allLines[] = str_split($line);
    }
    fclose($handle);
}

//echo "\n=====\n";//debug
//print_r($allLines);//debug
// Process
// First and Last lines, avoid index out of bounds
$lineFirst = array_key_first($allLines);
$lineLast = array_key_last($allLines);
// Line length
$lineStart = array_key_first($allLines[$lineLast]);
$lineEnd = array_key_last($allLines[$lineLast]);
//echo "Line stats first $lineFirst last $lineLast start $lineStart end $lineEnd\n";//debug

/*
 * 467..114.. <- 467 is diagonal to "*" on the next row. 114 is not connected to any symbol
 * ...*...... <- "*" has two part numbers adjacent
 * ..35..633. <- 35 is below "*"
 * ......#...
 * 617*...... <- 617 is directly next to "*" on the same row. "*" has one part number adjacent
 * .....+.58. <- 58 is not connected to any symbol
 * ..592..... <- 592 is diagonal to "+" on previous line
 * ......755. <- diagonal
 * ...$.*.... <- "*" has two part number adjacent
 * .664.598.. <- 664 and 498 are directly below their symbols
 */
$partNumbers = array();
$allSymbols = array();
$allSymbolsAlt = array();
$allGears = array();
$actualGearsWithRatio = array();
$thisNumberArr = array();
$validDigits = range(0, 9);
$notSymbols = $validDigits;
$notSymbols[] = ".";
$notSymbols[] = "\n";
//print_r($notSymbols);
foreach ($allLines as $lineIdx => $lineArr) {
    $tmpNumber = array();
    foreach ($lineArr as $charIdx => $char) {
        /*
         * If symbol, add to list of symbols with its location
         * If number, find each digit's location and add to list of numbers
         */
        if (in_array($char, $validDigits)) {
            // This is a digit, add it to $tmpNumber
            $tmpNumber[] = ["char" => $char, "lineIdx" => $lineIdx, "charIdx" => $charIdx];
        } else {
            // process number
            if (!$tmpNumber == array()) {
                $partNumbers[] = $tmpNumber;
                $tmpNumber = array();
            }
            // symbols
            if (!in_array($char, $notSymbols)) {
                $allSymbols[] = ["char" => $char, "lineIdx" => $lineIdx, "charIdx" => $charIdx];
                $allSymbolsAlt[$lineIdx][$charIdx] = "$char";
            } else {
                // This should be a dot.
                if (!$char == ".") {
                    throw new Exception("What is this?>'$char'< at line $lineIdx char $charIdx");
                }
            }
        }
    }
}
/*
 * Process *
 * *******
 * We now have two arrays. One of symbols including gears "*", One of a series of numbers. Each symbol and each digit has their X/Y location.
*/
//print_r($allSymbols);

//print_r($allSymbolsAlt);
$partNumbersHavingGears = array();
$gearsWithPartNumbers = array();
/*
 * Loop through digits, then loop through symbols.
 *  -  If symbol is "near" digit
 *  --   If symbol is Gear, add gear and Number to new $partNumbersHavingGears array
 */
foreach ($partNumbers as $pIdx => $partNumber) {
    foreach ($partNumber as $dIdx => $digit) {
//        echo "DIGIT at " . $digit["lineIdx"] . "," . $digit["charIdx"];
        foreach (range(-1, 1) as $lineOffset) {
            $l = $digit["lineIdx"] + $lineOffset;
            if (array_key_exists($l, $allSymbolsAlt)) {
                foreach (range(-1, 1) as $charOffset) {
                    $c = $digit["charIdx"] + $charOffset;
                    if (array_key_exists($c, $allSymbolsAlt[$l])) {
//                        echo "Printing symbol at $l,$c";
                        $x = $allSymbolsAlt[$l][$c];
//                        echo "'$x'";
                        if ($x == "*") {
                            $partNumbers[$pIdx]["gear"][$l][$c] = $x;
                            $partNumbersHavingGears[$pIdx] = $partNumber;
                            $partNumbersHavingGears[$pIdx]["gear"][$l][$c] = $x;
                            $gearsWithPartNumbers
                            [$l]
                            [$c]
                            [
                                $partNumber[0]
                                    ["lineIdx"]
                            ]
                            [
                                $partNumber[0]
                                    ["charIdx"]
                            ]=$partNumber;
                        }
                    } else {
//                        echo "No symbol at charIdx " . $c;
                    }

//                    echo "\n";

                }
            } else {
//                echo "No symbol on Line " . $l;
            }
//            echo "\n";
        }
    }
}
//print_r($gearsWithPartNumbers);
/*
 * We now have a horrifically nested multidimentional array.
 *   First levels 0 and 1's keys are the Gear's Line/Column.
 *   Next levels 2 and 3's keys are Part Number's starting position
 *   Level 3 is also the Part Number container.
 *   Level 4 is the Digit container (in order)
 *   Level 5 "char" key being a digit of the number
 * Array
 * 0    1    2    3   4   5
( [1] => Array    |   |   |   <-- Gear Line
     ( [3] => Array   |   |   <-- Gear Column
          ( [0] => Arr|y  |   <-- Part Number Start Line
               ( [0] =| Ar|ay <-- Part Number Start Column
                 (   [0] =| Array <-- Part Number first digit container
                      ( [char] => 1 <-- Part Number first digit
                        [lineIdx] => 0
                        [charIdx] => 0
                 )
                     [1] => Array   <-- Part Number second digit
                      ( [char] => 2
                        [lineIdx] => 0
                        [charIdx] => 1
                 )
                     [2] => Array
                      ( [char] => 3
                        [lineIdx] => 0
                        [charIdx] => 2
                 )))
           [1] => Array       <-- Part Number Start Line
               ( [0] => Array <-- Part Number Start Column
                 (   [0] => Array
                      ( [char] => 7
                        [lineIdx] => 1
                        [charIdx] => 0
                 )
                     [1] => Array
                      ( [char] => 8
                        [lineIdx] => 1
                        [charIdx] => 1
                 )
                     [2] => Array
                      ( [char] => 9
                        [lineIdx] => 1
                        [charIdx] => 2
                 )))))
 * 0    1    2    3   4   5
  [2] => Array    |   |   |   <-- Gear Line
     ( [5] => Array   |   |   <-- Gear Column
          ( [1] => Arr|y  |   <-- Part Number Start Line
               ( [6] =| Ar|ay <-- Part Number Start Column
                 (   [0] => Array <-- Part Number first digit container
                      ( [char] => 1 Part Number first digit
                        [lineIdx] => 1
                        [charIdx] => 6
                 )
                     [1] => Array
                      ( [char] => 2
                        [lineIdx] => 1
                        [charIdx] => 7
                 ))))))
 */
/*
 * First pair of foreach is the Gear covering Level's 0 and 1
 * Second pair of foreach is the Part covering Level's 2 and 3
 * Count the number of level 3's for each Gear
 */
$sum = 0;
foreach ($gearsWithPartNumbers as $l0 => $gearLine) {
    foreach ($gearLine as $l1 => $gearChar) {
        $gearRatio = 1;
        $tmpInts = array();
        foreach ($gearChar as $l2 => $partLine){
            foreach ($partLine as $l3 => $partColumn){
                $str = "";
                foreach ($partColumn as $d) {
                    $str = $str . $d["char"];
                }
                $int = intval($str);
//                echo $int . "\n";
                $tmpInts[] = $int;
            }
        }
        if(count($tmpInts)==2){
            foreach ($tmpInts as $int){
                $gearRatio *= $int;
            }
            $sum += $gearRatio;
        } else {
//            echo "Not a gear, just a star\n";
//            print_r($gearChar);
//            echo "\n";
        }
    }
}
echo "Total of all gear ratios: $sum";