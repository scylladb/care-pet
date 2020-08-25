package com.carepet.util;

@FunctionalInterface
public interface SupplierWithException<R, E extends Exception> {
    R apply() throws E;
}
