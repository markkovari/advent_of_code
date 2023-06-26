package mark.kovari.aoc;

import com.google.common.collect.ListMultimap;
import com.google.common.collect.Multimap;
import com.google.common.collect.MultimapBuilder;
import utils.InputUtils;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.HashMap;
import java.util.Map;

public class Tenth {


    public void solve() {

        ListMultimap<Integer, Long> bots = MultimapBuilder.hashKeys().arrayListValues().build();
        Map<Integer, Long> outputs = new HashMap<>();

        try {
            String[] lines = Files.readString(Paths.get("./inputs/10/prod.data")).split(System.lineSeparator());


            for (var line : lines) {
                if (line.startsWith("value")) {
                    var parts = InputUtils.scan(line, "value %d goes to bot %d");
                    long value = parts.get(0).asLong();
                    int bot = parts.get(1).asInt();
                    bots.put(bot, value);
                }
            }

            int result1 = -1;
            while (!bots.isEmpty()) {
                for (var line : lines) {
                    if (line.startsWith("bot")) {
                        var parts = InputUtils.scan(line,
                                "bot %d gives low to %s and high to %s");
                        int bot = parts.get(0).asInt();
                        if (bots.get(bot).size() == 2) {
                            long low = Math.min(bots.get(bot).get(0), bots.get(bot).get(1));
                            long high = Math.max(bots.get(bot).get(0), bots.get(bot).get(1));
                            put(bots, outputs, low, parts.get(1).asString());
                            put(bots, outputs, high, parts.get(2).asString());
                            bots.removeAll(bot);

                            if (low == 17 && high == 61) {
                                result1 = bot;
                            }
                        }
                    }
                }
            }

            long result2 = outputs.get(0) * outputs.get(1) * outputs.get(2);

            System.out.println("Tenth day: first result " + result1);
            System.out.println("Tenth day: second result: " + result2);

        } catch (IOException e) {
            e.printStackTrace();
        }

    }

    private static void put(Multimap<Integer, Long> bots, Map<Integer, Long> outputs, long value, String destination) {
        int id = Integer.parseInt(destination.split(" ")[1]);
        if (destination.startsWith("output")) {
            outputs.put(id, value);
        } else {
            bots.put(id, value);
        }
    }


}
