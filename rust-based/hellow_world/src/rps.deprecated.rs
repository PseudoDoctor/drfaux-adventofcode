static NEW_LINE: u8 = b'\n';
static SPACE: u8 = b' ';
static A: u8 = b'A';
static B: u8 = b'B';
static C: u8 = b'C';
static X: u8 = b'X';
static Y: u8 = b'Y';
static Z: u8 = b'Z';
// static ROCK: i8 = 1;
// static PAPER: i8 = 2;
// static SCISSORS: i8 = 3;
static LOSE: i8 = 0;
static WIN: i8 = 6;
static DRAW: i8 = 3;
enum FighterType {
    Rock,
    Paper,
    Scissors
}
struct FighterObject {
    fighter:FighterType,
    inferior:FighterType,
    play:u8,
    response:u8,
    score:i8,   
}
// static ROCK:FighterObject = FighterObject {
//     fighter:FighterType::Rock,
//     inferior:FighterType::Scissors,
//     play:A,
//     response:X,
//     score:1
// };
// static PAPER:FighterObject = FighterObject{
//     fighter:FighterType::Paper,
//     inferior:FighterType::Rock,
//     play:B,
//     response:Y,
//     score:2
// };
// static SCISSORS:FighterObject = FighterObject{
//     fighter:FighterType::Scissors,
//     inferior:FighterType::Paper,
//     play:C,
//     response:Z,
//     score:3
// };

let ROCK:FighterObject = FighterObject {
    fighter:FighterType::Rock,
    inferior:FighterType::Scissors,
    play:A,
    response:X,
    score:1
};
let PAPER:FighterObject = FighterObject{
    fighter:FighterType::Paper,
    inferior:FighterType::Rock,
    play:B,
    response:Y,
    score:2
};
let SCISSORS:FighterObject = FighterObject{
    fighter:FighterType::Scissors,
    inferior:FighterType::Paper,
    play:C,
    response:Z,
    score:3
};