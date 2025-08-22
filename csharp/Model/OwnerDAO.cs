using System;
using Cassandra;
using Cassandra.Mapping;

namespace CarePet.Model
{
    public class OwnerDAO
    {
        private readonly IMapper _mapper;

        public OwnerDAO(ISession session)
        {
            _mapper = new Cassandra.Mapping.Mapper(session);
        }

        public void Create(Owner owner)
        {
            _mapper.Insert(owner);
        }

        public void Update(Owner owner)
        {
            _mapper.Update(owner);
        }

        public Owner Get(Guid id)
        {
            return _mapper.FirstOrDefault<Owner>("WHERE owner_id = ?", id);
        }
    }
}
