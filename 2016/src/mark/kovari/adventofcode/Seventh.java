package src.mark.kovari.adventofcode;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class Seventh {

  public static <T> Stream<List<T>> sliding(List<T> list, int size) {
    if (size > list.size())
      return Stream.empty();
    return IntStream.range(0, list.size() - size + 1)
        .mapToObj(start -> list.subList(start, start + size));
  }

  private record IPRecord(String prefix, String middle, String suffix) {
    public static IPRecord fromString(final String from) {
      String[] parts = from.split("\\[|\\]");
      return new IPRecord(parts[0], parts[1], parts[2]);
    }

    public List<String> getABBAs() {
      List<String> result = new ArrayList<String>(mirroredPairs(prefix));
      result.addAll(mirroredPairs(suffix));
      return result;
    }

    public List<String> getMiddleABBAs() {
      return mirroredPairs(middle);
    }

    private boolean isValid() {
      return getABBAs().size() > 0 && getMiddleABBAs().size() == 0;
    }

    private boolean mirroredPair(final List<Character> of) {
      if (of.size() != 4) {
        return false;
      }
      String firstHalf = String.format("%c%c", of.get(0), of.get(1));
      String secondHalfReversed = String.format("%c%c", of.get(3), of.get(2));
      return firstHalf.equals(secondHalfReversed) && !of.get(0).equals(of.get(1));
    }

    private String fromChars(final List<Character> from) {
      return from.stream().map((c) -> c.toString()).collect(Collectors.joining());
    }

    private List<String> mirroredPairs(String from) {
      if (from.length() < 4) {
        return new ArrayList<String>();
      }
      List<Character> charList = Arrays.asList(from.chars().mapToObj(a -> (char) a).toArray(Character[]::new));
      return sliding(charList, 4)
          .filter((list) -> mirroredPair(list))
          .map((chars) -> fromChars(chars))
          .collect(Collectors.toList());
    }
  }

  public void solve() {

    try {
      String[] content = Files.readString(Paths.get("./inputs/7/prod.data")).split(System.lineSeparator());
      List<IPRecord> valids = new ArrayList<IPRecord>();
      for (String line : content) {
        var current = IPRecord.fromString(line);
        if (current.isValid()) {
          valids.add(current);
        }
      }

      System.out.println(
          "Sixth day: first result " + valids.size());
      System.out.println(
          "Sixth day: second result ");
    } catch (IOException e) {
      e.printStackTrace();
    }

  }
}