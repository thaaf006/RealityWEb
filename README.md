# Inventory Smart Contract

Aplikasi inventory sederhana menggunakan **Soroban Smart Contract** di jaringan Stellar.  
Contract ini memungkinkan pengguna untuk:

- Menambahkan item inventory
- Melihat daftar item
- Mengupdate stok item
- Menghapus item dari inventory

---

# Features

✅ Add Item  
✅ Get All Items  
✅ Update Stock  
✅ Delete Item  

---

# Struktur Data

```rust
pub struct Item {
    id: u64,
    name: String,
    stock: u32,
    description: String,
}
```

---

# Functions

## 1. Add Item

Menambahkan item baru ke inventory.

```rust
add_item(env, name, stock, description)
```

### Parameters

| Parameter | Type | Description |
|---|---|---|
| name | String | Nama item |
| stock | u32 | Jumlah stok |
| description | String | Deskripsi item |

---

## 2. Get Items

Mengambil seluruh data inventory.

```rust
get_items(env)
```

### Return

```rust
Vec<Item>
```

---

## 3. Update Stock

Mengupdate jumlah stok item berdasarkan ID.

```rust
update_stock(env, id, new_stock)
```

### Parameters

| Parameter | Type | Description |
|---|---|---|
| id | u64 | ID item |
| new_stock | u32 | Jumlah stok baru |

---

## 4. Delete Item

Menghapus item berdasarkan ID.

```rust
delete_item(env, id)
```

### Parameters

| Parameter | Type | Description |
|---|---|---|
| id | u64 | ID item |

---

# Cara Menjalankan

## 1. Clone Repository

```bash
git clone https://github.com/username/inventory-smartcontract.git
cd inventory-smartcontract
```

---

## 2. Build Contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

---

## 3. Jalankan Test

```bash
cargo test
```

---

# Struktur Project

```bash
.
├── src
│   ├── lib.rs
│   └── test.rs
├── Cargo.toml
└── README.md
```

---

# Example Usage

## Add Item

```rust
add_item(
    env,
    String::from_str(&env, "Laptop"),
    10,
    String::from_str(&env, "Laptop Gaming")
);
```

---

## Update Stock

```rust
update_stock(env, 12345, 20);
```

---

## Delete Item

```rust
delete_item(env, 12345);
```

---

# Teknologi

- Rust
- Soroban SDK
- Stellar Blockchain

---

# Author

Developed using Soroban Smart Contract 🚀

CDH5PWN7NXFUDJEJ546TQN5EEFIR66PXY4MZ2I6N77TDWJNHZY4ACGCL
