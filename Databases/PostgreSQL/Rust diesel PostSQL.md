We can use Rust and PostgreSQL with an Object-Relational Mapping (ORM) tool. I chose Diesel, but you can choose whatever you want.

## Preparing 

1. Install `libpq`: `brew install libpq`. Sometimes you'll need to run `brew link --force libpq`
2. Install the diesel cli `cargo install diesel_cli --no-default-features --features postgres`
> You can read the [documentation](https://diesel.rs/guides/getting-started)

We will be using the Dockerfile defined in [[Architecture]]

If you want to run this `psql` command `\copy track_raw(title,artist,album,count,rating,len) FROM 'library.csv' WITH DELIMITER ',' CSV;` using the `postgresql` crate, you'll need to do something like this
```rust
let mut client =  
    Client::connect(&connection_string, NoTls).expect("Failed to connect to the database");  
  
// Start the COPY stream  
let mut copy_in = client  
    .copy_in("COPY track_raw(title, artist, album, count, rating, len) FROM STDIN WITH CSV")
    .expect("Failed to start COPY");
// ... read stream
let file = File::open(csv_file_path).expect("Failed to open CSV");  
let mut reader = BufReader::new(file);  
  
// Stream contents to the COPY process  
std::io::copy(&mut reader, &mut copy_in).expect("Failed to copy data to database");
copy_in.finish().expect("Failed to finish copy");
```
You can see how we used `STDIN` to read and send the data