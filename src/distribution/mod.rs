#[derive(Debug, PartialEq, Eq, Hash)]
struct Warehouse(String);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Region(String);

#[derive(Debug, PartialEq, Eq)]
struct Customer(String);

#[derive(Debug)]
struct Inventory(String, u32);

#[derive(Debug)]
struct Form {
    customer: Customer,
    region: Region,
    warehouse: Warehouse,
    inventories: Vec<Inventory>,
}

#[derive(Debug)]
struct Pool {
    warehouse: Warehouse,
    customers: Vec<Customer>,
    inventories: Vec<Inventory>,
    invs: HashMap<String, u32>,
    quantity_customer: u32,
    quantity_inventory: u32,
}

impl Pool {
    fn new(w: &str) -> Self {
        Self {
            warehouse: Warehouse(w.into()),
            customers: vec![],
            inventories: vec![],
            invs: HashMap::new(),
            quantity_customer: 0,
            quantity_inventory: 0,
        }
    }

    fn add(&mut self, f: Form) {
        if !self.customers.contains(&f.customer) {
            println!("not contains");
            self.customers.push(f.customer);
            self.quantity_customer += 1;
        }
        f.inventories.iter().for_each(|v| {
            let counter = self.invs.entry(v.0.clone()).or_insert(0);
            *counter += v.1;
        });
        self.quantity_inventory = self.invs.values().sum();
    }
}

use std::collections::HashMap;

#[derive(Debug)]
struct GroupPools {
    pools: HashMap<Region, HashMap<Warehouse, Pool>>,
}

impl GroupPools {
    fn new() -> Self {
        Self {
            pools: HashMap::new(),
        }
    }

    fn register_region(&mut self, r: &str) {
        self.pools.insert(Region(r.into()), HashMap::new());
    }

    fn register_warehouse(&mut self, w: &str) {
        self.pools.iter_mut().for_each(|(_, v)| {
            v.insert(Warehouse(w.into()), Pool::new(w));
        });
    }

    fn add(&mut self, f: Form) {
        if let Some(v) = self.pools.get_mut(&f.region) {
            if let Some(p) = v.get_mut(&f.warehouse) {
                p.add(f);
            }
        }
    }
}
