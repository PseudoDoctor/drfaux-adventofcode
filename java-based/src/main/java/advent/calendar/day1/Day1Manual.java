package advent.calendar.day1;

import java.io.ByteArrayOutputStream;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;
import advent.calendar.App;

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

    static int[] sort(int[] a) {
        int n = a.length;
        for (int i = 0; i < n - 1; i++)
            for (int j = 0; j < n - 1; j++)
                if (a[j] > a[j + 1]) {
                    int t = a[j];
                    a[j] = a[j + 1];
                    a[j + 1] = t;
                }
        return a;

    }
    
    static byte[] sort(byte[] a) {
        int n = a.length;
        for (int i = 0; i < n - 1; i++)
            for (int j = 0; j < n - 1; j++)
                if (a[j] > a[j + 1]) {
                    byte t = a[j];
                    a[j] = a[j + 1];
                    a[j + 1] = t;
                }
        return a;
    }

    public static void day1Part1() throws IOException {
        try (InputStream is = new FileInputStream(App.ROOT_DIR + App.DAY1_INPUT)) {
            byte[] buffer = new byte[0];
            boolean wasDigit = false;
            int max = 0;
            int elf = 0;
            int b = 0;
            // InputStream.read() is one byte at a time represented as an int
            while (true) {
                b = is.read();
                if (b > -1) {
                    // not end of file, do something
                    if (b == 10) {
                        if (wasDigit == true) {
                            // end of int, take buffer and add too elf
                            int k = Day1Manual.dump(buffer);
                            elf += k;
                        } else {
                            // end of elf, compare to max
                            if (elf > max) {
                                max = elf;
                            }
                            // reset elf
                            elf = 0;
                        }
                        // clear buffer regardless of elf or max
                        buffer = new byte[0];
                        wasDigit = false;
                    } else if (b >= 48 && b <= 57) {
                        wasDigit = true;
                        // add digit to buffer
                        buffer = Day1Manual.add(buffer, b);
                    }
                } else {
                    // finalize last elf
                    int k = Day1Manual.dump(buffer);
                    elf += k;
                    if (elf > max) {
                        max = elf;
                    }
                    break;
                }
            }
            System.out.println("Max: " + max);
        }
    }

    public static void day1Part2() throws IOException {
        try (InputStream is = new FileInputStream(App.ROOT_DIR + App.DAY1_INPUT)) {
            byte[] buffer = new byte[0];
            int[] topThree = {0, 0, 0};
            boolean wasDigit = false;
            int max = 0;
            int elf = 0;
            int b = 0;
            // InputStream.read() is one byte at a time represented as an int
            while (true) {
                b = is.read();
                if (b > -1) {
                    // not end of file, do something
                    if (b == 10) {
                        if (wasDigit == true) {
                            // end of int, take buffer and add too elf
                            int k = Day1Manual.dump(buffer);
                            elf += k;
                        } else {
                            for (int j = 0; j < 3; j++) {
                                if (elf > topThree[j]) {
                                    topThree[j] = elf;
                                    break;
                                }
                            }
                            topThree = sort(topThree);
                            // reset elf
                            elf = 0;
                        }
                        // clear buffer regardless of elf or max
                        buffer = new byte[0];
                        wasDigit = false;
                    } else if (b >= 48 && b <= 57) {
                        wasDigit = true;
                        // add digit to buffer
                        buffer = Day1Manual.add(buffer, b);
                    }
                } else {
                    // finalize last elf
                    int k = Day1Manual.dump(buffer);
                    elf += k;
                    for (int j = 0; j < 3; j++) {
                        if (elf > topThree[j]) {
                            topThree[j] = elf;
                            break;
                        }
                    }
                    break;
                }
            }
            for (int i : topThree) {
                max += i;
            }
            System.out.println("Top three total: " + max);
        }
    }

}

