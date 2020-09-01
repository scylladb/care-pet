package com.carepet.model;

import com.datastax.oss.driver.api.mapper.annotations.ClusteringColumn;
import com.datastax.oss.driver.api.mapper.annotations.CqlName;
import com.datastax.oss.driver.api.mapper.annotations.Entity;
import com.datastax.oss.driver.api.mapper.annotations.PartitionKey;

import java.util.UUID;

@Entity
@CqlName("pet")
public class Pet {
    @PartitionKey
    private UUID ownerId;

    @ClusteringColumn
    private UUID petId;

    private int age;

    private float weight;

    private String address;

    private String name;

    public Pet() {}

    public Pet(UUID ownerId, UUID petId, int age, float weight, String address, String name) {
        this.ownerId = ownerId;
        this.petId = petId;
        this.age = age;
        this.weight = weight;
        this.address = address;
        this.name = name;
    }

    public UUID getOwnerId() {
        return ownerId;
    }

    public void setOwnerId(UUID ownerId) {
        this.ownerId = ownerId;
    }

    public UUID getPetId() {
        return petId;
    }

    public void setPetId(UUID petId) {
        this.petId = petId;
    }

    public int getAge() {
        return age;
    }

    public void setAge(int age) {
        this.age = age;
    }

    public float getWeight() {
        return weight;
    }

    public void setWeight(float weight) {
        this.weight = weight;
    }

    public String getAddress() {
        return address;
    }

    public void setAddress(String address) {
        this.address = address;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    @Override
    public String toString() {
        return "Pet{" +
                "ownerId=" + ownerId +
                ", petId=" + petId +
                ", age=" + age +
                ", weight=" + weight +
                ", address='" + address + '\'' +
                ", name='" + name + '\'' +
                '}';
    }
}
