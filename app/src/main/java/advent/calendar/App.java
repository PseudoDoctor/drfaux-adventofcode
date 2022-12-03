package advent.calendar;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.Map;
import java.util.stream.Collectors;
import advent.calendar.day1.Day1Manual;

public class App {

    public static final String DAY1_INPUT = "day1/input.omcsettings.txt";

    public String getGreeting() {
        return "Hello World!";
    }

    public BufferedReader getResourceAsBufferedReader(String path) {
        InputStream is = getClass().getClassLoader().getResourceAsStream(path);
        InputStreamReader isr = new InputStreamReader(is);
        BufferedReader br = new BufferedReader(isr);
        return br;
    }

    public void day1(){
        try (BufferedReader br = new App().getResourceAsBufferedReader(DAY1_INPUT)){
            ArrayList<Elf> elves = new Elf().populateElves(br);
            // Day 1 part 1, max elf
            int max = new Elf().maxCalories(elves);
            System.out.println("The most calories carried by a single elf is "+max);
            // Day 1 part 2, top 3 elf total
            Map<Integer,Elf> elfMap = new Elf().sortedElves(elves);
            int elfCount = elfMap.size();
            Map<Integer,Elf> top3ElfMap = elfMap.entrySet().stream()
                .skip(elfCount-3)
                .collect(Collectors.toMap(
                    Map.Entry::getKey
                   , Map.Entry::getValue
                    ));
            top3ElfMap.forEach((key,val)->{
                    out("Top Elf #"+val.getOriginalIndex()+" had "+val.getTotalCalories()+" calories");
                });
            int topElfTotal = new Elf().grandTotal(top3ElfMap);
            System.out.println("Total for top 3 elfs "+topElfTotal);
            
        } catch (IOException e) {
            // TODO Auto-generated catch block
            e.printStackTrace();
        }

    }
    public void day2() {

    }

    public static void main(String[] args) throws IOException {
        System.out.println(new App().getGreeting());
        new App().day1();
        Day1Manual.day1Part1();
        Day1Manual.day1Part2();
        // new App().day2();
        
    }

    private void out(String s){
        // System.out.println(s);
    }
    
}
