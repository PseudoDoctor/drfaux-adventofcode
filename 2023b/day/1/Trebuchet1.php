<?php
//Check for files
$inputFile = "input.txt";
$smallInputFile = "smallinput.txt";
$mediumInputFile = "mediuminput.txt";
//if (file_exists($inputFile)) {
////    "Primary file $inputFile exists.";
//} else {
//    throw new Exception("ERROR Primary file $inputFile does not exist.");
//}
//if (file_exists($smallInputFile)) {
//    echo "Secondary file $smallInputFile exists.";
//} else {
//    echo "ERROR Primary file $smallInputFile does not exist.";
//}
// PART ONE
$finalNumbers = array();
//$handle = fopen("$smallInputFile", "r");
$handle = fopen("$inputFile", "r");
if ($handle) {
    // start
    $numbers = array();
    while (($line = fgets($handle)) !== false) {
        echo "Looking for numbers $line"; // debug
        // process the line read.
        $arr1 = str_split($line);
        print_r($arr1); // debug
        // loop through chars/array
        foreach ($arr1 as $index => $item) {
            echo "Checking arr1[$index]=>'$item'\n"; // debug
            if(is_numeric($item)){
                // is a number, add $numbers
                echo "Adding $item to numbers \n"; // debug
               $numbers[] = $item;
            }
        }
        echo "Current numbers:"; // debug
        print_r($numbers); // debug
        echo "List of numbers: "; // debug
        foreach ($numbers as $num) { // debug
            echo $num . " "; // debug
        } // debug
        echo ".\n"; // debug

        // grab first and last in like... 2 lines of code
        $first = $numbers[array_key_first($numbers)];
        $last = $numbers[array_key_last($numbers)];
        echo "f $first l $last\n"; // debug
        $finalNumbers[] = $first . $last;


        // -----------
        // done - reset
        $numbers = array();
    }
    // end of file
    fclose($handle);
} else {
    throw new Exception("Can't open $smallInputFile ");
}
// Process final numbers
$sum = 0;
foreach ($finalNumbers as $finalNumber) {
    $sum += $finalNumber;
}
print_r($finalNumbers);
echo "Total: ". $sum;
