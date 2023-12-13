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
//print_r($notSymbols);
foreach ($allLines as $lineIdx => $lineArr) {
    foreach ($lineArr as $charIdx => $char) {
        if ($char == "*"){
            echo "Adding Gear at [$lineIdx][$charIdx]\n";
            $gear = [$lineIdx,$charIdx];
            $allGears[] = $gear;
        }
        // If is digit, add to $thisNumberArr
        if (in_array($char,$validDigits)) {
            $thisNumberArr[$charIdx] = $char;
        } else {
            // Otherwise process $thisNumberArr
//            echo "\nProcessing Number:\n";
//            print_r($thisNumberArr);
            $isNotPartNumber = true;
            foreach ($thisNumberArr as $digitIdx => $digit) {
//                echo "\nProcessing digit at $lineIdx,$digitIdx which is '$digit'";

                $range = range(-1, 1, 1);
                foreach ($range as $vert) {
                    $lineLookup = $lineIdx + $vert;
                    if (in_array($lineLookup, range($lineFirst, $lineLast))) {
                        foreach ($range as $hori) {
                            $charLookup = $digitIdx + $hori;
                            if (in_array($charLookup, range($lineStart, $lineEnd))) {
                                // Process char lookup at $allLines[$lineLookup][$charLookup]
                                $lookedupChar = $allLines[$lineLookup][$charLookup];
//                                echo "\nLooked up Char at $lineLookup,$charLookup is '$lookedupChar'";
                                // Is a symbol (or is not a number nor a dot)
                                if (!in_array($lookedupChar, $notSymbols)) {
//                                    echo "SYMBOL";
                                    $isNotPartNumber = false;
                                }

                            }
                        }
                    }
                }
            }
            // Add if not empty and is missing.
            $emptyArr = array();
            if (!$thisNumberArr == $emptyArr) {
                $thisString = "";
                foreach ($thisNumberArr as $digit) {
                    $thisString = $thisString . "$digit";
                }
                $thisNumber = intval($thisString);
                if (! $isNotPartNumber) {
                    $partNumbers[] = $thisNumber;
                }
            }
            // reset

            $thisNumberArr = array();
        }
    }
}
// We now have an array of possible gear locations.
// If we assume no part number can be more than 3 digits, then each gear can only have digits in the three lines nearest
//  And the places 3 left and 3 right, meaning a 7x3 grid.

$width=range(-3,3);
$height=range(-1,1);
foreach ($allGears as $gear) {
    $localChars = array();
    echo "Gear at {$gear[0]},{$gear[1]}\n";
    foreach ($height as $l){
        foreach ($width as $c){
            $localChars[$gear[0]-$l][$gear[1]-$c] = $allLines[$gear[0]-$l][$gear[1]-$c];
            echo $allLines[$gear[0]-$l][$gear[1]-$c];
        }
        echo "\n";
    }
    print_r($localChars);
    // First look at lines, since part numbers can only be in a line.
    $localDigits = array();
    $localParts = array();
    foreach ($height as $lineOffset){
        // Then look at the immediate digits
        foreach($height as $charOffset){
            if(in_array($allLines[$gear[0]+$lineOffset][$gear[1]+$charOffset],$validDigits)){
                echo "Found digit near gear: {$allLines[$gear[0]+$lineOffset][$gear[1]+$charOffset]}\n";
                $localDigits[] = [$gear[0]+$lineOffset,$gear[1]+$charOffset];
            }
        }
        foreach ($localDigits as $digit){
            foreach ($width as $offset){
                // Digit has x,y
            }
        }
    }
    // Only contiguous strings of digits that include values in $localDigits can be considered.

}

//print_r($partNumbers);
$sum = 0;
foreach ($partNumbers as $num){
    $sum += $num;
}
//sort($partNumbers);
//print_r($partNumbers);
//echo "\nPart Number Sum: $sum";