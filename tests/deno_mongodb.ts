import EventEmitter from 'https://deno.land/x/event_emitter/mod.ts';
const textDecoder = new TextDecoder();

export class MongoDB extends EventEmitter {
  plugin:Deno.Plugin;
  testMongo;

  public constructor(connectionString:string,collections?:string[]) {
    super();
    this.plugin = Deno.openPlugin("./target/debug/libdeno_mongodb.so");

    //this.testMongo = plugin.ops.testMongo
    
    setTimeout( ()=>{
        this.connect();
    },1)
  }

  public connect() {
    
    this.plugin.ops.testMongo.setAsyncHandler( res => {
        console.log(`Plugin Async Response: ${textDecoder.decode(res)}`);
    })

    const response = this.plugin.ops.testMongo.dispatch(
        new Uint8Array([116, 101, 115, 116]),
        new Uint8Array([116, 101, 115, 116])
    );

    if (response != null || response != undefined) {
        throw new Error("Expected null response!");
    }

    this.emit('connect', 'test event!!');
    //return this; // Chainable
  }
}