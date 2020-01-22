use mongodb::{Client, options::ClientOptions};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


fn main() -> Result<()> {

    
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")?;

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // Get a handle to a database.
    let _db = client.database("scratchfixpro");

    // List the names of the collections in that database.
    for collection_name in _db.list_collection_names(None)? {
        println!("{}", collection_name);
    }

    // // List the names of the databases in that deployment.
    // println!(client.list_database_names())
    // // for db_name in client.list_database_names(None) {
    // //     println!("{}", db_name);
    // // }

    // println!("Hello, world!");
    Ok(())
}
