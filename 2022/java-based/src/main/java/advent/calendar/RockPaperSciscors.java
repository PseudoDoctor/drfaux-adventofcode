package advent.calendar;

public class RockPaperSciscors {

}
enum RPS {
    ROCK("A","X","1"),
    PAPER("B","Y","2"),
    SCISCORS("C","Z","3");

    private final String opponent;
    private final String response;
    private final String score;

    RPS(String opponent, String response,String score){
        this.opponent = opponent;
        this.response = response;
        this.score = score;
    }

    public String getOpponent() {
        return opponent;
    }
    public String getResponse() {
        return response;
    }
    public String getScore() {
        return score;
    }

}
class Round {

}
