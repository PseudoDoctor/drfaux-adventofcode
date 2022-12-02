package advent.calendar;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class ElfTest {
    @Test void elfSingleCalorie() {
        String[] calories = {"1000"};
        Elf classUnderTest = new Elf(calories);
        int total = classUnderTest.setTotalCalories();
        assertEquals(total,1000,"should have 1000 calories");
    }
    @Test void elfDoubleCalorie() {
        String[] calories = {"1000","2000"};
        Elf classUnderTest = new Elf(calories);
        int total = classUnderTest.setTotalCalories();
        assertEquals(total,3000,"should have 3000 calories");
    }
    @Test void elfEmptyCalorie() {
        String[] calories = {"1000","2000","","3000"};
        Elf classUnderTest = new Elf(calories);
        int total = classUnderTest.setTotalCalories();
        assertEquals(total,6000,"should have 6000 calories");
    }
}
