package sgf;

import java.util.function.Function;

public final class Util
{


    public static <T, R, E extends Exception> Function<T, R> errorWrapper(FunctionWithException<T, R, E> fe)
    {
        return arg -> {
            try {
                return fe.apply(arg);
            } catch(Exception e) {
                throw new RuntimeException(e);
            }
        };
    }
}
