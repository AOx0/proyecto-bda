# proyecto-bda 

<img width="1392" alt="Screenshot 2023-11-17 at 6 20 52 a m" src="https://github.com/AOx0/proyecto-bda/assets/50227494/78118ba4-6805-4025-a755-2c3cb54b0377">

## Dependencies

1. You must have the Rust Programming Language Stable Toolchain, read more [here](https://www.rust-lang.org/tools/install)
2. You may need to install `libmysqlclient` to run the migrations with diesel-cli, read more [here](https://diesel.rs/guides/getting-started#installing-diesel-cli) and [here](https://dev.mysql.com/doc/c-api/8.0/en/c-api-implementations.html)

## Clean data

1. Download de complete dataset [here](https://datos.cdmx.gob.mx/dataset/carpetas-de-investigacion-fgj-de-la-ciudad-de-mexico), there are multiple csv files per period, the one you want to download is `Carpetas de Investigación de la FGJ (Completa)`.
2. Run the program that cleans and splits the dataset into multiple csv files. Be aware, this program places all results at `./results/`, thus creates a directory at your current directory.

```
cargo run --package clean --release -- ./path/to/the/downloaded/dataset.csv    
```

## Create the database

1. Install the `diesel-cli`, you need to make sure you have the necessary [dependencies](https://diesel.rs/guides/getting-started#installing-diesel-cli). To install the cli program only for mariadb run:

```
cargo install diesel_cli --no-default-features --features mysql
```

2. Create a user named `bdavan` with password `1234567890` or change the environment variable `DATABASE_URL` at the `.env` file to suit your existing setup. Make sure the user has full permissions on `delitos.*`. There is an example [at the Arch wiki](https://wiki.archlinux.org/title/MariaDB#Add_user).

3. Create the database. This is an easy step since `diesel-cli` will read the SQL files located at `migrations/` and create the database, plus all tables. For this step to work diesel reads the `DATABASE_URL` from the `.env` file, hence the credentials on the file must be correct for diesel to access and do the necessary actions on the given databse with the given user/password combination.

```
diesel database setup
```

4. Insert the data. To do this source the file `./results/insert.sql` from within MySql/MariaDB. You may need to open the database client from inside the `./results/` directory.

```
source insert.sql
```  

## Run the server

1. To run the server just execute the following command, now you should be able to open `http://localhost` in your browser.

```
cargo run --release
```
  
# Resultados

<img width="1048" alt="Screenshot 2023-11-17 at 6 23 29 a m" src="https://github.com/AOx0/proyecto-bda/assets/50227494/3a0d6806-ec34-4676-8892-3db8afabdf89">
<img width="1048" alt="Screenshot 2023-11-17 at 6 23 11 a m" src="https://github.com/AOx0/proyecto-bda/assets/50227494/c22c3283-1adb-4e92-8784-49cc439b878a">
<img width="1048" alt="Screenshot 2023-11-17 at 6 25 22 a m" src="https://github.com/AOx0/proyecto-bda/assets/50227494/6c5e7ca6-9504-452d-bd6c-2ddb2c626b11">
