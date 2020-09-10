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

    public static SensorType fromString(String text) {
        for (SensorType t : SensorType.values()) {
            if (t.type.equalsIgnoreCase(text)) {
                return t;
            }
        }

        throw new IllegalArgumentException();
    }

    public String getType() {
        return type;
    }
}
