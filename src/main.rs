use axum::{
    extract::{ConnectInfo, Json},
    routing::{get, Router},
};
use axum_client_ip::InsecureClientIp;
use ip2location::{error, LocationRecord, Record, DB};
use serde_json::{json, Value};
use std::net::{IpAddr, SocketAddr};
use daemonize::Daemonize;

#[tokio::main]
async fn main() {
    let daemonize = Daemonize::new()
        .pid_file("ip2location.pid")
        .chown_pid_file(true)
        .working_directory("./")
        .user("root")
        .group("root")
        .privileged_action(|| "Executed before drop privileges");
    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => eprintln!("Error, {}", e),
    }
    let addr = "0.0.0.0:3000";
    println!("Listening on {}", addr);

    // build our application with a single route
    let app = Router::new().route("/", get(handler));
    // run it with hyper on localhost:3000
    axum::Server::bind(&addr.parse().unwrap())
        .serve(
            // Don't forget to add `ConnectInfo`
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
}

async fn handler(InsecureClientIp(ip): InsecureClientIp) -> Result<Json<Value>, Json<Value>> {
    const IPV6BIN: &str = "IP2Location/IP-COUNTRY-REGION-CITY-LATITUDE-LONGITUDE.BIN";
    let db = DB::from_file(IPV6BIN).unwrap();
    let res = ip_lookup_in_ipv6bin(ip.to_string(), db);
    if res.is_err() {
        return Err(Json(json!({ "error": "Record not found"})));
    }
    let res = res.unwrap();
    let response = json!({
        "city": res.city.unwrap_or_else(|| "".to_owned()),
        "latitude": res.latitude.unwrap_or_else(|| 0.0),
        "longitude": res.longitude.unwrap_or_else(|| 0.0),
    });
    Ok(Json(response))
}

// Lookup an IPv4 in the IP2Location V6 BIN Database
fn ip_lookup_in_ipv6bin(ip: String, mut db: DB) -> Result<LocationRecord, error::Error> {
    let record = db.ip_lookup(ip.parse().unwrap())?;
    let record = if let Record::LocationDb(rec) = record {
        Some(rec)
    } else {
        None
    };
    // if no record was found return error
    if record.is_none() {
        return Err(error::Error::RecordNotFound);
    }

    Ok(record.unwrap())
}
