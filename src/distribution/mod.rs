use serde::Serialize;
use std::collections::HashMap;
use tiberius::numeric::Decimal;

use crate::model::sale_delivery::OrderForm;
use crate::model::{Code, Region, Warehouse};

#[derive(Debug, Serialize, Clone)]
struct Pool {
    codes: Vec<Code>,
    customers: Vec<String>,
    inventories: HashMap<String, Decimal>,
    quantity_inventory: u32,
    quantity_customer: u32,
    quantity_form: u32,
}

impl Pool {
    fn new() -> Self {
        Self {
            codes: vec![],
            customers: vec![],
            inventories: HashMap::new(),
            quantity_customer: 0,
            quantity_form: 0,
            quantity_inventory: 0,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Group {
    pools: HashMap<Region, HashMap<Warehouse, Pool>>,
    registered: bool,
    pub latest_saledelivery_indb_id: i32,
}

impl Group {
    pub fn new() -> Self {
        Self {
            pools: HashMap::new(),
            registered: false,
            latest_saledelivery_indb_id: 0,
        }
    }

    pub fn register_region(&mut self, r: Vec<&str>) {
        if !self.pools.is_empty() {
            return;
        }
        self.pools = r
            .iter()
            .map(|x| (Region(x.to_string()), HashMap::new()))
            .collect();
    }

    pub fn register_warehouse(&mut self, w: Vec<&str>) {
        if self.registered {
            return;
        }
        self.pools.iter_mut().for_each(|(_, v)| {
            *v = w
                .iter()
                .map(|x| (Warehouse(x.to_string()), Pool::new()))
                .collect()
        });
        self.registered = true
    }

    pub fn clear(&mut self) {
        self.pools.clear();
        self.registered = false;
    }

    pub fn add(&mut self, of: OrderForm) {
        self.latest_saledelivery_indb_id = of.sd.id;

        if let Some(r) = &of.sd.region {
            if let Some(x) = self.pools.get_mut(r) {
                if let Some(y) = x.get_mut(&of.sd.warehouse) {

                    if !y.codes.contains(&of.sd.code) {
                        y.codes.push(of.sd.code);
                        y.quantity_form += 1;
                    }
                    

                    if !y.customers.contains(&of.sd.customer) {
                        y.customers.push(of.sd.customer);
                        y.quantity_customer += 1;
                    };

                    of.inventories.iter().for_each(|sdb| {
                        let count = y
                            .inventories
                            .entry(sdb.inventory.clone())
                            .or_insert(0.into());
                        *count += sdb.quantity;
                    });
                }
            }
        }
    }
}
