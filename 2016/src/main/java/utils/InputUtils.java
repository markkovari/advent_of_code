package utils;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.regex.MatchResult;
import java.util.regex.Pattern;

public final class InputUtils {

    private static final Pattern decimalPattern = Pattern.compile("-?\\d+");

    private InputUtils() {
    }

    public static String readString(String fileName) {
        try {
            return Files.readString(getPath(fileName), StandardCharsets.UTF_8)
                    .replaceAll("\r\n", "\n")
                    .replaceAll("\r", "\n");
        } catch (IOException e) {
            throw new IllegalArgumentException(e);
        }
    }

    public static String readSingleLine(String fileName) {
        return readLines(fileName).get(0);
    }

    public static List<String> readLines(String fileName) {
        try {
            return Files.readAllLines(getPath(fileName), StandardCharsets.UTF_8);
        } catch (IOException e) {
            throw new IllegalArgumentException(e);
        }
    }

    public static int[] parseInts(String input) {
        return decimalPattern.matcher(input)
                .results()
                .map(MatchResult::group)
                .mapToInt(Integer::parseInt)
                .toArray();
    }

    public static long[] parseLongs(String input) {
        return decimalPattern.matcher(input)
                .results()
                .map(MatchResult::group)
                .mapToLong(Long::parseLong)
                .toArray();
    }

    public static char[][] readCharMatrix(String fileName) {
        var lines = readLines(fileName);
        var matrix = new char[lines.size()][];
        for (int i = 0, n = matrix.length; i < n; i++) {
            matrix[i] = new char[lines.get(i).length()];
            for (int j = 0, m = matrix[i].length; j < m; j++) {
                matrix[i][j] = lines.get(i).charAt(j);
            }
        }
        return matrix;
    }

    private static Path getPath(String fileName) {
        return Path.of(InputUtils.class.getResource("../" + fileName).getPath());
    }

    public static List<ParsedValue> scan(String str, String pattern) throws IllegalArgumentException {
        var groupFinder = Pattern.compile("%.").matcher(pattern);
        var groupPatterns = new ArrayList<String>();
        while (groupFinder.find()) {
            groupPatterns.add(groupFinder.group());
        }

        var regex = pattern.replaceAll("%d", "(\\\\d+)")
                .replaceAll("%c", "(.)")
                .replaceAll("%s", "(.*)");

        var result = new ArrayList<ParsedValue>();
        var matcher = Pattern.compile(regex).matcher(str);
        if (matcher.matches()) {
            if (matcher.groupCount() == groupPatterns.size()) {
                for (int i = 0; i < groupPatterns.size(); i++) {
                    var groupPattern = groupPatterns.get(i);
                    var group = matcher.group(i + 1); // 0-th group is the entire match
                    result.add(ParsedValue.parse(group, groupPattern));
                }
            } else {
                throw new IllegalArgumentException(String.format(
                        "Input string '%s' has %d groups instead of expected %d for RegEx '%s'"
                                + " (created from pattern '%s').",
                        str, matcher.groupCount(), groupPatterns.size(), regex, pattern));
            }
        } else {
            throw new IllegalArgumentException(String.format(
                    "Input string '%s' does not match the RegEx '%s' (created from pattern '%s').",
                    str, regex, pattern));
        }

        return result;
    }

    public final static class ParsedValue {

        private final Object value;

        private ParsedValue(Object value) {
            this.value = value;
        }

        private static ParsedValue parse(String s, String pattern) {
            if ("%d".equals(pattern)) {
                return new ParsedValue(Long.parseLong(s));
            } else if ("%c".equals(pattern)) {
                return new ParsedValue(s.charAt(0));
            } else {
                return new ParsedValue(s);
            }
        }

        public int asInt() {
            return (int) (long) value;
        }

        public long asLong() {
            return (long) value;
        }

        public char asChar() {
            return (char) value;
        }

        public String asString() {
            return value.toString();
        }

        public String get() {
            return value.toString();
        }

        public boolean isInteger() {
            return value.getClass().equals(Long.class);
        }

        public boolean isChar() {
            return value.getClass().equals(Character.class);
        }

        public boolean isString() {
            return value.getClass().equals(String.class);
        }

        @Override
        public String toString() {
            return "ParsedValue(" + value.getClass().getSimpleName() + ": " + value + ")";
        }
    }
}
