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
import java.util.Collection;
import java.util.Collections;
import java.util.HashMap;
import java.util.LinkedHashMap;
import java.util.Map;
import java.util.stream.Collectors;

import com.google.common.base.Strings;

public class Elf {
    private ArrayList<String> listOfCalories;
    private int totalCalories;
    private int originalIndex;

    public Elf() {

    }

    public Elf(ArrayList<String> calories,int originalIndex) {
        listOfCalories = calories;
        totalCalories = setTotalCalories();
        this.originalIndex = originalIndex;
    }

    public Elf(String[] calories,int originalIndex) {
        ArrayList<String> caloriess = new ArrayList<String>();
        for (String string : calories) {
            caloriess.add(string);
        }
        listOfCalories = caloriess;
        totalCalories = setTotalCalories();
        this.originalIndex = originalIndex;
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
            System.out.println("Processing: "+line);
            if (Strings.isNullOrEmpty(line)) {
                System.out.println("Line blank, Adding new elf#"+originalIndex+" with "+calories);
                elves.add(new Elf(new ArrayList<String>(calories),originalIndex));
                calories.clear();
                originalIndex++;
            } else {
                System.out.println("Line not blank");
                calories.add(line);
            }
        }
        if(calories.size() > 0){
            System.out.println("Last elf missed, adding elf#"+originalIndex+" with "+calories);
            elves.add(new Elf(new ArrayList<String>(calories),originalIndex));
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
    public Map<Integer, Elf> getElvesAsMap(ArrayList<Elf> elves) {
        Map<Integer, Elf> elfMap = new HashMap<>();
        for (Elf elf : elves) {
            elfMap.put(Integer.valueOf(elf.getTotalCalories()),elf);
        }
        return elfMap;
    }

    public Map<Integer,Elf> topThreeElves(ArrayList<Elf> elves){
        Map<Integer, Elf> elfMap = getElvesAsMap(elves);
        return topThreeElves(elfMap);
    }
    public Map<Integer,Elf> topThreeElves(Map<Integer,Elf> elfMap){
        Map<Integer,Elf> sortedElves = elfMap.entrySet().stream().sorted((e1,e2)->
                Integer.valueOf(e1.getValue().getTotalCalories())
                .compareTo(Integer.valueOf(e2.getValue().getTotalCalories())))
            .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue,
                (e1, e2) -> e1, LinkedHashMap::new));
        
        sortedElves.forEach((key,val)->{
            System.out.println("Elf #"+val.getOriginalIndex()+" had "+val.getTotalCalories()+" calories");
        });
        return sortedElves;
    }
//     Map<String,Person> map = new HashMap<>();
// map.put("g",new Person(5, "EE", 51, Person.SEX.FEMALE, "A"));
// map.put("a",new Person(4, "DD", 25, Person.SEX.MALE, "D"));
// map.put("e",new Person(3, "CC", 44, Person.SEX.FEMALE,"B"));

// Map<String,Person> sortedNewMap = map.entrySet().stream().sorted((e1,e2)->
//         e1.getValue().getLocation().compareTo(e2.getValue().getLocation()))
//         .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue,
//                 (e1, e2) -> e1, LinkedHashMap::new));
// sortedNewMap.forEach((key,val)->{
//     System.out.println(key+ " = "+ val.toString());

    public int grandTotal(ArrayList<Elf> elves){
        int total = 0;
        for (Elf elf : elves) {
            total += elf.getTotalCalories();
        }
        return total;
    }
    // public ArrayList<Elf> topElves(ArrayList<Elf> elves) {
    //     Collections.sort()
    // }
}
