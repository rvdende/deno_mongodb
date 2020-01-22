#[macro_use]
extern crate deno_core;
extern crate futures;

use deno_core::CoreOp;
use deno_core::Op;
use deno_core::PluginInitContext;
use deno_core::{Buf, PinnedBuf};
use futures::future::FutureExt;

// mongodb
use mongodb::{ Client };



fn init(context: &mut dyn PluginInitContext) {
  //context.register_op("testSync", Box::new(op_test_sync));
  //context.register_op("testAsync", Box::new(op_test_async));
  context.register_op("testMongo", Box::new(op_test_mongo));
}
init_fn!(init);

// pub fn op_test_sync(data: &[u8], zero_copy: Option<PinnedBuf>) -> CoreOp {
//   if let Some(buf) = zero_copy {
//     let data_str = std::str::from_utf8(&data[..]).unwrap();
//     let buf_str = std::str::from_utf8(&buf[..]).unwrap();
//     println!(
//       "Hello from plugin. data: {} | zero_copy: {}",
//       data_str, buf_str
//     );
//   }
//   let result = b"test";
//   let result_box: Buf = Box::new(*result);
//   Op::Sync(result_box)
// }

// pub fn op_test_async(data: &[u8], zero_copy: Option<PinnedBuf>) -> CoreOp {
//   let data_str = std::str::from_utf8(&data[..]).unwrap().to_string();
//   let fut = async move {
//     if let Some(buf) = zero_copy {
//       let buf_str = std::str::from_utf8(&buf[..]).unwrap();
//       println!(
//         "Hello from plugin. data: {} | zero_copy: {}",
//         data_str, buf_str
//       );
//     }
//     let (tx, rx) = futures::channel::oneshot::channel::<Result<(), ()>>();
//     std::thread::spawn(move || {
//       std::thread::sleep(std::time::Duration::from_secs(1));
//       tx.send(Ok(())).unwrap();
//     });
//     assert!(rx.await.is_ok());
//     let result = b"test";
//     let result_box: Buf = Box::new(*result);
//     Ok(result_box)
//   };

//   Op::Async(fut.boxed())
// }

pub fn op_test_mongo(data: &[u8], zero_copy: Option<PinnedBuf>) -> CoreOp {
  let data_str = std::str::from_utf8(&data[..]).unwrap().to_string();
  let fut = async move {
    if let Some(buf) = zero_copy {

      //let mut client_options = ClientOptions::parse("mongodb://localhost:27017")?;
      let buf_str = list_database_names();

      //let buf_str = std::str::from_utf8(&buf[..]).unwrap();
      println!(
        "Hello from plugin. data: {} | zero_copy: {}",
        data_str, buf_str
      );
    }
    
    let (tx, rx) = futures::channel::oneshot::channel::<Result<(), ()>>();

    std::thread::spawn(move || {
      std::thread::sleep(std::time::Duration::from_secs(1));
      tx.send(Ok(())).unwrap();
    });


    /* */
    //mongodoit();

    assert!(rx.await.is_ok());
    let result = b"test";
    let result_box: Buf = Box::new(*result);
    Ok(result_box)
  };

  

  Op::Async(fut.boxed())
}

fn list_collection_names() {
  println!("quick test...");

  let client = Client::with_uri_str("mongodb://localhost:27017");

  match client {
    Ok(client) => {
      let _db = client.database("scratchfixpro");

      let collection_names = _db.list_collection_names(None);

      match collection_names {
        Ok(collection_names) => {
          for collection_name in collection_names {
            println!("{}", collection_name);        
          }          
        },
        Err(e) => {
          panic!("{}",e)
        }
      }

      
    },
    Err(e) => {
      panic!("{}",e); 
    }
  };

  // Get a handle to a database.
  

}




fn list_database_names() -> (String) {
  println!("quick test...");

  let client = Client::with_uri_str("mongodb://localhost:27017");

  match client {
    Ok(client) => {
      //let _db = client.database("scratchfixpro");

      let dbnames = client.list_database_names(None);

      match dbnames {
        Ok(dbnames) => {
          // add first [
          let mut dbnamesstring = "[".to_string();
          
          for n in 0..(dbnames.len()) {
            //println!("{}", name);        
            dbnamesstring = format!("{}\"{}\"",dbnamesstring,dbnames[n]);

            // add , after if it is not the last string
            if n != (dbnames.len()-1) {
              dbnamesstring = format!("{},",dbnamesstring);
            }

          }          

          
          // add last ]
          dbnamesstring = format!("{}]",dbnamesstring);
          return dbnamesstring;
        },
        Err(e) => {
          panic!("{}",e)
        }
      }

      
    },
    Err(e) => {
      panic!("{}",e); 
    }
  };

  // Get a handle to a database.
  

}