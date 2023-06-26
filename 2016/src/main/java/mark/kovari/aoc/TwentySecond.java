package mark.kovari.aoc;

import utils.BFS;
import utils.InputUtils;
import utils.Point;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;
import java.util.Map.Entry;
import java.util.function.Predicate;
import java.util.stream.Collectors;

public class TwentySecond {

    private static boolean areAdjacent(Point p1, Point p2) {
        return Math.abs(p1.x() - p2.x()) <= 1 && Math.abs(p1.y() - p2.y()) <= 1;
    }

    private static int countViablePairs(Map<Point, Storage> map) {
        int cnt = 0;
        for (Point p1 : map.keySet()) {
            for (Point p2 : map.keySet()) {
                if (isViablePair(map, p1, p2)) {
                    cnt++;
                }
            }
        }
        return cnt;
    }

    private static boolean isViablePair(Map<Point, Storage> map, Point p1, Point p2) {
        return !p1.equals(p2) && map.get(p1).used() > 0 && map.get(p1).used() <= map.get(p2).avail();
    }

    public void solve() {
        try {
            List<String> lines = Files.readAllLines(Paths.get("./inputs/22/prod.data"));

            Map<Point, Storage> map = new HashMap<Point, Storage>();
            for (int i = 2; i < lines.size(); i++) {
                var line = lines.get(i);
                var parts = InputUtils.scan(line, "/dev/grid/node-x%d-y%d *%dT *%dT *%dT.*");
                var point = new Point(parts.get(0).asInt(), parts.get(1).asInt());
                var storage = new Storage(parts.get(3).asInt(), parts.get(4).asInt());
                map.put(point, storage);
            }

            long width = map.keySet().stream().mapToLong(Point::x).max().orElseThrow() + 1;
            long height = map.keySet().stream().mapToLong(Point::y).max().orElseThrow() + 1;


            int minCapacity = map.values().stream().mapToInt(st -> st.used() + st.avail()).min().getAsInt();
            Set<Point> walls = map.entrySet().stream()
                    .filter(entry -> entry.getValue().used() > minCapacity)
                    .map(Entry::getKey)
                    .collect(Collectors.toSet());

            // Find initial positions of the data and the hole tiles
            var dataStart = new Point(width - 1, 0);
            var holeStart = map.entrySet().stream()
                    .filter(entry -> entry.getValue().used() == 0)
                    .map(Entry::getKey)
                    .findFirst()
                    .orElseThrow();

            var result = BFS.run(new State(dataStart, holeStart),
                    state -> {
                        boolean alreadyAdjacent = areAdjacent(state.data(), state.hole());
                        var nextStates = new ArrayList<State>();
                        for (var newHole : state.hole().validNeighbors(width, height, Predicate.not(walls::contains))) {
                            var newData = newHole.equals(state.data()) ? state.hole() : state.data();

                            // Heuristic to prune search space: once the hole tile becomes adjacent to the data tile
                            // (i.e. one of its 8 neighbor tiles), they should remain adjacent. There is no reason to
                            // increase their distance. This heuristic results in ~130 times fewer nodes visited and
                            // more than 50 times speed-up.
                            if (alreadyAdjacent && !areAdjacent(newData, newHole)) {
                                continue;
                            }
                            nextStates.add(new State(newData, newHole));
                        }
                        return nextStates;
                    },
                    state -> new Point(0, 0).equals(state.data())
            );

            System.out.println("Part 1: " + countViablePairs(map));
            System.out.println("Part 2: " + result.orElseThrow().getDist());
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    private static record Storage(int used, int avail) {
    }

    private static record State(Point data, Point hole) {
    }

}
