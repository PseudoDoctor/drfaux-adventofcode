<?php

$inputFile = "input.txt";
$smallInputFile = "smallinput.txt";
$mediumInputFile = "mediuminput.txt";

$handle = fopen("$smallInputFile", "r");
//$handle = fopen("$mediumInputFile", "r");
//$handle = fopen("$inputFile", "r");

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
         * If symbol, add to list of symbols.
         *   - Gears will be filtered later.
         * If number, add to list of numbers with each digits location.
         *   - Find symbols near each digit.
         */
        if (in_array($char, $validDigits)) {
            // This is a digit, add it to $tmpNumber
            $tmpNumber[] = ["char"=>$char, "lineIdx"=>$lineIdx, "charIdx"=>$charIdx];
        } else {
            // process number
            if (!$tmpNumber == array()) {
                $str = "";
                foreach ($tmpNumber as $d) {
                    $str = $str . $d["char"];
                }
                $int = intval($str);
                echo $int . "\n";
                $partNumbers[] = ["int"=>$int, "digits"=>$tmpNumber];
                // clear tmp Number array
                $tmpNumber = array();
            }
            // symbols
            if (!in_array($char, $notSymbols)) {
                $allSymbols[] = ["char"=>$char, "lineIdx"=>$lineIdx, "charIdx"=>$charIdx];
            }
        }
    }
}
print_r($allSymbols);
