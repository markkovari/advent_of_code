package mark.kovari.aoc;

import utils.BFS;
import utils.Point;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

public class Thirteenth {

    private static boolean isNotWall(Point point, long shift) {
        return (Long.bitCount(getMagicNumber(point.x(), point.y(), shift)) & 1) == 0;
    }

    private static long getMagicNumber(long x, long y, long shift) {
        return x * (x + 3) + y * (x + x + y + 1) + shift;
    }

    public void solve() {
        try {
            long shift = Integer.parseInt(Files.readString(Paths.get("./inputs/12/prod.data")));

            var start = new Point(1, 1);
            var target = new Point(31, 39);
            var mapSize = 52;

            var results = BFS.run(start,
                    point -> point.validNeighbors(mapSize, mapSize, p -> isNotWall(p, shift)));

            System.out.println("Thirteenth Part 1: " + results.get(target).getDist());
            System.out.println("Thirteenth Part 2: " + results.values().stream().filter(res -> res.getDist() <= 50).count());

        } catch (IOException e) {
            e.printStackTrace();
        }

    }
}
