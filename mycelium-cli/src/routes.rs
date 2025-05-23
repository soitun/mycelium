use mycelium_api::{NoRouteSubnet, QueriedSubnet, Route};
use prettytable::{row, Table};
use std::net::SocketAddr;

use tracing::{debug, error};

pub async fn list_selected_routes(
    server_addr: SocketAddr,
    json_print: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let request_url = format!("http://{server_addr}/api/v1/admin/routes/selected");
    match reqwest::get(&request_url).await {
        Err(e) => {
            error!("Failed to retrieve selected routes");
            return Err(e.into());
        }
        Ok(resp) => {
            debug!("Listing selected routes");

            if json_print {
                // API call returns routes in JSON format by default
                let selected_routes = resp.text().await?;
                println!("{selected_routes}");
            } else {
                // Print routes in table format
                let routes: Vec<Route> = resp.json().await?;
                let mut table = Table::new();
                table.add_row(row!["Subnet", "Next Hop", "Metric", "Seq No"]);

                for route in routes.iter() {
                    table.add_row(row![
                        &route.subnet,
                        &route.next_hop,
                        route.metric,
                        route.seqno,
                    ]);
                }

                table.printstd();
            }
        }
    }

    Ok(())
}

pub async fn list_fallback_routes(
    server_addr: SocketAddr,
    json_print: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let request_url = format!("http://{server_addr}/api/v1/admin/routes/fallback");
    match reqwest::get(&request_url).await {
        Err(e) => {
            error!("Failed to retrieve fallback routes");
            return Err(e.into());
        }
        Ok(resp) => {
            debug!("Listing fallback routes");

            if json_print {
                // API call returns routes in JSON format by default
                let fallback_routes = resp.text().await?;
                println!("{fallback_routes}");
            } else {
                // Print routes in table format
                let routes: Vec<Route> = resp.json().await?;
                let mut table = Table::new();
                table.add_row(row!["Subnet", "Next Hop", "Metric", "Seq No"]);

                for route in routes.iter() {
                    table.add_row(row![
                        &route.subnet,
                        &route.next_hop,
                        route.metric,
                        route.seqno,
                    ]);
                }

                table.printstd();
            }
        }
    }
    Ok(())
}

pub async fn list_queried_subnets(
    server_addr: SocketAddr,
    json_print: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let request_url = format!("http://{server_addr}/api/v1/admin/routes/queried");
    match reqwest::get(&request_url).await {
        Err(e) => {
            error!("Failed to retrieve queried subnets");
            return Err(e.into());
        }
        Ok(resp) => {
            debug!("Listing queried routes");

            if json_print {
                // API call returns routes in JSON format by default
                let queried_routes = resp.text().await?;
                println!("{queried_routes}");
            } else {
                // Print routes in table format
                let queries: Vec<QueriedSubnet> = resp.json().await?;
                let mut table = Table::new();
                table.add_row(row!["Subnet", "Query expiration"]);

                for query in queries.iter() {
                    table.add_row(row![query.subnet, query.expiration,]);
                }

                table.printstd();
            }
        }
    }
    Ok(())
}

pub async fn list_no_route_entries(
    server_addr: SocketAddr,
    json_print: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let request_url = format!("http://{server_addr}/api/v1/admin/routes/no_route");
    match reqwest::get(&request_url).await {
        Err(e) => {
            error!("Failed to retrieve subnets with no route entries");
            return Err(e.into());
        }
        Ok(resp) => {
            debug!("Listing no route entries");

            if json_print {
                // API call returns routes in JSON format by default
                let nrs = resp.text().await?;
                println!("{nrs}");
            } else {
                // Print routes in table format
                let no_routes: Vec<NoRouteSubnet> = resp.json().await?;
                let mut table = Table::new();
                table.add_row(row!["Subnet", "Entry expiration"]);

                for nrs in no_routes.iter() {
                    table.add_row(row![nrs.subnet, nrs.expiration,]);
                }

                table.printstd();
            }
        }
    }
    Ok(())
}
