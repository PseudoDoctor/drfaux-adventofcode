<?php

$inputFile = "input.txt";
$smallInputFile = "smallinput.txt";
$mediumInputFile = "mediuminput.txt";

$handle = fopen("$smallInputFile", "r");
//$handle = fopen("$mediumInputFile", "r");
//$handle = fopen("$inputFile", "r");
$missingNumbers = array();
$allLines=array();
print_r($handle);
if ($handle) {
    while (($line = fgets($handle)) !== false) {
        // dump all lines into memory
        print_r($line);//debug
        $allLines[] = str_split($line);
        fclose($handle);
    }
}
echo "=====";//debug
print_r($allLines);//debug
// Process
// First and Last lines, avoid index out of bounds
$lineFirst = array_key_first($allLines);
$lineLast = array_key_first($allLines);
// Line length
$lineStart = array_key_first($allLines[0]);
$lineEnd = array_key_last($allLines[0]);
$thisNumberArr = array();
/*
 * 467..114.. <- 467 is diagonal to "*" on the next row. 114 is not connected to any symbol
 * ...*......
 * ..35..633.
 * ......#...
 * 617*...... <- 617 is directly next to "*" on the same row
 * .....+.58. <- 58 is not connected to any symbol
 * ..592..... <- 592 is diagonal to "+" on previous line
 * ......755. <- diagonal
 * ...$.*....
 * .664.598.. <- 664 and 498 are directly below their symbols
 */

foreach ($allLines as $lineIdx => $lineArr) {
    foreach ($lineArr as $charIdx => $char){

    }
}
