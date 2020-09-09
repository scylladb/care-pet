package com.carepet.model;

import com.datastax.oss.driver.api.mapper.annotations.ClusteringColumn;
import com.datastax.oss.driver.api.mapper.annotations.CqlName;
import com.datastax.oss.driver.api.mapper.annotations.Entity;
import com.datastax.oss.driver.api.mapper.annotations.PartitionKey;
import org.apache.commons.lang.RandomStringUtils;
import org.apache.commons.lang.math.RandomUtils;

import java.util.UUID;

@Entity
@CqlName("pet")
public class Pet {
    @PartitionKey
    private UUID ownerId;

    @ClusteringColumn
    private UUID petId;

    private String chipId ;

    private String species ;

    private String breed   ;

    private String color   ;

    private String gender  ;

    private int age;

    private float weight;

    private String address;

    private String name;

    public Pet() {}

    public Pet(UUID ownerId, UUID petId, String chipId, String species, String breed, String color, String gender, int age, float weight, String address, String name) {
        this.ownerId = ownerId;
        this.petId = petId;
        this.chipId = chipId;
        this.species = species;
        this.breed = breed;
        this.color = color;
        this.gender = gender;
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

    public String getChipId() {
        return chipId;
    }

    public void setChipId(String chipId) {
        this.chipId = chipId;
    }

    public String getSpecies() {
        return species;
    }

    public void setSpecies(String species) {
        this.species = species;
    }

    public String getBreed() {
        return breed;
    }

    public void setBreed(String breed) {
        this.breed = breed;
    }

    public String getColor() {
        return color;
    }

    public void setColor(String color) {
        this.color = color;
    }

    public String getGender() {
        return gender;
    }

    public void setGender(String gender) {
        this.gender = gender;
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

    public static Pet random() {
        return new Pet(
                UUID.randomUUID(),
                UUID.randomUUID(),
                "", "", "", "", "",
                1 + RandomUtils.nextInt(100),
                5.0f + 10.0f * RandomUtils.nextFloat(),
                "home",
                RandomStringUtils.randomAlphanumeric(8));
    }

    @Override
    public String toString() {
        return "Pet{" +
                "ownerId=" + ownerId +
                ", petId=" + petId +
                ", chipId='" + chipId + '\'' +
                ", species='" + species + '\'' +
                ", breed='" + breed + '\'' +
                ", color='" + color + '\'' +
                ", gender='" + gender + '\'' +
                ", age=" + age +
                ", weight=" + weight +
                ", address='" + address + '\'' +
                ", name='" + name + '\'' +
                '}';
    }
}
