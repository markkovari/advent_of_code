package mark.kovari.aoc;

import utils.CounterMap;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class TwentyThird {

    private static final String MULTIPLICATION_ORIG =
            "cpy 0 a\ncpy b c\ninc a\ndec c\njnz c -2\ndec d\njnz d -5";

    private static final String MULTIPLICATION_OPTIMIZED =
            "cpy b a\nmul a d\ncpy 0 c\ncpy 0 d\ncpy 0 d\ncpy 0 d\ncpy 0 d";

    private static long solve(List<String> code, long initialValue) {
        var mem = new CounterMap<String>();
        mem.put("a", initialValue);

        boolean[] toggled = new boolean[code.size()];
        for (int i = 0; i < code.size(); ) {
            var p = code.get(i).split(" ");
            var cmd = p[0];

            if (toggled[i]) {
                cmd = switch (cmd) {
                    case "inc" -> "dec";
                    case "dec", "tgl" -> "inc";
                    case "jnz" -> "cpy";
                    case "cpy" -> "jnz";
                    default -> cmd;
                };
            }

            switch (cmd) {
                case "cpy" -> {
                    if (isRegister(p[2])) {
                        mem.put(p[2], getValue(mem, p[1]));
                    }
                }
                case "inc" -> {
                    if (isRegister(p[1])) {
                        mem.inc(p[1]);
                    }
                }
                case "dec" -> {
                    if (isRegister(p[1])) {
                        mem.dec(p[1]);
                    }
                }
                case "jnz" -> {
                    long value = getValue(mem, p[1]);
                    if (value != 0) {
                        i += (int) getValue(mem, p[2]);
                        continue;
                    }
                }
                case "tgl" -> {
                    int j = i + (int) getValue(mem, p[1]);
                    if (j >= 0 && j < toggled.length) {
                        toggled[j] = !toggled[j];
                    }
                }
                case "mul" -> {
                    if (isRegister(p[1])) {
                        mem.put(p[1], mem.get(p[1]) * getValue(mem, p[2]));
                    }
                }
                default -> throw new IllegalArgumentException("Unknown command: " + cmd);
            }

            i++;
        }

        return mem.getValue("a");
    }

    private static long getValue(CounterMap<String> mem, String arg) {
        return isRegister(arg) ? mem.getValue(arg) : Long.parseLong(arg);
    }

    private static boolean isRegister(String s) {
        return s.length() == 1 && Character.isLetter(s.charAt(0));
    }

    public void solve() {
        try {
            String code = Files.readString(Paths.get("./inputs/23/prod.data"));

            code = code.replace(MULTIPLICATION_ORIG, MULTIPLICATION_OPTIMIZED);
            var newLines = List.of(code.split("\n"));
            System.out.println("Part 1: " + solve(newLines, 7));
            System.out.println("Part 2: " + solve(newLines, 12));
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

}
