<?php

$inputFile = "input.txt";
$smallInputFile = "smallinput.txt";
$mediumInputFile = "mediuminput.txt";

//$handle = fopen("$smallInputFile", "r");
//$handle = fopen("$mediumInputFile", "r");
$handle = fopen("$inputFile", "r");
$gamesPowers = array();
if ($handle) {
    // start file

    while (($line = fgets($handle)) !== false) {
        // process line
        print_r($line);//debug
        $gameID = preg_match("/Game (\d*):/", $line, $gameIDMatch);
        $gameIsValid = true;
        echo "\nGame ID {$gameIDMatch[1]} \n";//debug

        $pullRegex = preg_match_all("/(((\d+) (green|red|blue))([,;\n]|))/", $line, $pullMatches);
//        print_r($pullMatches);//debug
        /*
         * $pullMatches[1] contains full pull string such as "6 red,"
         * $pullMatches[2] contains pull itself, "6 red"
         * $pullMatches[3] contains the cube count, "6"
         * $pullMatches[4] contains the color "red"
         * $pullMatches[5] contains the terminator "," (or ";" or "\n") (or empty string for the final game)
         */
        /*
         * Loop through and put pulls in temporary array
         * If [5] is a semicolon, check $maxCubes, reset temporary array
         * If success, add game ID to $validGames
         */
        $gamesMaxCubes = ["red" => 0, "green" => 0, "blue" => 0];
        $thisCubes = ["red" => 0, "green" => 0, "blue" => 0];
        foreach ($pullMatches[5] as $idx => $value) {
            $thisCubes[$pullMatches[4][$idx]] += intval($pullMatches[3][$idx]);
//            print_r($thisCubes);//debug
            if (";" == $value or "\n" == $value or "" == $value) {
                echo "Check last few pulls and find max of previous pulls.";//debug

                foreach ($gamesMaxCubes as $color => $maxCube) {
                    if ($thisCubes[$color] >= $maxCube) {
                        $gamesMaxCubes[$color] = $thisCubes[$color];
                    }
                }
                // reset
                $thisCubes = ["red" => 0, "green" => 0, "blue" => 0];
            }
        }

        print_r($gamesMaxCubes);
        $gamePower = 1;
        foreach ($gamesMaxCubes as $val) {
            $gamePower *= $val;
        }
        $gamesPowers[] = $gamePower;

    }
    // end of file handle
    fclose($handle);
}
// Add up
$sum = 0;
print_r($gamesPowers);//debug
foreach ($gamesPowers as $game) {
    $sum += $game;
}
echo "Sum of powers: $sum";
