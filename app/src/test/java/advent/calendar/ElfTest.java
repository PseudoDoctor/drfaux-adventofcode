package advent.calendar;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.util.ArrayList;

class ElfTest {
    @Test void elfSingleCalorie() {
        String[] calories = {"1000"};
        Elf classUnderTest = new Elf(calories,0);
        int total = classUnderTest.getTotalCalories();
        assertEquals(total,1000,"should have 1000 calories");
    }
    @Test void elfDoubleCalorie() {
        String[] calories = {"1000","2000"};
        Elf classUnderTest = new Elf(calories,0);
        int total = classUnderTest.getTotalCalories();
        assertEquals(total,3000,"should have 3000 calories");
    }
    @Test void elfEmptyCalorie() {
        String[] calories = {"1000","2000","","3000"};
        Elf classUnderTest = new Elf(calories,1);
        int total = classUnderTest.getTotalCalories();
        assertEquals(total,6000,"should have 6000 calories");
    }
    @Test void topThreeSmallSample() throws IOException {
        BufferedReader br = getResourceAsBufferedReader("day1/input/smallinput.txt");
        ArrayList<Elf> elves = new Elf().populateElves(br);
        new Elf().sortedElves(elves);
    }
    public BufferedReader getResourceAsBufferedReader(String path) {
        InputStream is = getClass().getClassLoader().getResourceAsStream(path);
        InputStreamReader isr = new InputStreamReader(is);
        BufferedReader br = new BufferedReader(isr);
        return br;
    }
}
