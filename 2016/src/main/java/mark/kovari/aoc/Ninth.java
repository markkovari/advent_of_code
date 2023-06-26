package mark.kovari.aoc;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

public class Ninth {


    private static long getDecompressedLength(String str, boolean recursive) {
        long total = 0;
        for (int i = 0; i < str.length(); i++) {
            char ch = str.charAt(i);
            if (ch == ' ') {
                continue;
            }
            if (ch == '(') {
                var endIndex = str.indexOf(')', i);
                var marker = str.substring(i + 1, endIndex).split("x");

                int charCount = Integer.parseInt(marker[0]);
                int count = Integer.parseInt(marker[1]);
                var toRepeat = str.substring(endIndex + 1, endIndex + charCount + 1);
                var length = recursive ? getDecompressedLength(toRepeat, true) : toRepeat.length();
                total += length * count;

                i = endIndex + charCount;
            } else {
                total++;
            }
        }
        return total;
    }

    public void solve() {

        try {
            String[] lines = Files.readString(Paths.get("./inputs/9/prod.data")).split(System.lineSeparator());

            System.out.println("Ninth day: first result " + getDecompressedLength(lines[0], false));

            System.out.println(
                    "Ninth day: second result " + getDecompressedLength(lines[0], true));
        } catch (IOException e) {
            e.printStackTrace();
        }

    }


}
