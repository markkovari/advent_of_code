package mark.kovari.aoc;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.Set;
import java.util.Map.Entry;
import java.util.stream.IntStream;

import static java.util.Comparator.comparing;
import static java.util.stream.Collectors.counting;
import static java.util.stream.Collectors.groupingBy;
import static java.util.stream.Collectors.joining;

public class Sixth {
  private static String decode(List<String> lines, boolean reversed) {
    var msg = new StringBuilder();
    for (int k = 0; k < lines.get(0).length(); k++) {
      var counts = new HashMap<Character, Integer>();
      for (var line : lines) {
        counts.merge(line.charAt(k), 1, Integer::sum);
      }

      char ch = counts.entrySet().stream()
          .max(comparing(e -> reversed ? -e.getValue() : e.getValue()))
          .orElseThrow()
          .getKey();
      msg.append(ch);
    }
    return msg.toString();
  }

  private static String decodeSecond(List<String> lines, boolean reversed) {
    return IntStream.range(0, lines.get(0).length())
        .mapToObj(k -> lines.stream().collect(groupingBy(line -> line.charAt(k), counting())))
        .map(Map::entrySet)
        .map(Set::stream)
        .map(s -> reversed ? s.min(Entry.comparingByValue()) : s.max(Entry.comparingByValue()))
        .map(Optional::orElseThrow)
        .map(Entry::getKey)
        .map(String::valueOf)
        .collect(joining());
  }

  public void solve() {

    try {
      List<String> lines = new ArrayList<String>();
      String[] content = Files.readString(Paths.get("./inputs/6/prod.data")).split(System.lineSeparator());
      for (String line : content) {
        lines.add(line);
        System.out.println(line);
      }
      System.out.println(
          "Sixth day: first result " + decode(lines, false));
      System.out.println(
          "Sixth day: seconbd result " + decodeSecond(lines, true));
    } catch (IOException e) {
      e.printStackTrace();
    }
  }
}
