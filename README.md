# deno_mongodb
WIP do not use. Deno plugin for mongodb
Not much works right now.

You must have mongodb running without auth on `mongodb://localhost:27017`

## Build the library:

```
cargo build --lib
deno --allow-plugin tests/test.ts
```

## Usage

Similar to https://www.npmjs.com/package/mongojs and official mongo queries.

```ts
import { MongoDB } from "./deno_mongodb.ts"

const db = new MongoDB("mongodb://localhost:27017/mydb");

db.on('connect', () => {
  console.log(`Mongo connected.`);
});

db.list_database_names( (dbs) => { 
    console.log("dbs: " + dbs); 
})

db.list_collection_names( (collections) => { 
    console.log("collections: " + collections); 
})

db.users.find( {} , (err,users) => {
    if (err) { console.error(err); }
    if (users) { console.log("users: " + users); }    
})
```