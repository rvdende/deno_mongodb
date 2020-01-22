const filenameBase = "deno_mongodb";

let filenameSuffix = ".so";
let filenamePrefix = "lib";

if (Deno.build.os === "win") {
  filenameSuffix = ".dll";
  filenamePrefix = "";
}
if (Deno.build.os === "mac") {
  filenameSuffix = ".dylib";
}

const filename = `../target/${Deno.args[0]}/${filenamePrefix}${filenameBase}${filenameSuffix}`;

console.log(filename);

const plugin = Deno.openPlugin(filename);

//const { testSync, testAsync, testMongo } = plugin.ops;
const { testMongo } = plugin.ops;

const textDecoder = new TextDecoder();

function runMongoTest() {

    testMongo.setAsyncHandler(response => {
        console.log(`Plugin Async Response: ${textDecoder.decode(response)}`);
    });

    const response = testMongo.dispatch(
        new Uint8Array([116, 101, 115, 116]),
        new Uint8Array([116, 101, 115, 116])
    );

    if (response != null || response != undefined) {
        throw new Error("Expected null response!");
    }
}

runMongoTest();

// function runTestSync() {
//   const response = testSync.dispatch(
//     new Uint8Array([116, 101, 115, 116]),
//     new Uint8Array([116, 101, 115, 116])
//   );

//   console.log(`Plugin Sync Response: ${textDecoder.decode(response)}`);
// }

// testAsync.setAsyncHandler(response => {
//   console.log(`Plugin Async Response: ${textDecoder.decode(response)}`);
// });

// function runTestAsync() {
//   const response = testAsync.dispatch(
//     new Uint8Array([116, 101, 115, 116]),
//     new Uint8Array([116, 101, 115, 116])
//   );

//   if (response != null || response != undefined) {
//     throw new Error("Expected null response!");
//   }
// }

// function runTestOpCount() {
//   const start = Deno.metrics();

//   testSync.dispatch(new Uint8Array([116, 101, 115, 116]));

//   const end = Deno.metrics();

//   if (end.opsCompleted - start.opsCompleted !== 2) {
//     // one op for the plugin and one for Deno.metrics
//     throw new Error("The opsCompleted metric is not correct!");
//   }
//   if (end.opsDispatched - start.opsDispatched !== 2) {
//     // one op for the plugin and one for Deno.metrics
//     throw new Error("The opsDispatched metric is not correct!");
//   }
// }

// runTestSync();
// runTestAsync();

// runTestOpCount();