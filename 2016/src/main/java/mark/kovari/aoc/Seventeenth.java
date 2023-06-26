package mark.kovari.aoc;

import com.google.common.hash.Hashing;
import utils.BFS;
import utils.PathResult;
import utils.Tile;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

public class Seventeenth {

    private static final char[] DIRS = {'U', 'D', 'L', 'R'};
    private static final int SIZE = 4;
    private static final Tile START_TILE = new Tile(0, 0);
    private static final Tile TARGET_TILE = new Tile(3, 3);

    private static List<State> getNextFeasibleStates(State state, String passCode) {
        if (state.tile().equals(TARGET_TILE)) {
            return List.of();
        }

        String hash = getMd5Hash(passCode + state.path());

        var result = new ArrayList<State>(4);
        for (int i = 0; i < 4; i++) {
            var neighbor = state.tile().neighbor(DIRS[i]);
            boolean doorIsOpen = neighbor.isValid(SIZE, SIZE)
                    && hash.charAt(i) >= 'b' && hash.charAt(i) <= 'f';
            if (doorIsOpen) {
                result.add(new State(neighbor, state.path + DIRS[i]));
            }
        }

        return result;
    }

    @SuppressWarnings({"deprecated", "UnstableApiUsage"})
    private static String getMd5Hash(String s) {
        return Hashing.md5().hashString(s, StandardCharsets.UTF_8).toString();
    }

    public void solve() {
        try {
            String passCode = Files.readAllLines(Paths.get("./inputs/17/prod.data")).get(0);
            var resultMap = BFS.run(new State(START_TILE, ""),
                    st -> getNextFeasibleStates(st, passCode));

            String shortestPath = resultMap.values().stream()
                    .filter(res -> res.getNode().tile().equals(TARGET_TILE))
                    .min(Comparator.comparing(PathResult::getDist))
                    .orElseThrow()
                    .getNode().path();

            long lengthOfLongestPath = resultMap.values().stream()
                    .filter(res -> res.getNode().tile().equals(TARGET_TILE))
                    .mapToLong(PathResult::getDist)
                    .max().orElseThrow();

            System.out.println("Seventeenth: Part 1: " + shortestPath);
            System.out.println("Seventeenth: Part 2: " + lengthOfLongestPath);

        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    private static record State(Tile tile, String path) {
    }
}
