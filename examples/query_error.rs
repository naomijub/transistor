use transistor::client::Crux;
use transistor::types::{query::Query};

fn main() -> Result<(), transistor::types::error::CruxError>{
    let client = Crux::new("localhost", "3000").docker_client();

    // field `n` doesn't exist
    let query_error_response = Query::find(vec!["p1", "n"])
        .where_clause(vec!["p1 :name g", "p1 :is-sql true"])
        .build();

    let error = client.query(query_error_response?)?;
    println!("Stacktrace \n{:?}", error);
    // Stacktrace
    // QueryError("{:via
    //      [{:type java.lang.IllegalArgumentException,
    //        :message \"Find refers to unknown variable: n\",
    //    :at [crux.query$q invokeStatic \"query.clj\" 1152]}],
    //  :trace
    //  [[crux.query$q invokeStatic \"query.clj\" 1152]
    //   [crux.query$q invoke \"query.clj\" 1099]
    //   [crux.query$q$fn__10850 invoke \"query.clj\" 1107]
    //   [clojure.core$binding_conveyor_fn$fn__5754 invoke \"core.clj\" 2030]
    //   [clojure.lang.AFn call \"AFn.java\" 18]
    //   [java.util.concurrent.FutureTask run \"FutureTask.java\" 264]
    //   [java.util.concurrent.ThreadPoolExecutor
    //    runWorker
    //    \"ThreadPoolExecutor.java\"
    //    1128]
    //   [java.util.concurrent.ThreadPoolExecutor$Worker
    //    run
    //    \"ThreadPoolExecutor.java\"
    //    628]
    //   [java.lang.Thread run \"Thread.java\" 834]],
    //  :cause \"Find refers to unknown variable: n\"}
    // ")

    Ok(())
}
