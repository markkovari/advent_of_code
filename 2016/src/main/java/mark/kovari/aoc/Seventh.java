package mark.kovari.aoc;


import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedList;
import java.util.List;
import java.util.regex.MatchResult;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Seventh {

    private static final Pattern ABBA_PATTERN = Pattern.compile("([a-z])([a-z])\\2\\1", Pattern.CASE_INSENSITIVE);
    private static final Pattern ABA_PATTERN = Pattern.compile("([a-z])([a-z])\\1", Pattern.CASE_INSENSITIVE);

    private static final Pattern HYPERNET_SEQUENCE_PATTERN = Pattern.compile("\\[([a-z]*?)\\]", Pattern.CASE_INSENSITIVE);

    public void solve() {
        try {
            String[] lineStream = Files.readString(Paths.get("./inputs/7/prod.data")).split(System.lineSeparator());
            List<IPv7> IPRecords = Arrays.stream(lineStream).map(IPv7::fromString).toList();
            List<IPv7> validIPRecordsTSL = new ArrayList<>();
            List<IPv7> validIPRecordsSSL = new ArrayList<>();
            for (IPv7 IPRecord : IPRecords) {
                if (IPRecord.isSupportingTLS()) {
                    validIPRecordsTSL.add(IPRecord);
                }
                if (IPRecord.isSupportingSSL()) {
                    validIPRecordsSSL.add(IPRecord);
                }
            }
            System.out.printf("Valid TLS IPv7 address count %d\n", validIPRecordsTSL.size());
            System.out.printf("Valid SSL IPv7 address count %d\n", validIPRecordsSSL.size());
        } catch (Exception e) {
            System.err.println("lol");
        }
    }

    private record IPv7(String raw, String prefix, String middle, String suffix) {

        public static IPv7 fromString(final String from) {
            String[] parts = from.split("[\\[\\]]");
            return new IPv7(from, parts[0], parts[1], parts[2]);
        }

        public boolean isSupportingTLS() {
            List<MatchResult> hypernetSequenceMatchResults = findMatchResultHypernetSequence();
            List<MatchResult> abbaMatchResults = findMatchResultForABBA();

            for (MatchResult abbaMatchResult : abbaMatchResults) {
                if (isEnclosedByHypernetSequence(abbaMatchResult, hypernetSequenceMatchResults)) {
                    return false;
                }
            }
            return abbaMatchResults.size() > 0;
        }

        private boolean isValidABBA(MatchResult abbaMatchResult) {
            return !abbaMatchResult.group(1).equalsIgnoreCase(abbaMatchResult.group(2));
        }

        private List<MatchResult> findMatchResultHypernetSequence() {
            List<MatchResult> hypernetSequenceMatchResults = new LinkedList<>();
            Matcher hypernetSequenceMatcher = HYPERNET_SEQUENCE_PATTERN.matcher(this.raw);
            while (hypernetSequenceMatcher.find()) {
                hypernetSequenceMatchResults.add(hypernetSequenceMatcher.toMatchResult());
            }
            return hypernetSequenceMatchResults;
        }

        private List<MatchResult> findMatchResultForABA() {
            List<MatchResult> abbaMatchResults = new LinkedList<>();
            Matcher abaMatcher = ABA_PATTERN.matcher(this.raw);

            return getMatchResults(abbaMatchResults, abaMatcher);
        }

        private List<MatchResult> getMatchResults(List<MatchResult> abbaMatchResults, Matcher abaMatcher) {
            for (int i = 0; i < this.raw.length() - 2; i++) {
                if (abaMatcher.region(i, i + 3).matches()) {
                    MatchResult abaMatchResult = abaMatcher.toMatchResult();
                    if (isValidABA(abaMatchResult)) {
                        abbaMatchResults.add(abaMatchResult);
                    }
                }
            }
            return abbaMatchResults;
        }

        private boolean isValidABA(MatchResult abaMatchResult) {
            return !abaMatchResult.group(1).equalsIgnoreCase(abaMatchResult.group(2));
        }

        private List<MatchResult> findMatchResultForBAB(MatchResult abaMatchResult) {
            List<MatchResult> babMatchResults = new LinkedList<>();

            String a = abaMatchResult.group(1);
            String b = abaMatchResult.group(2);
            Pattern babPattern = Pattern.compile(String.format("(%s)(%s)(%s)", b, a, b), Pattern.CASE_INSENSITIVE);

            Matcher babMatcher = babPattern.matcher(this.raw);
            return getMatchResults(babMatchResults, babMatcher);
        }

        public boolean isSupportingSSL() {
            List<MatchResult> hypernetSequenceMatchResults = findMatchResultHypernetSequence();
            List<MatchResult> abaMatchResults = findMatchResultForABA();

            for (MatchResult abaMatchResult : abaMatchResults) {
                if (isEnclosedByHypernetSequence(abaMatchResult, hypernetSequenceMatchResults)) {
                    continue;
                }

                List<MatchResult> babMatchResults = findMatchResultForBAB(abaMatchResult);
                for (MatchResult babMatchResult : babMatchResults) {
                    if (isEnclosedByHypernetSequence(babMatchResult, hypernetSequenceMatchResults)) {
                        return true;
                    }
                }
            }

            return false;
        }

        private boolean isEnclosedByHypernetSequence(MatchResult matchResult, List<MatchResult> hypernetSequenceMatchResults) {
            for (MatchResult hypernetSequenceMatchResult : hypernetSequenceMatchResults) {
                int startInterval = hypernetSequenceMatchResult.start();
                int endInterval = hypernetSequenceMatchResult.end();
                if (startInterval <= matchResult.start() && matchResult.end() <= endInterval) {
                    return true;
                }
            }
            return false;
        }

        private List<MatchResult> findMatchResultForABBA() {
            List<MatchResult> abbaMatchResults = new LinkedList<>();
            Matcher abbaMatcher = ABBA_PATTERN.matcher(this.raw);
            while (abbaMatcher.find()) {
                MatchResult abbaMatchResult = abbaMatcher.toMatchResult();
                if (isValidABBA(abbaMatchResult)) {
                    abbaMatchResults.add(abbaMatchResult);
                }
            }
            return abbaMatchResults;
        }

    }
}