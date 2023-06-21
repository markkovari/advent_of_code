package src.mark.kovari.adventofcode;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.security.Identity;
import java.util.ArrayList;
import java.util.Collection;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class Fourth {
  static Pattern pattern = Pattern.compile("([a-z-]+)-(\\d+)\\[([a-z]+)\\]");

  private record Room(String name, int sector, String checksum) {

    private static Room fromString(String line) {
      Matcher matcher = pattern.matcher(line);
      if (!matcher.matches()) {
        return null;
      }
      String name = matcher.group(1).replaceAll("-", " ");
      int sector = Integer.parseInt(matcher.group(2));
      String checksum = matcher.group(3);
      return new Room(name, sector, checksum);
    }

    private Map<Character, Integer> countLetters() {
      Map<Character, Integer> counts = new TreeMap<Character, Integer>();
      for (char c : name.toCharArray()) {
        if (c == '-' || c == ' ') {
          continue;
        }
        counts.put(c, counts.getOrDefault(c, 0) + 1);
      }
      return counts;
    }

    private String calculateChecksum() {
      Map<Character, Integer> counts = countLetters();

      List<Map.Entry<Character, Integer>> sortedCounts = counts.entrySet()
          .stream()
          .sorted(Map.Entry.<Character, Integer>comparingByValue().reversed()
              .thenComparing(Map.Entry.comparingByKey()))
          .collect(Collectors.toList());

      return sortedCounts.stream().limit(5).map(Map.Entry::getKey).map(String::valueOf).collect(Collectors.joining());
    }

    public boolean valid() {
      return calculateChecksum().equals(checksum);
    }

    public Integer getSectorValue() {
      return valid() ? sector : 0;
    }

    private String decrypt() {
      StringBuilder builder = new StringBuilder();
      for (char c : name.toCharArray()) {
        if (c == '-') {
          builder.append(' ');
          continue;
        }
        builder.append((char) (((c - 'a' + sector) % 26) + 'a'));
      }
      return builder.toString();
    }
  }

  public void solve() {
    try {
      List<Room> rooms = new ArrayList<Room>();
      String[] content = Files.readString(Paths.get("./inputs/4/prod.data")).split(System.lineSeparator());
      for (String line : content) {
        Room room = Room.fromString(line.trim());
        if (room != null) {
          rooms.add(room);
          System.out.println(room.decrypt() + " " + room.getSectorValue());
        }
      }
      System.out.println(
          "Fourth day: first result " + rooms.stream().map(Room::getSectorValue).reduce(0, Integer::sum));

    } catch (IOException e) {
      e.printStackTrace();
    }
  }
}
