import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class main() {

    interface StringFunction {
        String run(String str);
    }

    public static void main() {

        List<Integer> numbers = Arrays.asList(1, 2, 3, 4);

        numbers.stream().filter(n -> n == 0);



    }



}