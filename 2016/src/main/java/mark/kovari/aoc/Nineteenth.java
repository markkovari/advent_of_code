package mark.kovari.aoc;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

public class Nineteenth {


    public static int play(int playerCount, boolean advanced) {
        int[] next = new int[playerCount];
        for (int i = 0; i < next.length - 1; i++) {
            next[i] = i + 1;
        }

        int i = advanced ? playerCount / 2 - 1 : 0;
        for (int count = playerCount; count > 1; count--) {
            next[i] = next[next[i]];
            i = advanced && (count % 2 == 0) ? i : next[i];
        }
        return i + 1;
    }

    public void solve() {
        try {
            int input = Integer.parseInt(Files.readAllLines(Paths.get("./inputs/19/prod.data")).get(0));
            System.out.println("Eighteenth: Part 1: " + play(input, false));
            System.out.println("Eighteenth: Part 2: " + play(input, true));

        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
