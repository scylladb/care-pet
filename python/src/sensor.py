import time
from datetime import datetime

import uuid
import random
import dognames
import names
import argparse
from db import config
from db.client import ScyllaClient


def generate_random_pet_name():
    return dognames.male() if random.randint(1, 2)==1 else dognames.female()


def generate_random_measurement_value(sensor_type):
    if sensor_type == "T":
        return 101 + random.uniform(0, 10) - 4;
    elif sensor_type == "L":
      return 35 + random.uniform(0, 5) - 2
    return random.uniform(0, 20)


def generate_random_measurement(sensor):
    return {
        "sensor_id": sensor["sensor_id"],
        "ts": datetime.now(),
        "value": generate_random_measurement_value(sensor["type"])
    }
  
  
def create_owner():
    return {
        "owner_id": uuid.uuid4(),
        "address": "home",
        "name": names.get_full_name() # generate random person's name
    }


def create_pet(owner):
    return {
        "owner_id": owner["owner_id"],
        "pet_id": uuid.uuid4(),
        "address": "home",
        "age": random.randint(1, 20),
        "name": generate_random_pet_name(),
        "weight": random.uniform(5.0, 15.0)
    }


def create_sensors(pet):
    return [
        {
            "pet_id": pet["pet_id"],
            "sensor_id": uuid.uuid4(),
            "type": "T"
        },
        {
            "pet_id": pet["pet_id"],
            "sensor_id": uuid.uuid4(),
            "type": "L"
        }
    ]

def insert_pet_static_data(client: ScyllaClient, pet, owner, sensors):
    client.insert_data(table="carepet.pet", data_dict=pet)
    client.insert_data(table="carepet.owner", data_dict=owner)
    for sensor in sensors:
        client.insert_data(table="carepet.sensor", data_dict=sensor)
    
    
def generate_measurements(sensors, count=2, delay=1):
    measurements_list = []
    for i in range(0, int(count)):
        for sensor in sensors:
            measurement = generate_random_measurement(sensor)
            measurements_list.append(measurement)
            
            print(f"sensor # {sensor['sensor_id']} type {sensor['type']},",
                  f"new measure: {measurement['value']},",
                  f"ts: {measurement['ts']}")
        time.sleep(delay)
    return measurements_list
    
    
def init_args_parser():
    arg_parser = config.argument_parser()
    arg_parser.add_argument("-b", "--buffer-interval",
                        help="Sensors measurement interval (seconds)",
                        required=False, default=10)
    arg_parser.add_argument("-m", "--measure",
                        help="Buffer to accumulate measures (seconds)",
                        required=False, default=1)
    return vars(arg_parser.parse_args())


def main():
    # parse config arguments from command line input
    config = init_args_parser()
    
    # both arguments are defined in seconds
    measure_sec = int(config["measure"])
    buffer_interval_sec = int(config["buffer_interval"])
    
    if measure_sec > buffer_interval_sec:
        raise ValueError("`--measure` cannot be larger than `--buffer-interval`")
    
    # calculate batch size based on the `buffer_interval` argument
    batch_size = buffer_interval_sec / measure_sec
    
    # create random static data
    new_owner = create_owner()
    new_pet = create_pet(owner=new_owner)
    new_sensors = create_sensors(pet=new_pet)
    
    print("Welcome to the Pet collar simulator")
    print(f"New owner # {new_owner['owner_id']}")
    print(f"New pet # {new_pet['pet_id']}")
    for i in range(0, len(new_sensors)):
        print(f"New sensor({i}) # {new_sensors[i]['sensor_id']}")
    
    with ScyllaClient(config) as client:
        insert_pet_static_data(client, new_pet, new_owner, new_sensors)
        # infinite loop to insert random time-series data into the measurement table
        while (True):
            measurements_data = generate_measurements(new_sensors,
                                                      count=batch_size,
                                                      delay=measure_sec)
            print("Pushing data")
            client.insert_batch_data("carepet.measurement", measurements_data)
        
main()






