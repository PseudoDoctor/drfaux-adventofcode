package advent.calendar;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.List;

public class App {

    public static final String DAY1_INPUT = "day1/input/input.omcsettings.txt";

    public String getGreeting() {
        return "Hello World!";
    }

    public BufferedReader getResourceAsBufferedReader(String path) {
        InputStream is = getClass().getClassLoader().getResourceAsStream(path);
        InputStreamReader isr = new InputStreamReader(is);
        BufferedReader br = new BufferedReader(isr);
        return br;
    }

    public static void main(String[] args) {
        System.out.println(new App().getGreeting());

        try (BufferedReader br = new App().getResourceAsBufferedReader(DAY1_INPUT)){
            ArrayList<Elf> elves = new Elf().populateElves(br);
            int max = new Elf().maxCalories(elves);
            System.out.println(max);
        } catch (IOException e) {
            // TODO Auto-generated catch block
            e.printStackTrace();
        }
        List<Elf> e;
        
    }
}
