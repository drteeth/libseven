package com.sevenshifts.libsevenclient;

public class RustGreeting {
    static {
        System.loadLibrary("seven");
    }
    private static native String greeting(final String pattern);

    public String sayHello(String to) {
        return greeting(to);
    }
}