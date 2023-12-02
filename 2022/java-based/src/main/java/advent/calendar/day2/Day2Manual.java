package advent.calendar.day2;

import java.io.IOException;
import java.io.InputStream;

public class Day2Manual {
    public static final String DAY2_INPUT = "day2/input.txt";

    public static void main(String[] args) throws IOException {
        InputStream is = new Day2Manual().getClass().getClassLoader().getResourceAsStream(DAY2_INPUT);
        int z = 0;
        while ((z = is.read()) > -1) {
            System.out.print("int'" + z + "' char'" + (char) z + "'");
            switch (z) {
                case 10:
                    System.out.println(" Newline");
                    break;
                case 32:
                    System.out.println(" Space");
                    break;
                case 65:
                    System.out.println(" A");
                    break;
                case 66:
                    System.out.println(" B");
                    break;
                case 67:
                    System.out.println(" C");
                    break;
                case 88:
                    System.out.println(" X");
                    break;
                case 89:
                    System.out.println(" Y");
                    break;
                case 90:
                    System.out.println(" Z");
                    break;
                default:
                    System.out.println(" Unhandled");
            }
        }
    }
}
