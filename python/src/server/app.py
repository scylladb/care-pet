from fastapi import FastAPI
from db.client import ScyllaClient
from db import config

app = FastAPI()
db_config = vars(config.argument_parser().parse_args())
db = ScyllaClient(db_config)

@app.get("/")
async def root():
    return {"message": "Pet collar simulator API"}


@app.get("/api/owner/{owner_id}")
async def owner(owner_id):
    return db.fetch_owner(owner_id).one()


@app.get("/api/owner/{owner_id}/pets")
async def pets(owner_id):
    return db.fetch_pets(owner_id).all()


@app.get("/api/pet/{pet_id}/sensors")
async def sensors(pet_id):
    return db.fetch_sensors(pet_id).all()
