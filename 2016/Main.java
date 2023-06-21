import java.nio.file.Files;
import java.nio.file.Paths;

enum Direction {
  NORTH,
  EAST,
  SOUTH,
  WEST
}

class Main {
  public static void main(String... args) {

    var currentDirection = Direction.NORTH;
    var currentX = 0;
    var currentY = 0;

    try {
      String content = Files.readString(Paths.get("./inputs/1/prod.data"));
      for (String line : content.split(", ")) {
        var direction = line.charAt(0);
        var distance = Integer.parseInt(line.substring(1));
        switch (direction) {
          case 'R':
            switch (currentDirection) {
              case NORTH:
                currentDirection = Direction.EAST;
                currentX += distance;
                break;
              case EAST:
                currentDirection = Direction.SOUTH;
                currentY -= distance;
                break;
              case SOUTH:
                currentDirection = Direction.WEST;
                currentX -= distance;
                break;
              case WEST:
                currentDirection = Direction.NORTH;
                currentY += distance;
                break;
            }
            break;
          case 'L':
            switch (currentDirection) {
              case NORTH:
                currentDirection = Direction.WEST;
                currentX -= distance;
                break;
              case EAST:
                currentDirection = Direction.NORTH;
                currentY += distance;
                break;
              case SOUTH:
                currentDirection = Direction.EAST;
                currentX += distance;
                break;
              case WEST:
                currentDirection = Direction.SOUTH;
                currentY -= distance;
                break;
            }
            break;
        }
      }
    } catch (Exception e) {
      System.out.println(e);
    }
    var diffSimple = Math.abs(currentX) + Math.abs(currentY);
    System.out.println("X: " + currentX + " Y: " + currentY + " Simple Distance: " + diffSimple);
  }
}
