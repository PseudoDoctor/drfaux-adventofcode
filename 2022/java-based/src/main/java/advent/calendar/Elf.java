package advent.calendar;

import java.io.BufferedReader;
import java.io.IOException;
import java.util.ArrayList;
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

    private int setTotalCalories() {
        int total = 0;
        for (String string : listOfCalories) {
            try {
                int oldtotal = total;
                total += Integer.valueOf(string);
                out(oldtotal+" + "+string+" = "+total);
            } catch (Exception e) {
                // TODO: Is this desired behavior? should it fail completely if the string isn't an int? Should we use a list of ints instead verifying each in a constructor first?
                // For day1, this *should* be sufficient
                out("WARNING, error parsing int from string '" + string + "'. Total may not be accurate.");
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
            out("Processing: "+line);
            if (Strings.isNullOrEmpty(line)) {
                out("Line blank, Adding new elf#"+originalIndex+" with "+calories);
                elves.add(new Elf(new ArrayList<String>(calories),originalIndex));
                calories.clear();
                originalIndex++;
            } else {
                out("Line not blank");
                calories.add(line);
            }
        }
        if(calories.size() > 0){
            out("Last elf missed, adding elf#"+originalIndex+" with "+calories);
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
            out("This elf #" + index + " has " + elfTotal + ". Compared to previous max #" + maxIndex + " total " + max);
            if (elfTotal > max) {
                max = elfTotal;
                maxIndex = index;
            }
            index++;
        }
        out("Elf number " + maxIndex + " had " + max + " calories");
        return max;
    }
    public Map<Integer, Elf> getElvesAsMap(ArrayList<Elf> elves) {
        Map<Integer, Elf> elfMap = new HashMap<>();
        for (Elf elf : elves) {
            elfMap.put(Integer.valueOf(elf.getTotalCalories()),elf);
        }
        return elfMap;
    }

    public Map<Integer,Elf> sortedElves(ArrayList<Elf> elves){
        Map<Integer, Elf> elfMap = getElvesAsMap(elves);
        return sortedElves(elfMap);
    }
    public Map<Integer,Elf> sortedElves(Map<Integer,Elf> elfMap){
        Map<Integer,Elf> sortedElves = elfMap.entrySet().stream().sorted((e1,e2)->
                Integer.valueOf(e1.getValue().getTotalCalories())
                .compareTo(Integer.valueOf(e2.getValue().getTotalCalories())))
            .collect(Collectors.toMap(
                Map.Entry::getKey
                , Map.Entry::getValue,
                (e1, e2) -> e1
                , LinkedHashMap::new));
        
        sortedElves.forEach((key,val)->{
            out("Elf #"+val.getOriginalIndex()+" had "+val.getTotalCalories()+" calories");
        });
        return sortedElves;
    }
    public int grandTotal(ArrayList<Elf> elves){
        int total = 0;
        for (Elf elf : elves) {
            total += elf.getTotalCalories();
        }
        return total;
    }
    public int grandTotal(Map<Integer,Elf> elfMap){
        int total = 0;
        total = elfMap.entrySet().stream().collect(Collectors.summingInt(v->v.getValue().getTotalCalories()));
        return total;
    }
 
    private void out(String s){
        // System.out.println(s);
    }
}
