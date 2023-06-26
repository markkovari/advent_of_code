package mark.kovari.aoc;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Set;

public class Second {

  private record Diff(int x, int y) {
  }

  private enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
  }

  private static HashMap<Direction, Diff> moves = new HashMap<>() {
    {
      put(Direction.UP, new Diff(0, -1));
      put(Direction.DOWN, new Diff(0, +1));
      put(Direction.LEFT, new Diff(-1, 0));
      put(Direction.RIGHT, new Diff(1, 0));
    }
  };

  private record Position(int x, int y, Integer value) {
  }

  private record PositionedString(int x, int y, String value) {
  }

  private Diff getMove(Direction button) {
    return moves.get(button);
  }

  private Set<Position> buttons = new HashSet<>() {
    {
      add(new Position(0, 0, 1));
      add(new Position(1, 0, 2));
      add(new Position(2, 0, 3));
      add(new Position(0, 1, 4));
      add(new Position(1, 1, 5));
      add(new Position(2, 1, 6));
      add(new Position(0, 2, 7));
      add(new Position(1, 2, 8));
      add(new Position(2, 2, 9));
    }
  };

  private Set<PositionedString> buttons2 = new HashSet<>() {
    {
      add(new PositionedString(0, 0, null));
      add(new PositionedString(1, 0, null));
      add(new PositionedString(2, 0, "1"));
      add(new PositionedString(3, 0, null));
      add(new PositionedString(4, 0, null));

      add(new PositionedString(0, 1, null));
      add(new PositionedString(1, 1, "2"));
      add(new PositionedString(2, 1, "3"));
      add(new PositionedString(3, 1, "4"));
      add(new PositionedString(4, 1, null));

      add(new PositionedString(0, 2, "5"));
      add(new PositionedString(1, 2, "6"));
      add(new PositionedString(2, 2, "7"));
      add(new PositionedString(3, 2, "8"));
      add(new PositionedString(4, 2, "9"));

      add(new PositionedString(0, 3, null));
      add(new PositionedString(1, 3, "A"));
      add(new PositionedString(2, 3, "B"));
      add(new PositionedString(3, 3, "C"));
      add(new PositionedString(4, 3, null));

      add(new PositionedString(0, 4, null));
      add(new PositionedString(1, 4, null));
      add(new PositionedString(2, 4, "D"));
      add(new PositionedString(3, 4, null));
      add(new PositionedString(4, 4, null));
    }
  };

  private Position current = new Position(1, 1, 5);
  private PositionedString current2 = new PositionedString(0, 2, "5");

  private Position moveTo(Direction direction, Position current) {
    Diff move = getMove(direction);
    int newX = current.x + move.x;
    int newY = current.y + move.y;
    return buttons.stream().filter(p -> p.x == newX && p.y == newY).findFirst().orElse(null);
  }

  private PositionedString moveTo2(Direction direction, PositionedString current) {
    Diff move = getMove(direction);
    int newX = current.x + move.x;
    int newY = current.y + move.y;
    return buttons2.stream().filter(p -> p.x == newX && p.y == newY).findFirst().orElse(null);
  }

  private String calculate(String[] input) {
    StringBuilder sb = new StringBuilder();
    for (String line : input) {
      for (char c : line.toCharArray()) {
        Direction direction = switch (c) {
          case 'U' -> Direction.UP;
          case 'D' -> Direction.DOWN;
          case 'L' -> Direction.LEFT;
          case 'R' -> Direction.RIGHT;
          default -> throw new IllegalStateException("Unexpected value: " + c);
        };
        Position next = moveTo(direction, current);
        if (next != null) {
          current = next;
        }
      }
      sb.append(current.value);
    }
    return sb.toString();
  }

  private String calculate2(String[] input) {
    StringBuilder sb = new StringBuilder();
    for (String line : input) {
      for (char c : line.toCharArray()) {
        Direction direction = switch (c) {
          case 'U' -> Direction.UP;
          case 'D' -> Direction.DOWN;
          case 'L' -> Direction.LEFT;
          case 'R' -> Direction.RIGHT;
          default -> throw new IllegalStateException("Unexpected value: " + c);
        };
        PositionedString next = moveTo2(direction, current2);
        if (next != null && next.value != null) {
          current2 = next;
        }
      }
      if (current2.value != null) {
        sb.append(current2.value);
      }
    }
    return sb.toString();
  }

  public void solve() {
    try {
      String[] content = Files.readString(Paths.get("./inputs/2/prod.data")).split(System.lineSeparator());
      String result = calculate(content);
      System.out.println(result);
      String result2 = calculate2(content);
      System.out.println(result2);
    } catch (IOException e) {
      e.printStackTrace();
    }
  }
}
