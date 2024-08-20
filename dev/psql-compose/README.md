This folder contains the files required to get a test data base up and running with podman

Run the following command from this folder:
```sh
podman compose up -d
```

To bring it down:
```sh
podman compose down
```

The file `init.sql` should contain the schema and test data, it is run automatically when starting the postgres container.
There are no volumes created, so all data is lost when the containers are destroyed.
