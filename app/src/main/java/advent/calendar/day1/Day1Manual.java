package advent.calendar.day1;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;

public class Day1Manual {
    static byte[] add(byte[] a, int i) throws IOException {
        ByteArrayOutputStream out = new ByteArrayOutputStream();
        out.write(a);
        out.write((byte) i);
        byte[] arr_combined = out.toByteArray();
        return arr_combined;
    }

    static int dump(byte[] a) {
        int place = 1;
        int total = 0;
        for (int j = a.length - 1; j >= 0; j--) {
            // System.out.print(a[j] + " ");
            if (a[j] != 0) {
                total = total + (place * Integer.valueOf(String.valueOf((char) a[j])));
                place = place * 10;
            } else {
                break;
            }
        }
        return total;
    }

    public static void day1Part1() throws IOException{
        InputStream is = new Day1Manual().getClass().getClassLoader().getResourceAsStream("day1/input.omcsettings.txt");
        byte[] buffer = new byte[0];
        boolean wasDigit = false;
        int max = 0;
        int elf = 0;
        int z = 0;
        // InputStream.read() is one byte at a time represented as an int
        while ((z = is.read()) > -1) {
            if (z == 10) {
                if (wasDigit == true) {
                    // end of int, take buffer and add too elf
                    int k = Day1Manual.dump(buffer);
                    elf += k;
                } else {
                    // end of elf, compare to max
                    if(elf > max){
                        max = elf;
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
        System.out.println("Max: "+max);
    }
    public static void day1Part2() throws IOException {
        InputStream is = new Day1Manual().getClass().getClassLoader().getResourceAsStream("day1/input.omcsettings.txt");
        byte[] buffer = new byte[0];
        int[] topThree = new int[3];
        boolean wasDigit = false;
        int max = 0;
        int elf = 0;
        int z = 0;
        // InputStream.read() is one byte at a time represented as an int
        while ((z = is.read()) > -1) {
            if (z == 10) {
                if (wasDigit == true) {
                    // end of int, take buffer and add too elf
                    int k = Day1Manual.dump(buffer);
                    elf += k;
                } else {
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
            max += i;
        }
        System.out.println("Top three total: "+max);
    }

}

