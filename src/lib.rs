extern crate rustc_serialize;
extern crate gtk;
extern crate bitcoin;
extern crate secp256k1;
extern crate rand;

use rand::{Rng, thread_rng};

use rustc_serialize::{Decodable, Decoder};
use rustc_serialize::json::{self, ToJson, Json};

use secp256k1::{Secp256k1};
use secp256k1::key::{PublicKey, SecretKey};

use bitcoin::util::base58::{self, FromBase58, ToBase58};
use bitcoin::util::{address, hash};

use std::error::Error;
use std::f32;
use std::env;
use std::process::Command;
use std::io::prelude::*;
use std::io;
use std::fs;
use std::str;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::fs::Metadata;
use std::rc::Rc;
use std::cell::RefCell;

use gtk::traits::*;
use gtk::signal::Inhibit;
use gtk::widgets::Builder;
use gtk::{signal, widgets};
use gtk::signal::TreeViewSignals;
use gtk::Window;

#[test]
fn it_works() {
}

//bitcoin related

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub enum Condition {
    Cancel,
    Cancelled,
    Alive,
    Enabled,
    Modifying,
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Friend {
    name: String,
    public_key: String,
}
#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct BuyOffer {
    pub item: Item,
    pub alias_owner: String,
    pub quantity: i64,
    pub price: f64,
    pub condition: Condition,
    pub hash_id: String,
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct SellOffer {
    pub item: Item,
    pub alias_owner: String,
    pub quantity: i64,
    pub price: f64,
    pub condition: Condition,
    pub hash_id: String,
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub quantity: i64,
    pub recommended_price: f64,
    pub contact: String,
    pub brand_name: String,
    pub keywords: Vec<String>,
    pub images: Vec<String>,
    pub alias_owner: String,
}

impl Item {
    pub fn new_full(name: String, description: String, quantity: i64, recommended_price: f64, contact: String, brand_name: String, 
        keywords: Vec<String>, images: Vec<String>, alias_owner: String) -> Item {
        Item { name: name, description: description, quantity: quantity, recommended_price: recommended_price, 
            contact: contact, brand_name: brand_name, keywords: keywords, images: images, alias_owner: alias_owner }
    }
    pub fn new() -> Item {
        Item { name: "".to_string(), description: "".to_string(), quantity: 0, recommended_price: 0.00, 
            contact: "".to_string(), brand_name: "".to_string(), keywords: Vec::new(), images: Vec::new(), alias_owner: "".to_string() }
    }

    pub fn change_quantity(&mut self, quant: i64) {
        self.quantity == quant;
    }

}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct ItemTransaction {
    timestamp: String,
    item: Item,
    price: f64,
    quantity: i64,
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct CoinTransaction {
    timestamp: String,
    address: String,
    amount: i64,
    cointype: String,
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Alias {
	pub name: String,
	pub bitcoin_public_keys: Vec<String>,
	pub bitcoin_private_keys: Vec<String>,
	pub safecoin_public_keys: Vec<String>,
	pub safecoin_private_keys: Vec<String>,
    pub items: Vec<Item>,
    pub selloffers: Vec<SellOffer>,
    pub buyoffers: Vec<BuyOffer>,
    pub public_key: String,
    pub private_key: String,
    pub item_trans: Vec<ItemTransaction>,
    pub coin_trans: Vec<CoinTransaction>,
    pub friends: Vec<Friend>,
    pub mock_balance: i64,
}

impl Alias {
    pub fn new() -> Alias {
        Alias { name: "".to_string(), bitcoin_public_keys: Vec::new(), bitcoin_private_keys: Vec::new(), safecoin_public_keys: Vec::new(), safecoin_private_keys: Vec::new(), items: Vec::new(), selloffers: Vec::new(), buyoffers: Vec::new(), public_key: "".to_string(), private_key: "".to_string(), item_trans: Vec::new(), coin_trans: Vec::new(), friends: Vec::new(), mock_balance: 0, }
    }
    //pub fn new(self) -> Alias { self }
    pub fn change_item_quantity(&mut self, quant: i64, item_index: usize) {
        self.items[item_index].change_quantity(quant);
    }
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Profile {
	pub username: String,
	pub password: String,
	pub alias: Vec<Alias>,
	pub private_key: String,
	pub public_key: String,
}

impl Profile {
    pub fn new() -> Profile {
        Profile { username: "".to_string(), password: "".to_string(), alias: Vec::new(), private_key: "".to_string(), public_key: "".to_string() }
    }
    pub fn change_item_quantity(&mut self, quant: i64, item_index: usize, alias_index: usize) {
        self.alias[alias_index].change_item_quantity(quant, item_index);
        
    }
    pub fn add_buy_offer(&mut self, buy: BuyOffer, item_index: usize, alias_index: usize) {

    }


}

pub fn add_new_alias(name: &str) {

}

pub fn make_new_account(path: &str, username: &str, password: &str, alt_name: &str) {
	touch(&Path::new(path)).unwrap_or_else(|why| {
               println!("! {:?}", why.kind());
    });
    let mut bitcoin_pub_vec: Vec<String> = Vec::new();
    let mut bitcoin_priv_vec: Vec<String> = Vec::new();
    let safecoin_pub_vec: Vec<String> = Vec::new();
    let safecoin_priv_vec: Vec<String> = Vec::new();
    let inventory_vec: Vec<Item> = Vec::new();
    let buy_vec: Vec<BuyOffer> = Vec::new();
    let sell_vec: Vec<SellOffer> = Vec::new();
    let mut alias_vec: Vec<Alias> = Vec::new();
    let item_trans_vec: Vec<ItemTransaction> = Vec::new();
    let coin_trans_vec: Vec<CoinTransaction> = Vec::new();
    let friends_vec: Vec<Friend> = Vec::new();

    let s = Secp256k1::new();
    let (sk, pk) = s.generate_keypair(&mut thread_rng()).unwrap();

    let the_addr = bitcoin::util::address::Address { 
      ty: bitcoin::util::address::Type::PubkeyHash, network: bitcoin::network::constants::Network::Bitcoin, hash: bitcoin::util::hash::Hash160::from_data(&pk.serialize_vec(&s, true)[..])};
        //let the_Str = sk.serialize_vec(&s, true);
    println!("{:?}", the_addr);
    let pk_base = the_addr.to_base58check();
    println!("{:?}", &pk_base);
      
    let mut format_sk = format!("{:?}", sk);
    let string_len = format_sk.len() - 1;
    format_sk.remove(string_len);
    for i in 0..10 {
        format_sk.remove(0);
    }
    let sk_byte = format_sk.as_bytes();
    let sk_base = bitcoin::util::base58::base58_encode_slice(sk_byte);
    println!("{:?}",sk_base);
    bitcoin_priv_vec.push(sk_base);
    bitcoin_pub_vec.push(pk_base);
    let the_alias = Alias {
    	name: alt_name.to_string(),
    	bitcoin_public_keys: bitcoin_pub_vec,
    	bitcoin_private_keys: bitcoin_priv_vec,
		safecoin_public_keys: safecoin_pub_vec,
		safecoin_private_keys: safecoin_priv_vec,
        items: inventory_vec,
        buyoffers: buy_vec,
        selloffers: sell_vec,
        public_key: "".to_string(),
        private_key: "".to_string(),
        item_trans: item_trans_vec,
        coin_trans: coin_trans_vec,
        friends: friends_vec,
        mock_balance: 1000,
    };
    alias_vec.push(the_alias);

    let the_profile = Profile {
    	username: username.to_string(),
		password: password.to_string(),
		alias: alias_vec,
		private_key: "".to_string(),
		public_key: "".to_string(),
    };

    let display = "a";
    let mut file = match OpenOptions::new().read(true).write(true).open(path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let encoded = json::encode(&the_profile).unwrap();
	let json_str = encoded.to_string();
	file.write_all(&encoded.as_bytes()).unwrap();
}

pub fn write_account(new_profile: &Profile) {
    let mut the_home_dir = String::new();

    match env::home_dir() {
        Some(ref p) => the_home_dir = p.display().to_string(),
        None => println!("Impossible to get your home dir!")
    }
    let path_string = String::from("/.test_root/");
    let path_string2 = path_string + &new_profile.username;
    let path_string3 = the_home_dir + &path_string2;
    let path = Path::new(&path_string3); 
    let display = "a";
    let mut file = match OpenOptions::new().read(true).write(true).open(path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let encoded = json::encode(&new_profile).unwrap();
    let json_str = encoded.to_string();
    file.write_all(&encoded.as_bytes()).unwrap();
}

pub fn read_account(username: &str) -> Profile {
    let mut the_home_dir = String::new();

    match env::home_dir() {
        Some(ref p) => the_home_dir = p.display().to_string(),
        None => println!("Impossible to get your home dir!")
    }
    let path_string = String::from("/.test_root/");
    let path_string2 = path_string + username;
    let path_string3 = the_home_dir + &path_string2;
    let path = Path::new(&path_string3);
	let display = "a";
    let mut file = match OpenOptions::new().read(true).write(false).open(path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };
    let mut file_string = String::new();
    match file.read_to_string(&mut file_string) {
    	Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
    	Ok(_) => println!("ok"),
    }

    let the_profile: Profile = json::decode(&file_string).unwrap();
    the_profile
}

pub fn read_account_path(path: &str) -> Profile {
    let display = "a";
    let mut file = match OpenOptions::new().read(true).write(false).open(path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };
    let mut file_string = String::new();
    match file.read_to_string(&mut file_string) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => println!("ok"),
    }

    let the_profile: Profile = json::decode(&file_string).unwrap();
    the_profile
}

pub fn make_app_root_dir(rootname: &str) {
	let mut the_home_dir = String::new();

	match env::home_dir() {
   		Some(ref p) => the_home_dir = p.display().to_string(),
   		None => println!("Impossible to get your home dir!")
	}

	let the_other_part = rootname;
	let the_full_path = the_home_dir + the_other_part;
	match fs::create_dir(&the_full_path) {
		Err(why) => { 
			println!("{:?}", why.kind()); 
		},
		Ok(_) => { 	
			println!("make it"); 
		},
	}
}  

pub fn add_to_app_root_dir(rootname: &str, filename: &str) {
	let mut the_home_dir = String::new();


	match env::home_dir() {
   		Some(ref p) => the_home_dir = p.display().to_string(),
   		None => println!("Impossible to get your home dir!")
	}
	
	let mut file_string = String::new();
	file_string.push_str(rootname);
	file_string.push_str(filename);

	let the_full_path = the_home_dir + &file_string;

	touch(&Path::new(&the_full_path)).unwrap_or_else(|why| {
               println!("! {:?}", why.kind());
    });
}  



pub fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().write(true).read(true).create(true).open(path) {
        Ok(_) => { 
        	println!("making {:?}", path);
        	Ok(()) },
        Err(e) => Err(e),
    }
}



//UI RELATED

pub fn append_column(title: &str, v: &mut Vec<gtk::TreeViewColumn>) {
    let l = v.len();
    let position = v.len();
    let renderer = gtk::CellRendererText::new().unwrap();

    v.push(gtk::TreeViewColumn::new().unwrap());
    let tmp = v.get_mut(l).unwrap();
    tmp.set_title(title);
    tmp.set_resizable(true);
    tmp.pack_start(&renderer, true);
    tmp.add_attribute(&renderer, "text", position as i32);

}
pub fn create_and_fill_model(list_store: Rc<RefCell<gtk::ListStore>>, alias: &str) {
    let mut top_level = gtk::TreeIter::new();

    list_store.borrow_mut().append(&mut top_level);
    list_store.borrow_mut().set_string(&top_level, 0, alias);
}

/*
pub fn create_and_fill_model(list_store: &mut gtk::ListStore, alias: &str) {
    let mut top_level = gtk::TreeIter::new();

    list_store.append(&mut top_level);
    list_store.set_string(&top_level, 0, alias);
}*/

