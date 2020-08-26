package com.carepet.util;

@FunctionalInterface
public interface CallableWithException<E extends Exception> {
    void apply() throws E;
}
