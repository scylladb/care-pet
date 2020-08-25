package com.carepet.util;

import java.util.function.Function;
import java.util.function.Supplier;

public class Wrapper {
    public static <R, E extends Exception>
    Supplier<R> unwrap0(SupplierWithException<R, E> f) {
        return () -> {
            try {
                return f.apply();
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        };
    }

    public static <T, R, E extends Exception>
    Function<T, R> unwrap(FunctionWithException<T, R, E> f) {
        return arg -> {
            try {
                return f.apply(arg);
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        };
    }
}
