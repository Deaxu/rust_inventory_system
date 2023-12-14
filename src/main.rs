use std::io;
mod users;
use chrono::prelude::*;
fn main() {
    let user_db = users::UserDatabase::new();

    println!("Kullanıcı Adı: ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Okuma hatası");
    let username = username.trim().to_string();

    println!("Şifre: ");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Okuma hatası");
    let password = password.trim().to_string();

    if security(&user_db, &username, &password) {
        println!("Giriş başarılı!");
    } else {
        panic!("Kullanıcı adı veya şifre hatalı!!");
    }

    let bilgisayar = Product{
        urun_adi: "Bilgisayar".to_string(),
        aciklama : "Teknolojik alet".to_string(),
        fiyat : 3200,
        miktar : 100,
    };

    let telefon = Product{
        urun_adi: "Telefon".to_string(),
        aciklama : "Teknolojik alet".to_string(),
        fiyat : 1700,
        miktar : 150,
    };

    let mut inventory = Inventory::new();

    inventory.product.push(bilgisayar);
    inventory.product.push(telefon);

    loop {
        println!("\n1) Envanter Yönetimi\n2) Satış Yönetimi\n3) Alış Yönetim\n0) Çıkış");

        let secim = String::new();
        let secim = take_input(secim.clone());
        let secim = parse_u32(secim.clone());
        
        if secim == 0 {
            panic!("Çıkış!");
        }

        loop {
            match secim {
                0 => {break;}
        
                1 => {
                    println!("\n0) Geri\n1) Ürün Ekle \n2) Ürün Sil \n3) Envanteri Göster \n4) Logs");
                    
                    let secim_1 = String::new();
                    let secim_1 = take_input(secim_1);
                    let secim_1 = parse_u32(secim_1);

                    match secim_1 {
                        0 => {break;}
        
                        1 => {
                            inventory.add_product();
                            break;
                        }

                        2 =>{
                            inventory.delete_product();
                            break;
                        }

                        3 =>{
                            inventory.display();
                            break;
                        }

                        4 =>{
                            inventory.display_history();
                            break;
                        }
                        
                        _ => println!("Geçersiz değer."),
                    }
                }
                2 =>{
                    inventory.sell();
                    break;
                }
                _ => println!("Geçersiz değer."),
            }    
        }
    }
}

struct Product{
    urun_adi:String,
    aciklama:String,
    fiyat:u32,
    miktar:u32,
}

struct Inventory{
    product: Vec<Product>,
    history: Vec<History>
}

struct History{
    eklenen_urun: String,
    zaman: DateTime<Utc>,
}

trait HistoryManagemnt {
    fn history(&mut self) -> String;
}

trait InventoryManagement  {
    fn new() -> Self;
    fn add_product(&mut self) -> Product;
    fn delete_product(&mut self) -> Product;
    fn set_produect(&mut self) -> Product;
    fn display(&mut self) -> Product;
    fn display_history(&mut self);
}

trait SalesManagement {
    fn sell(&mut self) -> Product;
    fn buy(&mut self) -> Product;
}

impl Inventory {
    fn new() -> Inventory {
        Inventory {
            product: Vec::new(),
            history: Vec::new(), }
    }

    fn add_product(&mut self){
        println!("Ürün Adı giriniz: ");
        let urun_adi = String::new();
        let urun_adi = take_input(urun_adi);
            
        println!("Açıklama: ");
        let aciklama = String::new();
        let aciklama = take_input(aciklama);
            
        println!("Fiyat: ");
        let fiyat = String::new();
        let fiyat = take_input(fiyat);
        let fiyat = parse_u32(fiyat);
            
        println!("Miktar: ");
        let miktar = String::new();
        let miktar = take_input(miktar);
        let miktar = parse_u32(miktar);
            
        let new_product = Product{
            urun_adi: urun_adi.clone(),
            aciklama: aciklama.clone(),
            fiyat,
            miktar,};
            
        self.product.push(new_product);

        let new_history = History {
            eklenen_urun: urun_adi.clone(),
            zaman: Utc::now(),
        };

        self.history.push(new_history);

        println!("Maliyet= {}", fiyat*miktar);
    }

    fn delete_product(&mut self) -> Option<Product>{

        self.display();

        println!("Silmek istediğiniz ürünün indeksi: ");

        let indeks = String::new();
        let indeks = take_input(indeks);
        let indeks: usize = indeks.trim().parse().expect("");

        if indeks < self.product.len() {
            Some(self.product.remove(indeks))
        } else {
            None
        }
    }

    fn display(&mut self) {
        let mut count: u32 = 0;
        println!("Envanter: ");
    
        for product in &self.product {
            println!("{}) Ürün Adı: {}, Açıklama: {}, Fiyat: {}, Miktar: {}",
                count,
                product.urun_adi,
                product.aciklama,
                product.fiyat,
                product.miktar,
            );
            count += 1;
        }
    }

    fn sell(&mut self){
        self.display();

        println!("Satmak istenilen ürünün numarası: ");
        let indeks = String::new();
        let indeks = take_input(indeks);
        let indeks: usize = indeks.trim().parse().expect("");

        println!("Satış fiyatı: ");
        let sale_price = String::new();
        let sale_price = take_input(sale_price);
        let sale_price: u32 = sale_price.trim().parse().expect(""); 

        if indeks < self.product.len(){
            println!("Satılan adet: ");
            let amount = String::new();
            let amount = take_input(amount);
            let amount: u32 = amount.trim().parse().expect("");

            if amount <= self.product[indeks].miktar {
                self.product[indeks].miktar -= amount;

                let kar = (sale_price - self.product[indeks].fiyat) * amount;
                println!("Satıştan elde edilen kar: {}", kar);
            }
            else{
                println!("Yetersiz stok!");
            }
        }
    }

    fn display_history(&mut self){
        for x in &self.history{
            println!("Eklenen Ürün: {}, Zamanı: {}", x.eklenen_urun, x.zaman);
        }
    }
}

fn security(user_db: &users::UserDatabase, username: &str, password: &str) -> bool {
    if let Some(expected_password) = user_db.get_password(username) {
        return *expected_password == password;
    }
    false
}

fn take_input(mut input: String) -> String{
    let _ = io::stdin().read_line(&mut input);
    let input = input.trim().to_string();
    input
}

fn parse_u32(input: String) -> u32{
    let input: u32 = input.trim().parse().expect("Invalid input");
    input
}