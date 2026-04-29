<?php

$inputFile = "input.txt";
$smallInputFile = "smallinput.txt";
$mediumInputFile = "mediuminput.txt";

//$handle = fopen("$smallInputFile", "r");
//$handle = fopen("$mediumInputFile", "r");
$handle = fopen("$inputFile", "r");
$allLines = array();
print_r($handle);
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
echo "Line stats first $lineFirst last $lineLast start $lineStart end $lineEnd\n";//debug

/*
 * 467..114.. <- 467 is diagonal to "*" on the next row. 114 is not connected to any symbol
 * ...*......
 * ..35..633.
 * ......#...
 * 617*...... <- 617 is directly next to "*" on the same row.
 * .....+.58. <- 58 is not connected to any symbol
 * ..592..... <- 592 is diagonal to "+" on previous line
 * ......755. <- diagonal
 * ...$.*....
 * .664.598.. <- 664 and 498 are directly below their symbols
 */
$partNumbers = array();
$thisNumberArr = array();
$validDigits = range(0, 9);
$notSymbols = $validDigits;
$notSymbols[] = ".";
print_r($notSymbols);
foreach ($allLines as $lineIdx => $lineArr) {
    foreach ($lineArr as $charIdx => $char) {
        // If is digit, add to $thisNumberArr
        if (in_array($char,$validDigits)) {
            $thisNumberArr[$charIdx] = $char;
        } else {
            // Otherwise process $thisNumberArr
            echo "\nProcessing Number:\n";
            print_r($thisNumberArr);
            $isNotPartNumber = true;
            foreach ($thisNumberArr as $digitIdx => $digit) {
                echo "\nProcessing digit at $lineIdx,$digitIdx which is '$digit'";

                $range = range(-1, 1, 1);
                foreach ($range as $vert) {
                    $lineLookup = $lineIdx + $vert;
                    if (in_array($lineLookup, range($lineFirst, $lineLast))) {
                        foreach ($range as $hori) {
                            $charLookup = $digitIdx + $hori;
                            if (in_array($charLookup, range($lineStart, $lineEnd))) {
                                // Process char lookup at $allLines[$lineLookup][$charLookup]
                                $lookedupChar = $allLines[$lineLookup][$charLookup];
                                echo "\nLooked up Char at $lineLookup,$charLookup is '$lookedupChar'";
                                // Is a symbol (or is not a number nor a dot)
                                if (!in_array($lookedupChar, $notSymbols)) {
                                    echo "SYMBOL";
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
//print_r($partNumbers);
$sum = 0;
foreach ($partNumbers as $num){
    $sum += $num;
}
echo "Part Number Sum: $sum";