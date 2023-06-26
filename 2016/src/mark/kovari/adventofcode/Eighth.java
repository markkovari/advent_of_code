package src.mark.kovari.adventofcode;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

public class Eighth {


    private static final int ROW_COUNT = 6;
    private static final int COL_COUNT = 50;


    private static void shiftRow(char[][] display, int row, int shift) {
        for (int k = 0; k < shift; k++) {
            char last = display[row][COL_COUNT - 1];
            for (int j = COL_COUNT - 1; j > 0; j--) {
                display[row][j] = display[row][j - 1];
            }
            display[row][0] = last;
        }
    }

    private static void shiftCol(char[][] display, int col, int shift) {
        for (int k = 0; k < shift; k++) {
            char last = display[ROW_COUNT - 1][col];
            for (int i = ROW_COUNT - 1; i > 0; i--) {
                display[i][col] = display[i - 1][col];
            }
            display[0][col] = last;
        }
    }

    private static void fillRect(char[][] display, int row, int col, char ch) {
        for (int i = 0; i < row; i++) {
            for (int j = 0; j < col; j++) {
                display[i][j] = ch;
            }
        }
    }

    private static int getPixelCount(char[][] display) {
        int cnt = 0;
        for (int i = 0; i < ROW_COUNT; i++) {
            for (int j = 0; j < COL_COUNT; j++) {
                if (display[i][j] == '#') {
                    cnt++;
                }
            }
        }
        return cnt;
    }

    public void solve() {

        try {
            char[][] display = new char[ROW_COUNT][COL_COUNT];
            fillRect(display, ROW_COUNT, COL_COUNT, '.');

            String[] content = Files.readString(Paths.get("./inputs/8/prod.data")).split(System.lineSeparator());

            for (var line : content) {
                if (line.startsWith("rect")) {
                    var parts = line.substring(5).split("x");
                    fillRect(display, Integer.parseInt(parts[1]), Integer.parseInt(parts[0]), '#');
                } else if (line.startsWith("rotate column")) {
                    var parts = line.split("=")[1].split(" by ");
                    shiftCol(display, Integer.parseInt(parts[0]), Integer.parseInt(parts[1]));
                } else if (line.startsWith("rotate row")) {
                    var parts = line.split("=")[1].split(" by ");
                    shiftRow(display, Integer.parseInt(parts[0]), Integer.parseInt(parts[1]));
                }
            }
            for (int i = 0; i < ROW_COUNT; i++) {
                for (int j = 0; j < COL_COUNT; j++) {
                    System.out.print(display[i][j] + " ");
                }
                System.out.println();
            }
            System.out.println("Eighth day: first result " + getPixelCount(display));

            System.out.println(
                    "Sixth day: second result ");
        } catch (IOException e) {
            e.printStackTrace();
        }

    }
}
