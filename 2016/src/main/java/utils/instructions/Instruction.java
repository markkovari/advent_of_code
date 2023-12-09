package utils.instructions;

public interface Instruction {
    public int execute();

    public int index();

    default int indexOf(char register) {
        return register - 97;
    }
}