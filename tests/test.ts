import { MongoDB } from "./deno_mongodb.ts"

const db = new MongoDB("mongodb://localhost:27017");

db.on('connect', (message: string): void => {
  console.log(`Message received: ${message}`);
});

//db.test();
// Message received: The createEvent() method was called