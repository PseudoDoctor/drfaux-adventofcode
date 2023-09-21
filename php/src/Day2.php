<?php

namespace Onine;

require __DIR__ . '/../vendor/autoload.php';

use Onine\Utils;
use SplFileObject;

use function Onine\partOne as OninePartOne;

/* PART 1
 * https://adventofcode.com/2022/day/2
 * Rock Paper Scissors
 * R beats S
 * S beats P
 * P beats R
 * Draw possible
 * 
 * Score
 * 1 Rock
 * 2 Paper
 * 3 Scissors
 * 0 Lost
 * 3 Draw
 * 6 Won
 * 
 * Input
 * Opponent
 * A for Rock
 * B for Paper
 * C for Scissors
 * Response
 * X for Rock
 * Y for Paper
 * Z for Scissors
 * 
 * i.e.
 * A Y - Opponent is going to play A(Rock), you should play Y(Paper). You win the round and get 2 points for Paper and 6 points for winning, total 8 points.
 * B X - B Paper, play X Rock. 1 point for Rock, 0 point for Loss, total 1.
 * C Z - C Scissors, play Z Scissors. 3 points for Scissors, 3 points for Draw, total 6 points.
 * 
 * 8+1+6=15
 * 
 * return 15
 */

$dataset = "small";
class Day2
{
    public static function partOne(SplFileObject $in, bool $verbose = false): int
    {
        $total = 0;
        foreach ($in as $k => $line) {

            $trimmed = trim($line);
            if ($trimmed == "") {
            } else {
                Utils::logger("Processing '$trimmed'");
                $score = Rps::score(substr($trimmed, 0, 1), substr($trimmed, 2, 1));
                Utils::logger("Score: " . $score);
                $total = $total + $score;
            }
        }
        Utils::logger("Total: $total");
        return $total;
    }
    public static function partTwo(SplFileObject $in, bool $verbose = false): int
    {
        $total = 0;
        foreach ($in as $k => $line) {

            $trimmed = trim($line);
            if ($trimmed == "") {
            } else {
                Utils::logger("Processing '$trimmed'");
            }
        }
        return 0;
    }
}



class Rps
{
    /**
     * @param string $left A,B,C
     * @param string $right X,Y,Z
     * @return int total score
     */
    public static function score(string $left, string $right): int
    {
        $score = array(
            "WIN" => 6,
            "LOSE" => 0,
            "DRAW" => 3,
            "Rock" => 1,
            "Paper" => 2,
            "Scissors" => 3
        );
        $play = array(
            "A" => "Rock",
            "B" => "Paper",
            "C" => "Scissors",
            "X" => "Rock",
            "Y" => "Paper",
            "Z" => "Scissors",
        );
        $actualLeft = $play[$left];
        $actualRight = $play[$right];
        Utils::logger("Scoring '$actualLeft' vs '$actualRight'");
        if ($actualLeft === $actualRight) {
            Utils::logger("DRAW");
            return $score[$actualRight] + $score["DRAW"];
        }
        if ($actualLeft === "Rock") {
            if ($actualRight === "Paper") {
                Utils::logger("WIN");
                return $score[$actualRight] + $score["WIN"];
            } else {
                Utils::logger("LOSE");
                return $score[$actualRight] + $score["LOSE"];
            }
        }
        if ($actualLeft === "Paper") {
            if ($actualRight === "Scissors") {
                Utils::logger("WIN");
                return $score[$actualRight] + $score["WIN"];
            } else {
                Utils::logger("LOSE");
                return $score[$actualRight] + $score["LOSE"];
            }
        }
        if ($actualLeft === "Scissors") {
            if ($actualRight === "Rock") {
                Utils::logger("WIN");
                return $score[$actualRight] + $score["WIN"];
            } else {
                Utils::logger("LOSE");
                return $score[$actualRight] + $score["LOSE"];
            }
        }
        return 0;
    }
}


$inputObject = Utils::getinput(2, $dataset);
Day2::partOne($inputObject);
