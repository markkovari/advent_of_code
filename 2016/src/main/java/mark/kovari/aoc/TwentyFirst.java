package mark.kovari.aoc;

import utils.InputUtils;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.List;

public class TwentyFirst {
    private static String scramble(String password, List<String> commands, boolean reversed) {
        char[] s = password.toCharArray();

        for (int i = 0; i < commands.size(); i++) {
            String line = reversed ? commands.get(commands.size() - 1 - i) : commands.get(i);

            if (line.startsWith("swap position")) {
                var parts = InputUtils.scan(line, "swap position %d with position %d");
                swap(s, parts.get(0).asInt(), parts.get(1).asInt(), reversed);
            } else if (line.startsWith("swap letter")) {
                var parts = InputUtils.scan(line, "swap letter %s with letter %s");
                swap(s, parts.get(0).get().charAt(0), parts.get(1).get().charAt(0), reversed);
            } else if (line.startsWith("reverse positions")) {
                var parts = InputUtils.scan(line, "reverse positions %d through %d");
                reverse(s, parts.get(0).asInt(), parts.get(1).asInt());
            } else if (line.startsWith("move position")) {
                var parts = InputUtils.scan(line, "move position %d to position %d");
                move(s, parts.get(0).asInt(), parts.get(1).asInt(), reversed);
            } else if (line.startsWith("rotate left") || line.startsWith("rotate right")) {
                var parts = InputUtils.scan(line, "rotate %s %d step.?");
                boolean right = "right".equals(parts.get(0).get()) ^ reversed;
                for (int k = 0; k < parts.get(1).asInt(); k++) {
                    if (right) {
                        rotateRight(s);
                    } else {
                        rotateLeft(s);
                    }
                }
            } else if (line.startsWith("rotate based on")) {
                var parts = InputUtils.scan(line, "rotate based on position of letter %s");
                char c = parts.get(0).get().charAt(0);
                if (reversed) {
                    char[] result = s.clone();
                    while (!checkRotateBasedOnLetter(s, result, c)) {
                        rotateLeft(s);
                    }
                } else {
                    rotateBasedOnLetter(s, c);
                }
            } else {
                throw new IllegalArgumentException();
            }
        }

        return toString(s);
    }

    private static String toString(char[] s) {
        var sb = new StringBuilder();
        for (char c : s) {
            sb.append(c);
        }
        return sb.toString();
    }

    private static void swap(char[] s, int i, int j, boolean reversed) {
        if (reversed) {
            swap(s, j, i);
        } else {
            swap(s, i, j);
        }
    }

    private static void swap(char[] s, int i, int j) {
        char c = s[i];
        s[i] = s[j];
        s[j] = c;
    }

    private static void swap(char[] s, char c1, char c2, boolean reversed) {
        if (reversed) {
            swap(s, c2, c1);
        } else {
            swap(s, c1, c2);
        }
    }

    private static void swap(char[] s, char c1, char c2) {
        for (int i = 0; i < s.length; i++) {
            if (s[i] == c1) {
                s[i] = c2;
            } else if (s[i] == c2) {
                s[i] = c1;
            }
        }
    }

    private static void reverse(char[] s, int from, int to) {
        for (int i = from, j = to; i < j; i++, j--) {
            swap(s, i, j);
        }
    }

    private static void move(char[] s, int from, int to, boolean reversed) {
        if (reversed) {
            move(s, to, from);
        } else {
            move(s, from, to);
        }
    }

    private static void move(char[] s, int from, int to) {
        char c = s[from];
        if (from < to) {
            System.arraycopy(s, from + 1, s, from, to - from);
        } else if (from > to) {
            System.arraycopy(s, to, s, to + 1, from - to);
        }
        s[to] = c;
    }

    private static void rotateLeft(char[] s) {
        char c = s[0];
        System.arraycopy(s, 1, s, 0, s.length - 1);
        s[s.length - 1] = c;
    }

    private static void rotateRight(char[] s) {
        char c = s[s.length - 1];
        System.arraycopy(s, 0, s, 1, s.length - 1);
        s[0] = c;
    }

    private static void rotateBasedOnLetter(char[] s, char c) {
        int i = 0;
        while (s[i] != c) {
            i++;
        }
        int rotationCount = i + (i >= 4 ? 2 : 1);
        for (int k = 0; k < rotationCount; k++) {
            rotateRight(s);
        }
    }

    private static boolean checkRotateBasedOnLetter(char[] from, char[] to, char c) {
        char[] s = from.clone();
        rotateBasedOnLetter(s, c);
        return Arrays.equals(s, to);
    }

    public void solve() {
        try {
            List<String> content = Files.readAllLines(Paths.get("./inputs/21/prod.data"));


            System.out.println("Twentieth: Part 1: " + scramble("abcdefgh", content, false));
            System.out.println("Twentieth: Part 2: " + scramble("fbgdceah", content, true));
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

}
