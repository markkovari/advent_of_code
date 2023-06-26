package mark.kovari.aoc;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.Optional;

public class Twentieth {

    private static Optional<Range> findRange(List<Range> ranges, long i) {
        return ranges.stream().filter(r -> r.contains(i)).findAny();
    }

    public void solve() {
        try {
            List<String> content = Files.readAllLines(Paths.get("./inputs/20/prod.data"));
            var ranges = content.stream().map(Range::new).toList();

            long first = -1;
            long cnt = 0;
            for (long i = 0; i <= 4294967295L; i++) {
                var range = findRange(ranges, i);
                if (range.isPresent()) {
                    i = range.get().max;
                } else {
                    cnt++;
                    if (first == -1) {
                        first = i;
                    }
                }
            }


            System.out.println("Twentieth: Part 1: " + first);
            System.out.println("Twentieth: Part 2: " + cnt);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    private static record Range(long min, long max) {

        private Range(String line) {
            this(Long.parseLong(line.split("-")[0]), Long.parseLong(line.split("-")[1]));
        }

        private boolean contains(long i) {
            return i >= min && i <= max;
        }

    }
}
