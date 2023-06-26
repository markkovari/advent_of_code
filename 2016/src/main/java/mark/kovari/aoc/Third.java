package mark.kovari.aoc;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

public class Third {
  private record Triangle(int x, int y, int z) {
    public boolean valid() {
      return x + y > z && x + z > y && y + z > x;
    }
  }

  private Triangle fromString(String line) {
    String[] split = line.split("\\s+");
    try {
      return new Triangle(Integer.parseInt(split[0]), Integer.parseInt(split[1]), Integer.parseInt(split[2]));
    } catch (NumberFormatException e) {
      return null;
    }
  }

  private List<Triangle> columned(List<Triangle> originals) {
    List<Triangle> columned = new ArrayList<Triangle>();
    for (int row = 0; row < originals.size(); row += 3) {
      columned.add(new Triangle(originals.get(row).x(), originals.get(row + 1).x(), originals.get(row + 2).x()));
      columned.add(new Triangle(originals.get(row).y(), originals.get(row + 1).y(), originals.get(row + 2).y()));
      columned.add(new Triangle(originals.get(row).z(), originals.get(row + 1).z(), originals.get(row + 2).z()));
    }
    return columned;
  }

  public void solve() {
    try {
      List<Triangle> triangles = new ArrayList<Triangle>();
      String[] content = Files.readString(Paths.get("./inputs/3/prod.data")).split(System.lineSeparator());
      for (String line : content) {
        Triangle triangle = fromString(line.trim());
        if (triangle != null) {
          triangles.add(triangle);
        }
      }
      System.out.println("First result " + triangles.stream().filter(Triangle::valid).count());
      System.out.println("Second result " + columned(triangles).stream().filter(Triangle::valid).count());
    } catch (IOException e) {
      e.printStackTrace();
    }
  }
}
