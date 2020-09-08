package model

type Pet struct {
	OwnerID UUID
	PetID   UUID
	ChipID  string
	Species string
	Breed   string
	Color   string
	Gender  string
	Age     int
	Weight  float32
	Address string
	Name    string
}
