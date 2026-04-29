<?php
$inputFile = "input.txt";
$smallInputFile = "smallinput.txt";
$mediumInputFile = "mediuminput.txt";

$finalNumbers = array();
//$handle = fopen("$smallInputFile", "r");
//$handle = fopen("$mediumInputFile", "r");
$handle = fopen("$inputFile", "r");
if ($handle) {
    // start
    $numbers = array();
    while (($line = fgets($handle)) !== false) {
        echo "Looking for numbers $line"; // debug
        // process the line read.
        $arr1 = str_split($line);
//        print_r($arr1); // debug
        // loop through chars/array
        $reSingleDigitNumbers = "/([1-9]|one|two|three|four|five|six|seven|eight|nine)/";
        preg_match_all($reSingleDigitNumbers, $line, $matches);
//        print_r($matches);// debug
//        foreach ($arr1 as $index => $item) {
//            echo "Checking arr1[$index]=>'$item'\n"; // debug
//            if(is_numeric($item)){
//                // is a number, add $numbers
//                echo "Adding $item to numbers \n"; // debug
//               $numbers[] = $item;
//            }
//        }
//        echo "Current numbers:"; // debug
//        print_r($numbers); // debug
//        echo "List of numbers: "; // debug
//        foreach ($numbers as $num) { // debug
//            echo $num . " "; // debug
//        } // debug
//        echo ".\n"; // debug
//
        // grab first and last in like... 2 lines of code
        $lookupNums = [
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9
        ];
        $tmpNums = $matches[0];
        print_r($tmpNums); //debug
        $first = $tmpNums[array_key_first($tmpNums)];
        if (!is_numeric($first)) {
            $first = $lookupNums[$first];
        }
        $last = $tmpNums[array_key_last($tmpNums)];
        if (!is_numeric($last)) {
            $last = $lookupNums[$last];
        }
        echo "f $first l $last\n"; // debug
        $finalNumbers[] = $first . $last;

        // -----------
        // done with line - reset numbers array
        $numbers = array();
    }
    // end of file handle
    fclose($handle);
}
// Process final numbers
$sum = 0;
foreach ($finalNumbers as $finalNumber) {
    $sum += $finalNumber;
}
print_r($finalNumbers);//debug
echo "Total: ". $sum;
