package src.mark.kovari.adventofcode;

import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Fifth {

  private boolean valid(String hash) {
    return hash.startsWith("00000") && Character.isDigit(hash.charAt(5)) && hash.charAt(5) < '8';
  }

  public String replaceCharAt(String str, int index, char replace) {
    if (str == null) {
      return str;
    } else if (index < 0 || index >= str.length()) {
      return str;
    }
    char[] chars = str.toCharArray();
    chars[index] = replace;
    return String.valueOf(chars);
  }

  public String getHash(String start, Long value) throws NoSuchAlgorithmException {
    MessageDigest md5Hasher = MessageDigest.getInstance("MD5");
    md5Hasher.update((start + value.toString()).getBytes());
    byte[] digest = md5Hasher.digest();
    StringBuilder builder = new StringBuilder();
    for (byte b : digest) {
      builder.append(String.format("%02x", b));
    }
    return builder.toString();
  }

  public void solve() {
    List<String> valid = new ArrayList<String>() {
    };
    String start = "wtnhxymk";
    // String start = "asd";
    try {

      long i = 0;
      var pwd1 = new StringBuilder();
      var pwd2 = new char[] { ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ' };
      int pwd2Cnt = 0;
      while (pwd2Cnt < 8) {
        var hash = getHash(start, i).toString();
        if (hash.startsWith("00000")) {
          char a = hash.charAt(5);
          char b = hash.charAt(6);
          if (pwd1.length() < 8) {
            pwd1.append(a);
          }
          if (a >= '0' && a < '8' && pwd2[a - '0'] == ' ') {
            pwd2[a - '0'] = b;
            pwd2Cnt++;
          }
        }
        i++;
      }

      System.out.println("Part 1: " + pwd1);
      System.out.println("Part 2: " + new String(pwd2));

    } catch (

    Exception e) {
      e.printStackTrace();
    }
  }
}
