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
