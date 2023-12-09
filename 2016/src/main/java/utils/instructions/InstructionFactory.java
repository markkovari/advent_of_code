package utils.instructions;


import java.util.List;

public class InstructionFactory {

    public static Instruction getInstruction(String instructionStr, int[] registers, int index,
                                             List<String> instructions) {
        String[] instructionParams = instructionStr.split(" ");
        String instructionName = instructionParams[0];

        return switch (instructionName) {
            case "cpy" ->
                    instructionParams.length == 3 ? new Cpy(registers, index, instructionParams[1], instructionParams[2])
                            : new Skp(index);
            case "inc" ->
                    instructionParams.length == 2 ? new Inc(registers, index, instructionParams[1]) : new Skp(index);
            case "dec" ->
                    instructionParams.length == 2 ? new Dec(registers, index, instructionParams[1]) : new Skp(index);
            case "jnz" ->
                    instructionParams.length == 3 ? new Jnz(registers, index, instructionParams[1], instructionParams[2]) : new Skp(index);
            case "tgl" ->
                    instructionParams.length == 2 ? new Tgl(index, instructionParams[1], instructions, registers) : new Skp(index);
            case "out" -> new Out(registers, index, instructionParams[1]);
            default -> throw new IllegalArgumentException(instructionName + " is not mapped to an instruction");
        };
    }

}