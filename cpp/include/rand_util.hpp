#pragma once

#include "model.hpp"

#include <cassandra.h>
#include <random>
#include <string>

// ---------------------------------------------------------------------------
// Thread-local Mersenne-Twister RNG.
// ---------------------------------------------------------------------------
inline std::mt19937& rng() {
    static thread_local std::mt19937 g(std::random_device{}());
    return g;
}

inline int rand_int(int lo, int hi) {
    return std::uniform_int_distribution<int>(lo, hi)(rng());
}

inline float rand_float(float lo, float hi) {
    return std::uniform_real_distribution<float>(lo, hi)(rng());
}

// Random lowercase-alpha string of given length.
inline std::string rand_string(size_t len) {
    static constexpr char kAlpha[] = "abcdefghijklmnopqrstuvwxyz";
    std::uniform_int_distribution<size_t> d(0, 25);
    std::string s(len, ' ');
    for (auto& c : s) c = kAlpha[d(rng())];
    return s;
}

inline Owner rand_owner(CassUuidGen* gen) {
    Owner o;
    cass_uuid_gen_random(gen, &o.owner_id);
    o.name    = rand_string(8);
    o.address = rand_string(10) + " st";
    return o;
}

inline Pet rand_pet(CassUuidGen* gen, const Owner& owner) {
    static constexpr const char* kSpecies[] = {"Dog", "Cat", "Rabbit", "Hamster"};
    Pet p;
    p.owner_id = owner.owner_id;
    cass_uuid_gen_random(gen, &p.pet_id);
    p.chip_id  = rand_string(16);
    p.species  = kSpecies[rand_int(0, 3)];
    p.breed    = rand_string(8);
    p.color    = rand_string(6);
    p.gender   = (rand_int(0, 1) == 0) ? "M" : "F";
    p.age      = rand_int(1, 15);
    p.weight   = rand_float(1.0f, 50.0f);
    p.address  = owner.address;
    p.name     = rand_string(6);
    return p;
}

inline Sensor rand_sensor(CassUuidGen* gen, const Pet& pet) {
    static constexpr const char* kTypes[] = {"T", "P", "R", "L"};
    Sensor s;
    s.pet_id = pet.pet_id;
    cass_uuid_gen_random(gen, &s.sensor_id);
    s.type = kTypes[rand_int(0, 3)];
    return s;
}

// Generate a plausible sensor reading for the given sensor type:
//   T = temperature (°F)   98–107
//   P = pulse (bpm)        80–120
//   R = respiration/min    33–37
//   L = location/distance   0–10
inline float rand_sensor_data(const Sensor& s) {
    if (s.type == "T") return rand_float(98.0f, 107.0f);
    if (s.type == "P") return rand_float(80.0f, 120.0f);
    if (s.type == "R") return rand_float(33.0f,  37.0f);
    return rand_float(0.0f, 10.0f); // "L"
}
