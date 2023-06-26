package mark.kovari.aoc;

import com.google.common.hash.Hashing;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import java.util.stream.IntStream;

public class Fourteenth {

    private static Optional<Character> getTripletChar(String s) {
        return IntStream.range(0, s.length() - 2)
                .filter(i -> s.charAt(i) == s.charAt(i + 1) && s.charAt(i) == s.charAt(i + 2))
                .mapToObj(s::charAt)
                .findAny();
    }

    private static boolean hasFiveCharInARow(String s, char ch) {
        return IntStream.range(0, s.length() - 4)
                .anyMatch(i -> s.charAt(i) == ch
                        && s.charAt(i + 1) == ch
                        && s.charAt(i + 2) == ch
                        && s.charAt(i + 3) == ch
                        && s.charAt(i + 4) == ch);
    }

    private static String getHash(List<String> hashes, String salt, int index, boolean stretched) {
        if (index == hashes.size()) {
            hashes.add(getHash(salt + index, stretched));
        }
        return hashes.get(index);
    }

    private static String getHash(String s, boolean stretched) {
        var h = s;
        for (int i = 0, cnt = stretched ? 2017 : 1; i < cnt; i++) {
            h = getMd5Hash(h);
        }
        return h;
    }

    @SuppressWarnings({"deprecated", "UnstableApiUsage"})
    private static String getMd5Hash(String s) {
        return Hashing.md5().hashString(s, StandardCharsets.UTF_8).toString();
    }


    private static int solve(String salt, boolean stretched) {
        var hashes = new ArrayList<String>();
        var keyIndices = new ArrayList<Integer>();
        for (int i = 0; keyIndices.size() < 64; i++) {
            var h1 = getHash(hashes, salt, i, stretched);
            var ch = getTripletChar(h1);
            if (ch.isPresent()) {
                boolean isKey = IntStream.rangeClosed(i + 1, i + 1000)
                        .anyMatch(j -> hasFiveCharInARow(getHash(hashes, salt, j, stretched), ch.get()));
                if (isKey) {
                    keyIndices.add(i);
                }
            }
        }

        return keyIndices.get(keyIndices.size() - 1);
    }

    public void solve() {
        try {
            String salt = Files.readString(Paths.get("./inputs/14/prod.data"));

            System.out.println("Fourteenth: Part 1: " + solve(salt, false));
            System.out.println("Fourteenth: Part 2: " + solve(salt, true));
        } catch (IOException e) {
            e.printStackTrace();
        }

    }
}
