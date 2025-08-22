using System;
using System.Collections.Generic;
using Cassandra;
using Cassandra.Mapping;

namespace CarePet.Model
{
    public class PetDAO
    {
        private readonly IMapper _mapper;

        public PetDAO(ISession session)
        {
            _mapper = new Cassandra.Mapping.Mapper(session);
        }

        public void Create(Pet pet)
        {
            _mapper.Insert(pet);
        }

        public void Update(Pet pet)
        {
            _mapper.Update(pet);
        }

        public Pet Get(Guid ownerId, Guid petId)
        {
            return _mapper.FirstOrDefault<Pet>(
                "WHERE owner_id = ? AND pet_id = ?",
                ownerId,
                petId
            );
        }

        public IEnumerable<Pet> FindByOwner(Guid ownerId)
        {
            return _mapper.Fetch<Pet>(
                "WHERE owner_id = ?",
                ownerId
            );
        }
    }
}
