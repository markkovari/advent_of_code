package mark.kovari.aoc;

import utils.BFS;
import utils.BackTracking;
import utils.InputUtils;
import utils.Tile;

import java.io.IOException;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

public class TwentyFourth {

    private static List<Tile> findTargetTiles(char[][] map) {
        List<Tile> tiles = new ArrayList<>();
        for (int i = 0; i < map.length; i++) {
            for (int j = 0; j < map[i].length; j++) {
                if (Character.isDigit(map[i][j])) {
                    int index = map[i][j] - '0';
                    while (index >= tiles.size()) {
                        tiles.add(null);
                    }
                    tiles.set(index, new Tile(i, j));
                }
            }
        }
        return tiles;
    }

    public void solve() {
        try {
            char[][] map = InputUtils.readCharMatrix(Paths.get("./inputs/24/prod.data"));

            var targetTiles = findTargetTiles(map);
            int targetCount = targetTiles.size();

            int[][] dist = new int[targetCount][targetCount];
            for (int i = 0; i < targetCount; i++) {
                var startTile = targetTiles.get(i);
                var result = BFS.run(startTile,
                        tile -> tile.neighbors(n -> map[n.row()][n.col()] != '#'));
                for (int j = 0; j < targetCount; j++) {
                    dist[i][j] = (int) result.get(targetTiles.get(j)).getDist();
                }
            }
            var permutations = BackTracking.findAll(targetCount - 1, BackTracking::distinct);
            int min1 = Integer.MAX_VALUE;
            int min2 = Integer.MAX_VALUE;
            for (int[] order : permutations) {
                int length = 0;
                int prev = 0;
                for (int k : order) {
                    int next = k + 1;
                    length += dist[prev][next];
                    prev = next;
                }
                min1 = Math.min(min1, length);
                min2 = Math.min(min2, length + dist[prev][0]);
            }
            System.out.println("TwentyFourth: Part 1: " + min1);
            System.out.println("TwentyFourth: Part 2: " + min2);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }
}
