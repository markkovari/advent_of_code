package mark.kovari.aoc;

import utils.CounterMap;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.List;

public class Twelfth {

    private static final int FLOOR_COUNT = 4;
    private static final int TARGET_FLOOR = FLOOR_COUNT - 1;

    static int visitedNodeCount = 0;

    private static long getValue(CounterMap<String> mem, String arg) {
        return isRegister(arg) ? mem.getValue(arg) : Long.parseLong(arg);
    }

    private static boolean isRegister(String s) {
        return s.length() == 1 && Character.isLetter(s.charAt(0));
    }

    private static long solve(List<String> code, CounterMap<String> mem) {
        for (int i = 0; i < code.size(); ) {
            var p = code.get(i).split(" ");
            switch (p[0]) {
                case "cpy" -> mem.put(p[2], getValue(mem, p[1]));
                case "inc" -> mem.inc(p[1]);
                case "dec" -> mem.dec(p[1]);
                case "jnz" -> {
                    long value = getValue(mem, p[1]);
                    if (value != 0) {
                        i += (int) getValue(mem, p[2]);
                        continue;
                    }
                }
            }
            i++;
        }
        return mem.getValue("a");
    }

    public void solve() {

        try {
            List<String> lines = Arrays.stream(Files.readString(Paths.get("./inputs/12/prod.data")).split(System.lineSeparator())).toList();

            var mem1 = new CounterMap<String>();

            var mem2 = new CounterMap<String>();
            mem2.put("c", 1L);
            System.out.println("Part 1: " + solve(lines, mem1));
            System.out.println("Part 2: " + solve(lines, mem2));

        } catch (IOException e) {
            e.printStackTrace();
        }

    }
}
