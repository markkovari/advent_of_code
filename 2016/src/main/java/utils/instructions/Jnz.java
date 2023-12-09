package utils.instructions;

public class Jnz implements Instruction {
    private final int[] registers;
    private final String paramA;
    private final String paramB;
    private int index;

    public Jnz(int[] registers, int index, String paramA, String paramB) {
        this.index = index;
        this.registers = registers;
        this.paramA = paramA;
        this.paramB = paramB;
    }

    @Override
    public int execute() {
        int cmp = paramA.matches("-?\\d+") ? Integer.parseInt(paramA) : registers[indexOf(paramA.charAt(0))];
        if (cmp != 0) {
            index += paramB.matches("-?\\d+") ? Integer.parseInt(paramB) : registers[indexOf(paramB.charAt(0))];
        } else {
            index++;
        }
        return 0;
    }

    @Override
    public int index() {
        return index;
    }

}
