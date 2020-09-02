package com.carepet.model;

public enum SensorType {
    Temperature("T"),
    Pulse("P"),
    Location("L"),
    Respiration("R");

    private final String type;

    SensorType(String type) {
        this.type = type;
    }

    public String getType() {
        return type;
    }
}
