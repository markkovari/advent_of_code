package mark.kovari.aoc;

import utils.instructions.Instruction;
import utils.instructions.InstructionFactory;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.IntStream;

public class TwentyFifth {

    private List<String> instructions;

    public TwentyFifth() {
        File f = new File("inputs/25/prod.data");
        instructions = inputToList(f);
    }

    private static List<String> inputToList(File input) {
        List<String> list = new ArrayList<>();

        try {
            BufferedReader br = new BufferedReader(new FileReader(input));
            String line = "";

            while ((line = br.readLine()) != null) {
                list.add(line);
            }

            br.close();
        } catch (IOException e) {
            e.printStackTrace();
        }

        return list;
    }

    public int run() {
        int a = 1;
        while (!isValid(a)) {
            a++;
        }

        return a;
    }

    private boolean isValid(int a) {
        int[] registers = IntStream.of(0, 0, 0, 0).toArray();
        registers[0] = a;

        int index = 0;
        int outNum = 0;
        int outIncr = 0;
        while (index < instructions.size() && index >= 0) {
            Instruction instruction = InstructionFactory.getInstruction(instructions.get(index), registers, index,
                    instructions);
            int out = instruction.execute();

            if (instructions.get(index).contains("out")) {
                outNum = outNum | (out << outIncr++);
            }

            if (outIncr == 8) {
                if (0b10101010 == outNum) {
                    return true;
                } else {
                    return false;
                }
            }

            index = instruction.index();

        }
        return false;
    }

}