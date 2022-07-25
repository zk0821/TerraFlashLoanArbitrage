pub struct Pool<'a> {
    pub contract_address: &'a str,
    pub token: &'a str,
    pub native_token: &'a str,
}

pub const ASTROPORT_POOLS: [Pool; 1] = [
    Pool {
        contract_address: "terra1udsua9w6jljwxwgwsegvt6v657rg3ayfvemupnes7lrggd28s0wq7g8azm",
        token: "terra167dsqkh2alurx997wmycw9ydkyu54gyswe3ygmrs4lwume3vmwks8ruqnv",
        native_token: "uluna"
    }
];

pub const TERRASWAP_POOLS: [Pool; 1] = [
    Pool {
        contract_address: "terra1ksu84lkky4pshnu2dyqvfvk789ypvlykhtqrk9nsjfsh9t5qy9dsaf8r04",
        token: "terra167dsqkh2alurx997wmycw9ydkyu54gyswe3ygmrs4lwume3vmwks8ruqnv",
        native_token: "uluna"
    }
];