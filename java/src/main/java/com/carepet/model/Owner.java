package com.carepet.model;

import com.datastax.oss.driver.api.mapper.annotations.CqlName;
import com.datastax.oss.driver.api.mapper.annotations.Entity;
import com.datastax.oss.driver.api.mapper.annotations.PartitionKey;

import java.util.UUID;

@Entity
@CqlName("owner")
public class Owner {
    @PartitionKey
    private UUID ownerId;

    private String name;

    private String address;

    public Owner(UUID ownerId, String name, String address) {
        this.ownerId = ownerId;
        this.name = name;
        this.address = address;
    }

    public Owner() {}

    public UUID getOwnerId() {
        return ownerId;
    }

    public void setOwnerId(UUID ownerId) {
        this.ownerId = ownerId;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public String getAddress() {
        return address;
    }

    public void setAddress(String address) {
        this.address = address;
    }

    @Override
    public String toString() {
        return "Owner{" +
                "ownerId=" + ownerId +
                ", name='" + name + '\'' +
                ", address='" + address + '\'' +
                '}';
    }
}
