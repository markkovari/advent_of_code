package utils;

import java.util.ArrayDeque;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.function.Function;
import java.util.function.Predicate;

/**
 * A general implementation of the BFS (breadth-first search) algorithm.
 * <p>
 * The input is an "implicit" directed graph (defined by a neighbor provider function) and one or more source nodes.
 * The nodes often represent feasible states of a puzzle, and the directed edges represent the transformation steps.
 * An optional target predicate can also be specified in order to get path result for a single node instead of
 * all nodes. In this case, the algorithm terminates when the shortest path is found for at least one target node
 * (more precisely, when all target nodes having minimum distance are found). This way, we can search paths even in a
 * potentially infinite graph of feasible states and steps.
 */
public final class BFS {

    private BFS() {
    }

    public static <T> Optional<PathResult<T>> run(T source, Function<T, Iterable<T>> neighborProvider,
                                                  Predicate<T> targetPredicate) {
        var map = run(List.of(source), neighborProvider, targetPredicate);
        return map.values().stream().filter(PathResult::isTarget).min(Comparator.comparing(PathResult::getDist));
    }

    public static <T> Map<T, PathResult<T>> run(T source, Function<T, Iterable<T>> neighborProvider) {
        return run(List.of(source), neighborProvider, t -> false);
    }

    public static <T> Map<T, PathResult<T>> run(Iterable<T> sources, Function<T, Iterable<T>> neighborProvider,
                                                Predicate<T> targetPredicate) {
        var results = new HashMap<T, PathResult<T>>();

        var queue = new ArrayDeque<T>();
        for (T source : sources) {
            results.put(source, new PathResult<>(source, 0, targetPredicate.test(source), null));
            queue.add(source);
        }

        while (!queue.isEmpty()) {
            T node = queue.poll();
            var result = results.get(node);
            if (result.isTarget()) {
                break;
            }
            for (T neighbor : neighborProvider.apply(node)) {
                if (!results.containsKey(neighbor)) {
                    results.put(neighbor, new PathResult<>(neighbor, result.getDist() + 1,
                            targetPredicate.test(neighbor), result));
                    queue.add(neighbor);
                }
            }
        }

        return results;
    }

}