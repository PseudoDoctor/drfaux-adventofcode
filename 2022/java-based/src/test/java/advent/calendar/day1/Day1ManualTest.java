package advent.calendar.day1;

import java.io.IOException;
import java.io.InputStream;

public class Day1ManualTest {
    
    public static void main(String[] args) throws IOException {
        InputStream is = new Day1Manual().getClass().getClassLoader().getResourceAsStream("day1/smallinput.txt");
        byte[] buffer = new byte[0];
        int[] topThree = new int[3];
        boolean wasDigit = false;
        int max = 0;
        int elf = 0;
        int z = 0;
        // InputStream.read() is one byte at a time represented as an int
        while ((z = is.read()) > -1) {
            System.out.println("int "+z+" char "+(char)z+" String '"+String.valueOf((char)z)+"'");
            if (z == 10) {
                if (wasDigit == true) {
                    // end of int, take buffer and add too elf
                    int k = Day1Manual.dump(buffer);
                    elf += k;
                } else {
                    System.out.println("Elf "+elf);
                    // end of elf, compare to max
                    for (int j=0;j<3;j++) {
                        if(elf > topThree[j]){
                            topThree[j] = elf;
                            break;
                        } 
                    }
                    // reset elf
                    elf = 0;
                }
                // clear buffer regardless of elf or max
                buffer = new byte[0];
                wasDigit = false;
            } else if (z >= 48 && z <= 57) {
                wasDigit = true;
                // add digit to buffer
                buffer = Day1Manual.add(buffer, z);
            }
        }
        for (int i : topThree) {
            System.out.println(i);
            max += i;
        }
        System.out.println("Top three total: "+max);
    }
}
