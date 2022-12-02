/*
 * For example, suppose the Elves finish writing their items' Calories and end up with the following list:
 * 
 * 1000
 * 2000
 * 3000
 * 
 * 4000
 * 
 * 5000
 * 6000
 * 
 * 7000
 * 8000
 * 9000
 * 
 * 10000
 * This list represents the Calories of the food carried by five Elves:
 * 
 * The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
 * The second Elf is carrying one food item with 4000 Calories.
 * The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
 * The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
 * The fifth Elf is carrying one food item with 10000 Calories.
 * 
 * In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories.
 * In the example above, this is 24000 (carried by the fourth Elf).
 * 
 * Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
 */
package advent.calendar;

import java.io.BufferedReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Collections;
import com.google.common.base.Strings;

public class Elf {
    private ArrayList<String> listOfCalories;
    private int totalCalories;
    private int originalIndex;

    public Elf() {

    }

    public Elf(ArrayList<String> calories) {
        listOfCalories = calories;
        totalCalories = setTotalCalories();
    }

    public Elf(String[] calories) {
        ArrayList<String> caloriess = new ArrayList<String>();
        for (String string : calories) {
            caloriess.add(string);
        }
        listOfCalories = caloriess;
        totalCalories = setTotalCalories();
    }

    private void setOriginalIndex(int i){
        originalIndex = i;
    }

    private int setTotalCalories() {
        int total = 0;
        for (String string : listOfCalories) {
            try {
                int oldtotal = total;
                total += Integer.valueOf(string);
                // System.out.println(oldtotal+" + "+string+" = "+total);
            } catch (Exception e) {
                // TODO: Is this desired behavior? should it fail completely if the string isn't an int? Should we use a list of ints instead verifying each in a constructor first?
                // For day1, this *should* be sufficient
                System.out.println("WARNING, error parsing int from string '" + string + "'. Total may not be accurate.");
                // e.printStackTrace();
            }
        }
        return total;
    }

    public ArrayList<String> getListOfCalories() {
        return listOfCalories;
    }
   
    public int getTotalCalories() {
        return totalCalories;
    }

    public int getOriginalIndex() {
        return originalIndex;
    }

    public ArrayList<Elf> populateElves(BufferedReader bufferedReader) throws IOException {
        ArrayList<Elf> elves = new ArrayList<Elf>();
        ArrayList<String> calories = new ArrayList<String>();
        String line;
        int originalIndex = 0;
        while ((line = bufferedReader.readLine()) != null) {
            if (Strings.isNullOrEmpty(line)) {
                ArrayList<String> c = new ArrayList<String>(calories);
                Elf e = new Elf(c);
                e.setOriginalIndex(originalIndex);
                originalIndex++;
                System.out.println(e.getListOfCalories());
                elves.add(new Elf(new ArrayList<String>(calories)));
                calories.clear();
            } else {
                calories.add(line);
            }
        }
        return elves;
    }

    public int maxCalories(ArrayList<Elf> elves) {
        int max = 0;
        int index = 0;
        int maxIndex = 0;
        for (Elf elf : elves) {
            int elfTotal = elf.setTotalCalories();
            System.out.println("This elf #" + index + " has " + elfTotal + ". Compared to previous max #" + maxIndex + " total " + max);
            if (elfTotal > max) {
                max = elfTotal;
                maxIndex = index;
            }
            index++;
        }
        System.out.println("Elf number " + maxIndex + " had " + max + " calories");
        return max;
    }

    // public ArrayList<Elf> topElves(ArrayList<Elf> elves) {
    //     Collections.sort()
    // }
}
