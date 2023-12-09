package utils.instructions;

public class Dec implements Instruction {
    private final int index;
    private final int[] registers;
    private final String paramA;

    public Dec(int[] registers, int index, String paramA) {
        this.registers = registers;
        this.index = index;
        this.paramA = paramA;
    }

    @Override
    public int execute() {
        registers[indexOf(paramA.charAt(0))]--;
        return 0;
    }

    @Override
    public int index() {
        return index + 1;
    }

}
