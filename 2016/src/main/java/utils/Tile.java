package utils;


import java.util.List;
import java.util.function.Predicate;

/**
 * Represents a tile (cell) in a table as an immutable pair of {@code int} values: row index and column index.
 * Provides methods to get the neighbors of a tile and the Manhattan distance between two tiles.
 * <p>
 * This class is similar to {@link Point} but with different index order and names.
 *
 * @see Point
 */
public record Tile(int row, int col) {

    /**
     * Returns the Manhattan distance between the given two tiles.
     */
    public static int dist(Tile t1, Tile t2) {
        return Math.abs(t1.row - t2.row) + Math.abs(t1.col - t2.col);
    }

    /**
     * Returns true if the indices of this tile are valid with respect to the given row and column count.
     *
     * @return true if both indices are between zero (inclusive) and the given count (exclusive).
     */
    public boolean isValid(int rowCount, int colCount) {
        return row >= 0 && row < rowCount && col >= 0 && col < colCount;
    }

    /**
     * Returns the neighbor in the given direction. The tile {@code (0,0)} represents the upper-left corner
     * of the table, and {@code (rowCount-1,colCount-1)} represents the bottom-right corner. The directions
     * are interpreted accordingly, so "up" or "north" means lower row index, while "down" or "south" means higher
     * row index.
     *
     * @param direction the direction. One of 'u' (up), 'd' (down), 'l' (left), 'r' (right),
     *                  'n' (north), 's' (south), 'w' (west), 'e' (east), and their uppercase variants.
     */
    public Tile neighbor(char direction) {
        return switch (direction) {
            case 'u', 'U', 'n', 'N' -> new Tile(row - 1, col);
            case 'd', 'D', 's', 'S' -> new Tile(row + 1, col);
            case 'l', 'L', 'w', 'W' -> new Tile(row, col - 1);
            case 'r', 'R', 'e', 'E' -> new Tile(row, col + 1);
            default -> throw new IllegalArgumentException("Unknown direction: '" + direction + "'.");
        };
    }

    /**
     * Returns the four neighbors of this tile.
     *
     * @return the four neighbor tiles in clockwise order (N, E, S, W)
     */
    public List<Tile> neighbors() {
        return List.of(
                new Tile(row - 1, col),
                new Tile(row, col + 1),
                new Tile(row + 1, col),
                new Tile(row, col - 1));
    }

    /**
     * Returns the neighbors of this tile that are accepted by the given custom predicate.
     *
     * @return the selected neighbors in clockwise order (N, E, S, W)
     */
    public List<Tile> neighbors(Predicate<Tile> predicate) {
        return neighbors().stream().filter(predicate).toList();
    }

    /**
     * Returns the {@link #isValid(int, int) valid} neighbors of this tile with respect to the given row count and
     * column count.
     *
     * @return the valid neighbors in clockwise order (N, E, S, W)
     */
    public List<Tile> validNeighbors(int rowCount, int colCount) {
        return neighbors(t -> t.isValid(rowCount, colCount));
    }

    /**
     * Returns the {@link #isValid(int, int) valid} neighbors of this tile with respect to the given row count,
     * column count, and custom predicate.
     *
     * @return the selected valid neighbors in clockwise order (N, E, S, W)
     */
    public List<Tile> validNeighbors(int rowCount, int colCount, Predicate<Tile> predicate) {
        return neighbors(t -> t.isValid(rowCount, colCount) && predicate.test(t));
    }

    /**
     * Returns the eight tiles that are close to this one: the four neighbors and the four tiles in diagonal
     * directions.
     *
     * @return the eight neighbor tiles (including diagonal directions) in clockwise order (N, NE, E, SE, S, SW, W, NW)
     */
    public List<Tile> eightNeighbors() {
        return List.of(
                new Tile(row - 1, col),
                new Tile(row - 1, col + 1),
                new Tile(row, col + 1),
                new Tile(row + 1, col + 1),
                new Tile(row + 1, col),
                new Tile(row + 1, col - 1),
                new Tile(row, col - 1),
                new Tile(row - 1, col - 1));
    }

    /**
     * Returns the Manhattan distance between this tile and the given tile.
     */
    public int dist(Tile t) {
        return dist(this, t);
    }

}
