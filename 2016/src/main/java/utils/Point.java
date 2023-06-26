package utils;

import java.util.Collection;
import java.util.List;
import java.util.function.Predicate;

public record Point(long x, long y) {

    /**
     * Returns the Manhattan distance between the given two points.
     */
    public static long dist(Point p1, Point p2) {
        return Math.abs(p1.x - p2.x) + Math.abs(p1.y - p2.y);
    }

    /**
     * Returns true if the coordinates of this point are valid with respect to the given width and height
     * of a rectangle.
     *
     * @return true if both coordinates are between zero (inclusive) and the given width/height (exclusive).
     */
    public boolean isValid(long width, long height) {
        return x >= 0 && x < width && y >= 0 && y < height;
    }

    /**
     * Returns the four neighbors of this point.
     */
    public Collection<Point> neighbors() {
        return List.of(
                new Point(x - 1, y),
                new Point(x + 1, y),
                new Point(x, y - 1),
                new Point(x, y + 1));
    }

    /**
     * Returns the neighbors of this point that are accepted by the given custom predicate.
     */
    public Collection<Point> neighbors(Predicate<Point> predicate) {
        return neighbors().stream().filter(predicate).toList();
    }

    /**
     * Returns the {@link #isValid(long, long) valid} neighbors of this point with respect to the given width and
     * height.
     */
    public Collection<Point> validNeighbors(long width, long height) {
        return neighbors(p -> p.isValid(width, height));
    }

    /**
     * Returns the {@link #isValid(long, long) valid} neighbors of this point with respect to the given width,
     * height, and custom predicate.
     */
    public Collection<Point> validNeighbors(long width, long height, Predicate<Point> predicate) {
        return neighbors(p -> p.isValid(width, height) && predicate.test(p));
    }

    /**
     * Returns the Manhattan distance between this point and the given point.
     */
    public long dist(Point p) {
        return dist(this, p);
    }

}