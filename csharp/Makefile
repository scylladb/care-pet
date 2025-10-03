check:
	dotnet format --verify-no-changes --severity warn --verbosity diagnostic CarePet.csproj & \
	dotnet format --verify-no-changes --severity warn --verbosity diagnostic CarePet.Migrate.csproj & \
	dotnet format --verify-no-changes --severity warn --verbosity diagnostic CarePet.Sensor.csproj & \
	wait

fix:
	dotnet format --severity warn --verbosity diagnostic CarePet.csproj & \
	dotnet format --severity warn --verbosity diagnostic CarePet.Migrate.csproj & \
	dotnet format --severity warn --verbosity diagnostic CarePet.Sensor.csproj & \
	wait

test-unit:
	dotnet test CarePet.csproj
	dotnet test CarePet.Migrate.csproj
	dotnet test CarePet.Sensor.csproj

clean:
	find . -name '*.csproj' -print0 | xargs -0 -n1 dotnet clean
