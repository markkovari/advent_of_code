package src.mark.kovari.adventofcode;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.HashSet;
import java.util.Set;

public class First {
  enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST
  }

  record Point(int x, int y) {

  }

  public static void solve() {

    var currentDirection = Direction.NORTH;
    var currentX = 0;
    var currentY = 0;
    Set<Point> visited = new HashSet<Point>();
    visited.add(new Point(currentX, currentY));
    try {
      String content = Files.readString(Paths.get("./inputs/1/prod.data"));
      String[] lines = content.split(", ");
      main: for (int i = 0; i < lines.length; i++) {
        var direction = lines[i].charAt(0);
        var distance = Integer.parseInt(lines[i].substring(1));
        switch (direction) {
          case 'R':
            switch (currentDirection) {
              case NORTH:
                currentDirection = Direction.EAST;
                for (int j = 0; j < distance; j++) {
                  currentX += 1;
                  Point current = new Point(currentX, currentY);
                  if (!visited.contains(current)) {
                    visited.add(current);
                  } else {
                    System.out.println(i + " " + current.toString());
                    break main;
                  }
                }
                break;
              case EAST:
                currentDirection = Direction.SOUTH;
                for (int j = 0; j < distance; j++) {
                  currentY -= 1;
                  Point current = new Point(currentX, currentY);
                  if (!visited.contains(current)) {
                    visited.add(current);
                  } else {
                    System.out.println(i + " " + current.toString());
                    break main;
                  }
                }
                break;
              case SOUTH:
                currentDirection = Direction.WEST;
                for (int j = 0; j < distance; j++) {
                  currentX -= 1;
                  Point current = new Point(currentX, currentY);
                  if (!visited.contains(current)) {
                    visited.add(current);
                  } else {
                    System.out.println(i + " " + current.toString());
                    break;
                  }
                }
                break;
              case WEST:
                currentDirection = Direction.NORTH;
                for (int j = 0; j < distance; j++) {
                  currentY += 1;
                  Point current = new Point(currentX, currentY);
                  if (!visited.contains(current)) {
                    visited.add(current);
                  } else {
                    System.out.println(i + " " + current.toString());
                    break;
                  }
                }
                break;
            }
            break;
          case 'L':
            switch (currentDirection) {
              case NORTH:
                currentDirection = Direction.WEST;
                for (int j = 0; j < distance; j++) {
                  currentX -= 1;
                  Point current = new Point(currentX, currentY);
                  if (!visited.contains(current)) {
                    visited.add(current);
                  } else {
                    System.out.println(i + " " + current.toString());
                    break;
                  }
                }
                break;
              case EAST:
                currentDirection = Direction.NORTH;
                for (int j = 0; j < distance; j++) {
                  currentY += 1;
                  Point current = new Point(currentX, currentY);
                  if (!visited.contains(current)) {
                    visited.add(current);
                  } else {
                    System.out.println(i + " " + current.toString());
                    break;
                  }
                }
                break;
              case SOUTH:
                currentDirection = Direction.EAST;
                for (int j = 0; j < distance; j++) {
                  currentX += 1;
                  Point current = new Point(currentX, currentY);
                  if (!visited.contains(current)) {
                    visited.add(current);
                  } else {
                    System.out.println(i + " " + current.toString());
                    break;
                  }
                }
                break;
              case WEST:
                currentDirection = Direction.SOUTH;
                for (int j = 0; j < distance; j++) {
                  currentY -= 1;
                  Point current = new Point(currentX, currentY);
                  if (!visited.contains(current)) {
                    visited.add(current);
                  } else {
                    System.out.println(i + " " + current.toString());
                    break;
                  }
                }
                break;
            }
            break;
        }
      }
    } catch (Exception e) {
      System.out.println(e);
    }
    var diffSimple = Math.abs(currentX) + Math.abs(currentY);
    System.out.println("X: " + currentX + " Y: " + currentY + " Simple Distance:" + diffSimple);

  }

}
