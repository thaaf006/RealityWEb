#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec,
};

// Struktur data inventory
#[contracttype]
#[derive(Clone, Debug)]
pub struct Item {
    id: u64,
    name: String,
    stock: u32,
    description: String,
}

// Storage key
const INVENTORY_DATA: Symbol = symbol_short!("INVENTORY");

#[contract]
pub struct InventoryContract;

#[contractimpl]
impl InventoryContract {
    // Ambil semua item inventory
    pub fn get_items(env: Env) -> Vec<Item> {
        env.storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Tambah item baru
    pub fn add_item(
        env: Env,
        name: String,
        stock: u32,
        description: String,
    ) -> String {
        // 1. Ambil data inventory lama
        let mut items: Vec<Item> = env
            .storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. Buat item baru
        let item = Item {
            id: env.prng().gen::<u64>(),
            name,
            stock,
            description,
        };

        // 3. Tambahkan ke inventory
        items.push_back(item);

        // 4. Simpan kembali ke storage
        env.storage().instance().set(&INVENTORY_DATA, &items);

        String::from_str(&env, "Item berhasil ditambahkan")
    }

    // Hapus item berdasarkan id
    pub fn delete_item(env: Env, id: u64) -> String {
        // 1. Ambil data inventory
        let mut items: Vec<Item> = env
            .storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. Cari item berdasarkan id
        for i in 0..items.len() {
            if items.get(i).unwrap().id == id {
                items.remove(i);

                // 3. Simpan perubahan
                env.storage().instance().set(&INVENTORY_DATA, &items);

                return String::from_str(&env, "Item berhasil dihapus");
            }
        }

        String::from_str(&env, "Item tidak ditemukan")
    }

    // Update stok item
    pub fn update_stock(env: Env, id: u64, new_stock: u32) -> String {
        // 1. Ambil data inventory
        let mut items: Vec<Item> = env
            .storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. Cari item
        for i in 0..items.len() {
            let mut item = items.get(i).unwrap();

            if item.id == id {
                // 3. Update stock
                item.stock = new_stock;

                // 4. Simpan kembali ke vector
                items.set(i, item);

                // 5. Simpan ke storage
                env.storage().instance().set(&INVENTORY_DATA, &items);

                return String::from_str(&env, "Stock berhasil diupdate");
            }
        }

        String::from_str(&env, "Item tidak ditemukan")
    }
}

mod test;