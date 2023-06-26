package mark.kovari.aoc;

import utils.InputUtils;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

public class Fifteenth {

    private static long solve(List<Disc> discs) {
        long startTime = 0;
        long step = 1;
        for (int i = 0; i < discs.size(); i++) {
            Disc disc = discs.get(i);
            while (((disc.startPos() + startTime + i + 1) % disc.posCount) != 0) {
                startTime += step;
            }
            step *= disc.posCount;
        }
        return startTime;
    }

    public void solve() {
        try {
            List<String> lines = Files.readAllLines(Paths.get("./inputs/15/prod.data"));

            var discs = new ArrayList<Disc>();
            for (String line : lines) {
                var parts = InputUtils.scan(line,
                        "Disc #%d has %d positions; at time=0, it is at position %d.");
                discs.add(new Disc(parts.get(1).asInt(), parts.get(2).asInt()));
            }

            long solution1 = solve(discs);

            discs.add(new Disc(11, 0));
            long solution2 = solve(discs);

            System.out.println("Fifteenth: Part 1: " + solution1);
            System.out.println("Fifteenth: Part 2: " + solution2);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    private record Disc(int posCount, int startPos) {
    }
}
