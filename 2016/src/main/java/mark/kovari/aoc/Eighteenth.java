package mark.kovari.aoc;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.stream.IntStream;

public class Eighteenth {


    private static int solve(String input, int rowCount) {
        int cnt = getSafeTileCount(input);

        String row = input;
        for (int i = 0; i < rowCount - 1; i++) {
            String prev = "." + row + ".";
            var sb = new StringBuilder();
            for (int j = 0, length = row.length(); j < length; j++) {
                String parents = prev.substring(j, j + 3);
                boolean isTrap = "^^.".equals(parents)
                        || ".^^".equals(parents)
                        || "..^".equals(parents)
                        || "^..".equals(parents);
                sb.append(isTrap ? '^' : '.');
            }
            row = sb.toString();
            cnt += getSafeTileCount(row);
        }

        return cnt;
    }

    private static int getSafeTileCount(String row) {
        return (int) IntStream.range(0, row.length()).filter(i -> row.charAt(i) == '.').count();
    }

    public void solve() {
        try {
            String input = Files.readAllLines(Paths.get("./inputs/18/prod.data")).get(0);


            System.out.println("Eighteenth: Part 1: " + solve(input, 40));
            System.out.println("Eighteenth: Part 2: " + solve(input, 400000));

        } catch (IOException e) {
            e.printStackTrace();
        }
    }


}
