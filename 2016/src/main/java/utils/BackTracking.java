package utils;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public final class BackTracking {

    private BackTracking() {
    }

    public static List<int[]> findAll(int n, BacktrackingPredicate predicate) {
        return run(n, n, predicate, Integer.MAX_VALUE);
    }

    public static List<int[]> findAll(int n, int m, BacktrackingPredicate predicate) {
        return run(n, m, predicate, Integer.MAX_VALUE);
    }

    public static int[] findFirst(int n, int m, BacktrackingPredicate predicate) {
        return run(n, m, predicate, 1).get(0);
    }

    public static boolean distinct(int[] array, int index) {
        for (int i = 0, value = array[index]; i < index; i++) {
            if (array[i] == value) {
                return false;
            }
        }
        return true;
    }

    private static List<int[]> run(int n, int m, BacktrackingPredicate predicate, int limit) {
        List<int[]> result = new ArrayList<>();

        int[] solution = new int[n];
        Arrays.fill(solution, -1);
        for (int k = 0; k >= 0; ) {
            do {
                solution[k]++;
            } while (solution[k] < m && !predicate.accept(solution, k));

            if (solution[k] < m) {
                if (k < n - 1) {
                    k++;
                } else {
                    result.add(solution.clone());
                    if (result.size() == limit) {
                        break;
                    }
                }
            } else {
                solution[k] = -1;
                k--;
            }
        }

        return result;
    }

    public interface BacktrackingPredicate {
        boolean accept(int[] candidate, int k);
    }

}