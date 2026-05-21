"""
Simple dog name generator replacing the unavailable `dognames` PyPI package.
Provides male() and female() functions that return a random dog name.
"""
import random

_MALE_NAMES = [
    "Ace", "Apollo", "Archie", "Atlas", "Axel", "Bailey", "Bear", "Beau",
    "Bentley", "Blue", "Bo", "Boomer", "Bruno", "Brute", "Brutus", "Buck",
    "Buddy", "Buster", "Caesar", "Cash", "Charlie", "Chase", "Chester",
    "Chief", "Chip", "Cody", "Cooper", "Copper", "Cosmo", "Diesel", "Duke",
    "Elvis", "Finn", "Frank", "Gage", "Goliath", "Gus", "Hank", "Harley",
    "Henry", "Hugo", "Hunter", "Jack", "Jax", "Joey", "Koda", "Leo", "Loki",
    "Lucky", "Mac", "Max", "Maverick", "Miles", "Moose", "Murphy", "Oscar",
    "Otis", "Rex", "Riley", "Rocky", "Roscoe", "Rufus", "Rusty", "Sam",
    "Scout", "Shadow", "Simba", "Spike", "Tank", "Thor", "Toby", "Tucker",
    "Tyson", "Zeus",
]

_FEMALE_NAMES = [
    "Abby", "Angel", "Athena", "Aurora", "Autumn", "Ava", "Babette", "Bailey",
    "Bella", "Bessie", "Birdie", "Blondie", "Bonnie", "Brandy", "Brie",
    "Brownie", "Candy", "Carla", "Chelsea", "Chloe", "Cleo", "Coco", "Cookie",
    "Daisy", "Dakota", "Darla", "Diva", "Dolly", "Elsie", "Emma", "Fiona",
    "Ginger", "Gracie", "Greta", "Harley", "Hazel", "Holly", "Honey", "Jade",
    "Jasmine", "Jessie", "Lady", "Lexi", "Lily", "Lola", "Lucy", "Luna",
    "Lulu", "Macy", "Maggie", "Maple", "Maya", "Mia", "Millie", "Minnie",
    "Missy", "Molly", "Nala", "Nora", "Olivia", "Peanut", "Pebbles", "Penny",
    "Pepper", "Piper", "Pixie", "Princess", "Rosie", "Roxie", "Ruby", "Sadie",
    "Sandy", "Sasha", "Stella", "Sugar", "Trixie", "Violet", "Willow", "Zara",
    "Zoe",
]


def male():
    """Return a random male dog name."""
    return random.choice(_MALE_NAMES)


def female():
    """Return a random female dog name."""
    return random.choice(_FEMALE_NAMES)
