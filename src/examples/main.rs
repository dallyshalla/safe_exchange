extern crate gtk;
extern crate glib;
extern crate rustc_serialize;
extern crate exchangelib;
extern crate sodiumoxide;
extern crate secp256k1;
extern crate rand;
extern crate bitcoin;

use rand::{Rng, thread_rng};

use secp256k1::*;
use secp256k1::key::{PublicKey, SecretKey};

use bitcoin::util::base58::{self, FromBase58, ToBase58};
use bitcoin::util::{address, hash};

use sodiumoxide::crypto::hash::sha512;
use sodiumoxide::randombytes::randombytes_into;

use exchangelib::*;

use rustc_serialize::base64::{self, ToBase64};
use rustc_serialize::hex::FromHex;

use sodiumoxide::crypto;
//use std::f32;
use std::env;
//use std::io::prelude::*;
//use std::io;
use std::fs;
//use std::str;
//use std::fs::File;
//use std::fs::OpenOptions;
use std::path::Path;
use std::fs::metadata;
use std::rc::Rc;
use std::cell::RefCell;

use gtk::traits::*;
use gtk::signal::Inhibit;
use gtk::widgets::Builder;
use gtk::Window;


fn main() {
	gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

	let glade_src = include_str!("main.glade");
   let builder = Builder::new_from_string(glade_src).unwrap();	

      //login window1 and ui components
   let window1: Window = builder.get_object("window1").unwrap();
   let login_button: gtk::Button = builder.get_object("loginButton").unwrap();
	let new_button: gtk::Button = builder.get_object("newButton").unwrap();
   let username_entry: gtk::Entry = builder.get_object("usernameEntry").unwrap();
   let password_entry: gtk::Entry = builder.get_object("passwordEntry").unwrap();
   let window1_login_successful_clone = window1.clone();


      //register window2 and ui components
   let window2: Window = builder.get_object("window2").unwrap();
   let create_button: gtk::Button = builder.get_object("createButton").unwrap();
   let create_username_entry: gtk::Entry = builder.get_object("createUsernameEntry").unwrap();
   let create_password1_entry: gtk::Entry = builder.get_object("createPassword1Entry").unwrap();
   let create_password2_entry: gtk::Entry = builder.get_object("createPassword2Entry").unwrap();
   let create_alt_entry: gtk::Entry = builder.get_object("createAltEntry").unwrap();
   let create_status_label: gtk::Label = builder.get_object("statusLabel").unwrap();
   let window2_register_clone = window2.clone();
   //let window2_register_clone2 = window2.clone();  
   let window2_close_clone = window2.clone();
   let window2_close_clone2 = window2.clone();


      /* profile window3 and ui components */
   let window3: Window = builder.get_object("window3").unwrap();
   let username_label: gtk::Label = builder.get_object("usernameLabel").unwrap();
   let alias_scroll_box: gtk::ScrolledWindow = builder.get_object("aliasScrollBox").unwrap();
   let own_addres_scroll_box: gtk::ScrolledWindow = builder.get_object("ownAddressScrollBox").unwrap();
   let newalias_button: gtk::Button = builder.get_object("newaliasButton").unwrap();
   let messaging_button: gtk::Button = builder.get_object("messagingViewButton").unwrap();
   let exchange_button: gtk::Button = builder.get_object("exchangeViewButton").unwrap();
   let friends_button: gtk::Button = builder.get_object("friendsViewButton").unwrap();
   let wallet_button: gtk::Button = builder.get_object("walletViewButton").unwrap();
   let window3_panel_clone = window3.clone();

   /* window7 - exchange part of window3 profile  + UI components for exchange Buy, Sell, New */
   let window7: Window = builder.get_object("window7").unwrap();
   let newbuy_button: gtk::Button = builder.get_object("newbuyButton").unwrap();
   let newsell_button: gtk::Button = builder.get_object("newsellButton").unwrap();
   let newitem_button: gtk::Button = builder.get_object("newitemButton").unwrap();
   let trade_scrollwindow: gtk::ScrolledWindow = builder.get_object("tradeScrollWindow").unwrap();
   let exchangeitems_scrollwindow: gtk::ScrolledWindow = builder.get_object("exchangeyouritemsScrollWindow").unwrap();
   let exchangebalances_scrollwindow: gtk::ScrolledWindow = builder.get_object("exchangebalancesScrollWindow").unwrap();
   let window7_open_clone = window7.clone();
   let window7_close_clone = window7.clone();

   /* window9 part of window7 + UI components for Buy */
   let window9: Window = builder.get_object("window9").unwrap();
   let window9_open_clone = window9.clone();
   let window9_close_clone = window9.clone();
   /* Buttons */
   let plusone_button: gtk::Button = builder.get_object("plusoneBuyQuantityButton").unwrap();
   let plusten_button: gtk::Button = builder.get_object("plustenBuyQuantityButton").unwrap();
   let plushundred_button: gtk::Button = builder.get_object("plushundredBuyQuantityButton").unwrap();
   let plusonek_button: gtk::Button = builder.get_object("plusonekBuyQuantityButton").unwrap();
   let minus_quantity_buy_button: gtk::Button = builder.get_object("minusBuyQuantityButton").unwrap();
   let plus_quantity_buy_button: gtk::Button = builder.get_object("plusBuyQuantityButton").unwrap();
   let plusfive_buyprice_button: gtk::Button = builder.get_object("plusfiveBuyPriceButton").unwrap();
   let minusfive_buyprice_button: gtk::Button = builder.get_object("minusfiveBuyPriceButton").unwrap();
   let recprice_buy_button: gtk::Button = builder.get_object("recBuyPriceButton").unwrap();
   let minus_buyprice_button: gtk::Button = builder.get_object("minusBuyPriceButton").unwrap();
   let plus_buyprice_button: gtk::Button = builder.get_object("plusBuyPriceButton").unwrap();
   let confirm_buy_button: gtk::Button = builder.get_object("confirmBuyButton").unwrap();
   let buy_search_button: gtk::Button = builder.get_object("buysearchButton").unwrap();
   /* Entries */
   let buyquantity_entry: gtk::Entry = builder.get_object("buyquantityEntry").unwrap();
   let buyquantity_entry_clone1 = buyquantity_entry.clone();
   let buyquantity_entry_clone2 = buyquantity_entry.clone();
   let buyquantity_entry_clone3 = buyquantity_entry.clone();
   let buyquantity_entry_clone4 = buyquantity_entry.clone();
   let buyquantity_entry_clone5 = buyquantity_entry.clone();
   let buyquantity_entry_clone6 = buyquantity_entry.clone();
   let buyprice_entry: gtk::Entry = builder.get_object("buypriceEntry").unwrap();
   let buyprice_entry_clone1 = buyprice_entry.clone();
   let buyprice_entry_clone2 = buyprice_entry.clone();
   let buyprice_entry_clone3 = buyprice_entry.clone();
   let buyprice_entry_clone4 = buyprice_entry.clone();
   let total_buyprice_entry: gtk::Entry = builder.get_object("totalBuyPriceEntry").unwrap();
   let buy_search_entry: gtk::Entry = builder.get_object("buysearchEntry").unwrap();
   /* Labels */
   let rec_buyprice_label: gtk::Label = builder.get_object("recBuyPriceLabel").unwrap();
   let itemtitle_buy_label: gtk::Label = builder.get_object("itemtitleBuyLabel").unwrap();
   /* ScrolledWindow */
   let buysearch_scrollwindow: gtk::ScrolledWindow = builder.get_object("buysearchresultScrollWindow").unwrap();   
   let buybalances_scrollwindow: gtk::ScrolledWindow = builder.get_object("balancesBuyScrollWindow").unwrap();
   let buyattachments_scrollwindow: gtk::ScrolledWindow = builder.get_object("buyattachedtoitemScrollWindow").unwrap();
   
//   let buy_description_text_view: gtk::TextView = builder.get_object("buyDescriptionTextView").unwrap();
   
   /* window10 part of window7 + UI components for Sell */
   let window10: Window = builder.get_object("window10").unwrap();
   let window10_open_clone = window10.clone();
   let window10_close_clone = window10.clone();
   /* window10 Buttons */
   let sellconfirm_button: gtk::Button = builder.get_object("sellconfirmButton").unwrap();
   let sellminusquantity_button: gtk::Button = builder.get_object("sellminusquantityButton").unwrap();
   let sellplusquantity_button: gtk::Button = builder.get_object("sellplusquantityButton").unwrap();
   let sellonequantity_button: gtk::Button = builder.get_object("sellonequantityButton").unwrap();
   let selltenquantity_button: gtk::Button = builder.get_object("selltenquantityButton").unwrap();
   let sellhundredquantity_button: gtk::Button = builder.get_object("sellhundredquantityButton").unwrap();
   let sellallquantity_button: gtk::Button = builder.get_object("sellallquantityButton").unwrap();
   let sellplusprice_button: gtk::Button = builder.get_object("sellpluspriceButton").unwrap();
   let sellminusprice_button: gtk::Button = builder.get_object("sellminuspriceButton").unwrap();
   let sellplusfive_button: gtk::Button = builder.get_object("sellplusfiveButton").unwrap();
   let sellminusfive_button: gtk::Button = builder.get_object("sellminusfiveButton").unwrap();
   let sellrecprice_button: gtk::Button = builder.get_object("sellrecpriceButton").unwrap();
   /* window10 Entries */
   let sellquantity_entry: gtk::Entry = builder.get_object("sellquantityEntry").unwrap();
   let sellquantity_entry_clone1 = sellquantity_entry.clone();
   let sellquantity_entry_clone2 = sellquantity_entry.clone();
   let sellquantity_entry_clone3 = sellquantity_entry.clone();
   let sellquantity_entry_clone4 = sellquantity_entry.clone();
   let sellquantity_entry_clone5 = sellquantity_entry.clone();
   let sellquantity_entry_clone6 = sellquantity_entry.clone();
   let sellquantity_entry_clone7 = sellquantity_entry.clone();
   let sellquantity_entry_clone8 = sellquantity_entry.clone();
   let sellquantity_entry_clone9 = sellquantity_entry.clone();

   let sellprice_entry: gtk::Entry = builder.get_object("sellpriceEntry").unwrap();
   let sellprice_entry_clone1 = sellprice_entry.clone();
   let sellprice_entry_clone2 = sellprice_entry.clone();
   let sellprice_entry_clone3 = sellprice_entry.clone();
   let sellprice_entry_clone4 = sellprice_entry.clone();
   let sellprice_entry_clone5 = sellprice_entry.clone();
   let sellprice_entry_clone6 = sellprice_entry.clone();
   let sellprice_entry_clone7 = sellprice_entry.clone();
   let sellprice_entry_clone8 = sellprice_entry.clone();
   let selltotal_entry: gtk::Entry = builder.get_object("selltotalEntry").unwrap();
   let selltotal_entry_clone1 = selltotal_entry.clone();
   let selltotal_entry_clone2 = selltotal_entry.clone();
   let selltotal_entry_clone3 = selltotal_entry.clone();
   let selltotal_entry_clone4 = selltotal_entry.clone();
   let selltotal_entry_clone5 = selltotal_entry.clone();
   let selltotal_entry_clone6 = selltotal_entry.clone();
   let selltotal_entry_clone7 = selltotal_entry.clone();
   let selltotal_entry_clone8 = selltotal_entry.clone();
   let selltotal_entry_clone9 = selltotal_entry.clone();
   let selltotal_entry_clone10 = selltotal_entry.clone();
   /* window10 TextView */
   let selldescription_text: gtk::TextView = builder.get_object("selldescriptionTextView").unwrap();
   /* window10 ScrolledWindows */
   let sellyouritems_scroll: gtk::ScrolledWindow = builder.get_object("sellyouritemsScrollWindow").unwrap();
   let sellbalances_scroll: gtk::ScrolledWindow = builder.get_object("sellyourbalancesScrollWindow").unwrap();
   /* window10 Labels */
   let sellname_label: gtk::Label = builder.get_object("sellnameLabel").unwrap();
   let sellprice_label: gtk::Label = builder.get_object("sellrecpriceLabel").unwrap();


   /* window11 part of window7 + UI components for making New Items */
   let window11: Window = builder.get_object("window11").unwrap();
   let window11_open_clone = window11.clone();
   let window11_close_clone = window11.clone();
      /* Buttons */
   let newitem_addimage_button: gtk::Button = builder.get_object("newitemAddImageButton").unwrap();
   let newitem_add_button: gtk::Button = builder.get_object("newitemAddButton").unwrap();
   let newitem_imageremove_button: gtk::Button = builder.get_object("newitemRemoveImageButton").unwrap();
      /* Entries */
   let newitem_name_entry: gtk::Entry = builder.get_object("newitemNameEntry").unwrap();
   let newitem_quantity_entry: gtk::Entry = builder.get_object("newitemQuantityEntry").unwrap();
   let newitem_recprice_entry: gtk::Entry = builder.get_object("newitemRecPriceEntry").unwrap();
   let newitem_keyword_entry: gtk::Entry = builder.get_object("newitemKeywordsEntry").unwrap();
   let newitem_brandname_entry: gtk::Entry = builder.get_object("newitemBrandNameEntry").unwrap();
   let newitem_description_entry: gtk::Entry = builder.get_object("newitemDescriptionEntry").unwrap();
   let newitem_contact_entry: gtk::Entry = builder.get_object("newitemContactEntry").unwrap();
   /* ScrolledWindows */
   let newitem_images_scrollwindow: gtk::ScrolledWindow = builder.get_object("newitemImagesListScrollWindow").unwrap();



   /* window8 new alias - appends a new alias with bitcoin, safecoin, keys, and items vector */
   let window8: Window = builder.get_object("window8").unwrap();
   let window8_open_clone = window8.clone();
   let window8_close_clone = window8.clone();
   /* window8 confirm alias button */
   let confirmnewalias_button: gtk::Button = builder.get_object("confirmaliasButton").unwrap();
   /* window8 new alias name entry */
   let newalias_name_entry: gtk::Entry = builder.get_object("newaliasnameEntry").unwrap();
   /* window8 new alias creation status label */
   let newalias_status_label: gtk::Label = builder.get_object("newaliasstatusLabel").unwrap();
   let imagecreation_list: Vec<String> = Vec::new();
   let imagelist_cell = Rc::new(RefCell::new(imagecreation_list));
   let imagelist_cell_clone1 = imagelist_cell.clone();

   let mut profile_global = Profile::new();
   let profile_cell = Rc::new(RefCell::new(profile_global));
   let profilecell_clone2 = profile_cell.clone();
   let profilecell_clone3 = profile_cell.clone();
   let profilecell_clone4 = profile_cell.clone();
   let profilecell_clone5 = profile_cell.clone();
   let profilecell_clone6 = profile_cell.clone();
   let profilecell_clone7 = profile_cell.clone();
   let profilecell_clone8 = profile_cell.clone();
   let profilecell_clone9 = profile_cell.clone();

   //let mut buysearch_items_found = Vec::new();
   //let buysearch_items_found_cell = Rc::new(RefCell::new(buysearch_items_found));
   //let buysearch_items_found_clone1 = buysearch_items_found_cell.clone();

   let mut exchange_alias = Alias::new();
   let exchange_alias_cell = Rc::new(RefCell::new(exchange_alias));
   let exchange_alias_cellclone1 = exchange_alias_cell.clone();
   let exchange_alias_cellclone2 = exchange_alias_cell.clone();
   let exchange_alias_cellclone3 = exchange_alias_cell.clone();
   let exchange_alias_cellclone4 = exchange_alias_cell.clone();

   let mut exchange_item = Item::new();
   let exchange_item_cell = Rc::new(RefCell::new(exchange_item));
   let exchange_item_cell_clone1 = exchange_item_cell.clone();


   let selected_alias_window3 = 0;
   let selected_alias_cell = Rc::new(RefCell::new(selected_alias_window3));
   let selected_alias_cell_clone1 = selected_alias_cell.clone();
   let selected_alias_cell_clone2 = selected_alias_cell.clone();
   let selected_alias_cell_clone3 = selected_alias_cell.clone();

   let selected_item_window7 = 0;
   let selected_item_window7cell = Rc::new(RefCell::new(selected_item_window7));
   let selected_item_cell_window7clone1 = selected_item_window7cell.clone();
   let selected_item_cell_window7clone2 = selected_item_window7cell.clone();

   let selected_item_window10 = 0;
   let selected_item_window10cell = Rc::new(RefCell::new(selected_item_window10));
   let selected_item_cell_window10clone1 = selected_item_window10cell.clone();
   let selected_item_cell_window10clone2 = selected_item_window10cell.clone();

   let selecteditem_window10_quantity = 0;
   let selecteditem_window10_quantity_cell = Rc::new(RefCell::new(selecteditem_window10_quantity));
   let selecteditem_window10_quantity_cell_clone1 = selecteditem_window10_quantity_cell.clone();
   let selecteditem_window10_quantity_cell_clone2 = selecteditem_window10_quantity_cell.clone();
   let selecteditem_window10_quantity_cell_clone3 = selecteditem_window10_quantity_cell.clone();
   let selecteditem_window10_quantity_cell_clone4 = selecteditem_window10_quantity_cell.clone();
   let selecteditem_window10_quantity_cell_clone5 = selecteditem_window10_quantity_cell.clone();
   let selecteditem_window10_quantity_cell_clone6 = selecteditem_window10_quantity_cell.clone();
   let selecteditem_window10_quantity_cell_clone7 = selecteditem_window10_quantity_cell.clone();

   let window10_quantity = 0;
   let window10_quantity_cell = Rc::new(RefCell::new(window10_quantity));
   let window10_quantity_cell_clone1 = window10_quantity_cell.clone();
   let window10_quantity_cell_clone2 = window10_quantity_cell.clone();
   let window10_quantity_cell_clone3 = window10_quantity_cell.clone();
   let window10_quantity_cell_clone4 = window10_quantity_cell.clone();
   let window10_quantity_cell_clone5 = window10_quantity_cell.clone();
   let window10_quantity_cell_clone6 = window10_quantity_cell.clone();
   let window10_quantity_cell_clone7 = window10_quantity_cell.clone();

   let window10_price = 0.00;
   let window10_price_cell = Rc::new(RefCell::new(window10_price));
   let window10_price_cell_clone1 = window10_price_cell.clone();
   let window10_price_cell_clone2 = window10_price_cell.clone();
   let window10_price_cell_clone3 = window10_price_cell.clone();

   let window10_recprice = 0.00;
   let window10_recprice_cell = Rc::new(RefCell::new(window10_recprice));
   let window10_recprice_cell_clone1 = window10_recprice_cell.clone();

   let mut window10exchange_item = Item::new();
   let window10exchange_item_cell = Rc::new(RefCell::new(window10exchange_item));
   let window10exchange_item_cell_clone1 = window10exchange_item_cell.clone();

    //let mut s = Rc::new(RefCell::new(Vec::new()));
      //let v = s.clone();
      //let z = s.clone(); 
      //let mut s2 = Rc::new(RefCell::new(Vec::new()));
      //let v2 = s2.clone();
      //let z2 = s2.clone(); 
      //let reser = Arc::new(Mutex::new(Vec::new()));

  	make_app_root_dir("/.test_root");
   /* window9 BuyTHings Buttons */
   plusone_button.connect_clicked(move |_| {
      let mut increase_this: i64 = buyquantity_entry.get_text().unwrap().parse().ok().expect("need number");
      increase_this += 1;
      buyquantity_entry.set_text(&increase_this.to_string());
      //need to get and parse text, then set it based on the button's role
   });  

   /* window9 BuyTHings Buttons */
   plusten_button.connect_clicked(move |_| {
      let mut increase_this: i64 = buyquantity_entry_clone1.get_text().unwrap().parse().ok().expect("need number");
      increase_this += 10;
      buyquantity_entry_clone1.set_text(&increase_this.to_string());
   });

      /* window9 BuyTHings Buttons */
   plushundred_button.connect_clicked(move |_| {
      let mut increase_this: i64 = buyquantity_entry_clone2.get_text().unwrap().parse().ok().expect("need number");
      increase_this += 100;
      buyquantity_entry_clone2.set_text(&increase_this.to_string());
   });

      /* window9 BuyTHings Buttons */
   plusonek_button.connect_clicked(move |_| {
      let mut increase_this: i64 = buyquantity_entry_clone3.get_text().unwrap().parse().ok().expect("need number");
      increase_this += 1000;
      buyquantity_entry_clone3.set_text(&increase_this.to_string());
   });

      /* window9 BuyTHings Buttons */
   minus_quantity_buy_button.connect_clicked(move |_| {
      let mut increase_this: i64 = buyquantity_entry_clone4.get_text().unwrap().parse().ok().expect("need number");
      increase_this -= 1;
      buyquantity_entry_clone4.set_text(&increase_this.to_string());
   });

      /* window9 BuyTHings Buttons */
   plus_quantity_buy_button.connect_clicked(move |_| {
      let mut increase_this: i64 = buyquantity_entry_clone5.get_text().unwrap().parse().ok().expect("need number");
      increase_this += 1;
      buyquantity_entry_clone5.set_text(&increase_this.to_string());
   });

      /* window9 BuyTHings Buttons */
   plusfive_buyprice_button.connect_clicked(move |_| {
      let mut increase_this: f64 = buyprice_entry_clone1.get_text().unwrap().parse().ok().expect("need number");
      increase_this *= 1.05;
      buyprice_entry_clone1.set_text(&increase_this.to_string());
   });

      /* window9 BuyTHings Buttons */
   minusfive_buyprice_button.connect_clicked(move |_| {
      let mut increase_this: f64 = buyprice_entry_clone2.get_text().unwrap().parse().ok().expect("need number");
      increase_this *= 0.95;
      if increase_this > 0.00 {
         buyprice_entry_clone2.set_text(&increase_this.to_string());
      }
   });

      /* window9 BuyTHings Buttons */
   recprice_buy_button.connect_clicked(move |_| {
      //take from the exchange profile the rec price and set the label and price enxtry.
   });

      /* window9 BuyTHings Buttons */
   minus_buyprice_button.connect_clicked(move |_| {
      let mut increase_this: f64 = buyprice_entry_clone3.get_text().unwrap().parse().ok().expect("need number");
      increase_this -= 1.0;
      if increase_this > 0.00 {
         buyprice_entry_clone3.set_text(&increase_this.to_string());
      }
   });

      /* window9 BuyTHings Buttons */
   plus_buyprice_button.connect_clicked(move |_| {
      let mut increase_this: f64 = buyprice_entry_clone4.get_text().unwrap().parse().ok().expect("need number");
      increase_this += 1.0;
      if increase_this > 0.00 {
         buyprice_entry_clone4.set_text(&increase_this.to_string());
      }
   });

   /* window10 Sell Quantity Buttons */
   sellminusquantity_button.connect_clicked(move |_| {
      let mut decrease_this: i64 = sellquantity_entry.get_text().unwrap().parse().ok().expect("need number");
      decrease_this -= 1;
      //have to read the profile get the item list, check the current amount in the quantity box;
      //increase only if there is enough between the quantity box and the amount in the item array.
      if decrease_this >= 0 {
         sellquantity_entry.set_text(&decrease_this.to_string());
      }
   });
   /* window10 Sell Quantity Buttons */
   sellplusquantity_button.connect_clicked(move |_| {
      let mut increase_this: i64 = sellquantity_entry_clone2.get_text().unwrap().parse().ok().expect("not number");
      if (*selecteditem_window10_quantity_cell_clone2.borrow()) >= (increase_this + 1) {
         increase_this += 1;
         sellquantity_entry_clone2.set_text(&increase_this.to_string());
      }

   });
   /* window10 Sell Quantity Buttons */
   sellonequantity_button.connect_clicked(move |_| {
      let mut increase_this: i64 = sellquantity_entry_clone3.get_text().unwrap().parse().ok().expect("not number");
      if (*selecteditem_window10_quantity_cell_clone3.borrow()) >= (increase_this + 1) {
         increase_this += 1;
         sellquantity_entry_clone3.set_text(&increase_this.to_string());
      }
   });
   /* window10 Sell Quantity Buttons */
   selltenquantity_button.connect_clicked(move |_| {
      let mut increase_this: i64 = sellquantity_entry_clone4.get_text().unwrap().parse().ok().expect("not number");
      if (*selecteditem_window10_quantity_cell_clone4.borrow()) >= (increase_this + 10) {
         increase_this += 10;
         sellquantity_entry_clone4.set_text(&increase_this.to_string());
      }
   });
   /* window10 Sell Quantity Buttons */
   sellhundredquantity_button.connect_clicked(move |_| {
      let mut increase_this: i64 = sellquantity_entry_clone5.get_text().unwrap().parse().ok().expect("not number");
      if (*selecteditem_window10_quantity_cell_clone5.borrow()) >= (increase_this + 100) {
         increase_this += 100;
         sellquantity_entry_clone5.set_text(&increase_this.to_string());
      }
   });
   /* window10 Sell Quantity Buttons */
   sellallquantity_button.connect_clicked(move |_| {
      sellquantity_entry_clone6.set_text(&(*selecteditem_window10_quantity_cell_clone6.borrow()).to_string());
   });
   /* window10 Sell price Buttons */
   sellplusprice_button.connect_clicked(move |_| {
      let mut current_price: f64 = sellprice_entry_clone1.get_text().unwrap().parse().ok().expect("not right number");
      current_price += 1.00;
      sellprice_entry_clone1.set_text(&current_price.to_string());
   });
   /* window10 Sell price Buttons */
   sellminusprice_button.connect_clicked(move |_| {
      let mut current_price: f64 = sellprice_entry_clone2.get_text().unwrap().parse().ok().expect("not right number");
      current_price -= 1.00;
      if current_price > 0.00 {
         sellprice_entry_clone2.set_text(&current_price.to_string());
      }
   });
   /* window10 Sell price Buttons */
   sellplusfive_button.connect_clicked(move |_| {
      let mut current_price: f64 = sellprice_entry_clone3.get_text().unwrap().parse().ok().expect("not right number");
      current_price *= 1.05;
      if current_price > 0.00 {
         sellprice_entry_clone3.set_text(&current_price.to_string());
      }
   });
   /* window10 Sell price Buttons */
   sellminusfive_button.connect_clicked(move |_| {
      let mut current_price: f64 = sellprice_entry_clone4.get_text().unwrap().parse().ok().expect("not right number");
      current_price *= 0.95;
      if current_price > 0.00 {
         sellprice_entry_clone4.set_text(&current_price.to_string());
      }
   });
   /* window10 Sell price Buttons */
   sellrecprice_button.connect_clicked(move |_| {
      let the_recprice = window10_recprice_cell.borrow();
      sellprice_entry_clone5.set_text(&the_recprice.to_string());
   });

      /* window9 BuyTHings Buttons */
   confirm_buy_button.connect_clicked(move |_| {

      //write to account deducting the amount from balances, 
      //increase buy orders, generate addresses for the buy offer. this is where coins will be stored.


   });

   let searchtobuy_tree_view = gtk::TreeView::new().unwrap();
   let searchtobuy_treeview_clone = searchtobuy_tree_view.clone();

   let searchtobuy_liststore = Rc::new(RefCell::new(gtk::ListStore::new(&[glib::Type::String, glib::Type::String, glib::Type::String, glib::Type::String]).unwrap()));
   let searchtobuy_liststore_clone = searchtobuy_liststore.clone();
   let searchtobuy_model = searchtobuy_liststore.borrow().get_model().unwrap();
   let searchtobuy_model_clone = Rc::new(RefCell::new(searchtobuy_model));

   let mut searchtobuy_of_columns: Vec<gtk::TreeViewColumn> = Vec::new();

   append_column("name", &mut searchtobuy_of_columns);
   append_column("quantity", &mut searchtobuy_of_columns);
   append_column("owner", &mut searchtobuy_of_columns);
   append_column("price", &mut searchtobuy_of_columns);

   for i in searchtobuy_of_columns {
      searchtobuy_tree_view.append_column(&i);
   }
   searchtobuy_tree_view.set_model(&searchtobuy_liststore.borrow().get_model().unwrap());

   buysearch_scrollwindow.add(&searchtobuy_tree_view);
      /* window9 BuyTHings Buttons */
   buy_search_button.connect_clicked(move |_| {

      searchtobuy_liststore.borrow_mut().clear();
      let search_text = buy_search_entry.get_text().unwrap();

      let mut the_home_dir = String::new();
      match env::home_dir() {
         Some(ref p) => the_home_dir = p.display().to_string(),
         None => println!("Impossible to get your home dir!")
      }
      let path_string = String::from("/.test_root/");
      let path_string3 = the_home_dir + &path_string;
      let path = Path::new(&path_string3);

      let paths = fs::read_dir(path).unwrap();
      

      for path in paths {
         let extension = path.unwrap().path();
         let str_path = extension.to_str().unwrap();
         let the_profile = read_account_path(str_path);
         for alias in the_profile.alias {
            for offer in alias.selloffers {
               if offer.item.name.contains(&search_text) {
               //println!("{:?}", offer);
               let mut top_level = gtk::TreeIter::new();
               searchtobuy_liststore.borrow_mut().append(&mut top_level);
               searchtobuy_liststore.borrow_mut().set_string(&top_level, 0, &offer.item.name.to_string()); 
               searchtobuy_liststore.borrow_mut().set_string(&top_level, 1, &offer.quantity.to_string()); 
               searchtobuy_liststore.borrow_mut().set_string(&top_level, 2, &offer.alias_owner.to_string());
               searchtobuy_liststore.borrow_mut().set_string(&top_level, 3, &offer.price.to_string()); 
            }

            }
         }
      }
      searchtobuy_tree_view.set_model(&searchtobuy_liststore.borrow().get_model().unwrap());

      //open the user account, look through the items if they match in name with the search text, then add them to the vector and include the name of the alias
   });

   /* window1 button */
  	new_button.connect_clicked(move |_| {
  		window2.show_all();
  	});

   /* window7 treeview, treemodel, liststore for your item list */
   //let items_vec = Rc::new(RefCell::new(Vec::new()));

   let exchangeitems_tree_view = gtk::TreeView::new().unwrap();
   let exchangeitems_treeview_clone = exchangeitems_tree_view.clone();

   let exchangeitems_liststore = Rc::new(RefCell::new(gtk::ListStore::new(&[glib::Type::String, glib::Type::USize]).unwrap()));
   let exchangeitems_liststore_clone = exchangeitems_liststore.clone();
   let exchangeitem_model = exchangeitems_liststore.borrow().get_model().unwrap();
   let exchangeitem_model_clone = Rc::new(RefCell::new(exchangeitem_model));

   let mut vec_of_columns: Vec<gtk::TreeViewColumn> = Vec::new();

   append_column("name", &mut vec_of_columns);
   append_column("quantity", &mut vec_of_columns);


   for i in vec_of_columns {
      exchangeitems_tree_view.append_column(&i);
   }

   //let mobile_model_clone2 = mobile_model2.clone();
      //get selected alias and then choose selected alias from profile list of aliases
      //then go through the items vector and put them into the model
      //add the model to exchangeyouritemsScrollWindow
      //fill window 7 scroll boxes with the corresponding item ist.
   /* window3 exchange open button */

   fn append_column(title: &str, v: &mut Vec<gtk::TreeViewColumn>) {
      let l = v.len();
      let renderer = gtk::CellRendererText::new().unwrap();

      v.push(gtk::TreeViewColumn::new().unwrap());
      let tmp = v.get_mut(l).unwrap();

      tmp.set_title(title);
      tmp.set_resizable(true);
      tmp.pack_start(&renderer, true);
      tmp.add_attribute(&renderer, "text", l as i32);
   }

   /* window3 exchange open button */
   /* on macintosh has to always set the treeview model well before you actually use it */

   let exchangeoffers_tree_view = gtk::TreeView::new().unwrap();
   let exchangeoffers_treeview_clone = exchangeoffers_tree_view.clone();

   let exchangeoffers_liststore = Rc::new(RefCell::new(gtk::ListStore::new(&[glib::Type::String, glib::Type::String, glib::Type::String]).unwrap()));
   let exchangeoffers_liststore_clone = exchangeoffers_liststore.clone();
   let exchangeoffers_model = exchangeoffers_liststore.borrow().get_model().unwrap();
   let exchangeoffers_model_clone = Rc::new(RefCell::new(exchangeoffers_model));
   exchangeitems_tree_view.set_model(&exchangeitems_liststore.borrow().get_model().unwrap());

   let mut vec_of_offerscolumns: Vec<gtk::TreeViewColumn> = Vec::new();

   append_column("name", &mut vec_of_offerscolumns);
   append_column("price", &mut vec_of_offerscolumns);
   append_column("quantity", &mut vec_of_offerscolumns);


   for i in vec_of_offerscolumns {
      exchangeoffers_tree_view.append_column(&i);
   }

   /* window3 exchange open button */
   exchange_button.connect_clicked(move |_| {

      exchangeitems_liststore.borrow_mut().clear();
      exchangeoffers_liststore_clone.borrow_mut().clear();
      let alias_index = (*selected_alias_cell_clone2.borrow()) as usize;
      let mut the_actual_profile = read_account(&profilecell_clone6.borrow().username);
      let profile_clone = the_actual_profile.clone();
      //Slet l = items_vec.borrow().len();

      


      /*if l < 1 as usize {
         let position = items_vec.borrow().len();
         let renderer = gtk::CellRendererText::new().unwrap();

         items_vec.borrow_mut().push(gtk::TreeViewColumn::new().unwrap());
         let mut tmp = items_vec.borrow_mut();
         tmp[0].set_title("items");
         tmp[0].set_resizable(true);
         tmp[0].pack_start(&renderer, true);
         tmp[0].add_attribute(&renderer, "text", 0 as i32);
         tmp.push(gtk::TreeViewColumn::new().unwrap());
         let renderer = gtk::CellRendererText::new().unwrap();

         tmp[1].set_title("quantity");
         tmp[1].set_resizable(true);
         tmp[1].pack_start(&renderer, true);
         tmp[1].add_attribute(&renderer, "text", 1 as i32);
         exchangeitems_tree_view.append_column(&tmp[0]);
         exchangeitems_tree_view.append_column(&tmp[1]);

      }  */   
     // println!("{:?}", items_vec.borrow().len().to_string());
      
     // println!("here -------------------- {:?}", items_vec.borrow().len().to_string());

      let item_vec = &the_actual_profile.alias[alias_index].items;
      for item in item_vec {
         println!("{:?}", &item);    
         let mut top_level = gtk::TreeIter::new();
         
         let mut val1 = glib::Value::new();
         val1.init(glib::Type::ISize);      
          val1.set_long(item.quantity as i64);
         exchangeitems_liststore.borrow_mut().append(&mut top_level);
         exchangeitems_liststore.borrow_mut().set_string(&top_level, 0, &item.name); 
         exchangeitems_liststore.borrow_mut().set_value(&top_level, 1, &val1); 
      }
      println!("here -------------------- ");

      exchangeitems_tree_view.set_model(&exchangeitems_liststore.borrow().get_model().unwrap());
            println!("here -------------------- ");

      exchangeitems_scrollwindow.add(&exchangeitems_tree_view);
            println!("here -------------------- ");

      let alias_clone = &profile_clone.alias[alias_index];
      let an_alias = alias_clone.clone();
      let mut bpuk: Vec<String> = Vec::new();
      for bpuks in &an_alias.bitcoin_public_keys {
         bpuk.push(bpuks.to_string());
      }
      let mut bprk: Vec<String> = Vec::new();
      for bprks in &an_alias.bitcoin_private_keys {
         bprk.push(bprks.to_string());
      }
      let mut spuk: Vec<String> = Vec::new();
      for spuks in an_alias.safecoin_public_keys {
         spuk.push(spuks);
      }
      let mut sprk: Vec<String> = Vec::new();
      for sprks in an_alias.safecoin_private_keys {
         sprk.push(sprks);
      }
      let mut item: Vec<Item> = Vec::new();
      for items in an_alias.items {
         item.push(items);
      }
      let mut boff: Vec<BuyOffer> = Vec::new();
      for boffs in an_alias.buyoffers {
         let mut top_level = gtk::TreeIter::new();
         /*let mut val1 = glib::Value::new();
         val1.init(glib::Type::F64);
         val1.set(boffs.price);
         let mut val2 = glib::Value::new();
         val2.init(glib::Type::I64);
         val2.set_long(boffs.price as i64);*/
         exchangeoffers_liststore.borrow_mut().append(&mut top_level);
         exchangeoffers_liststore.borrow_mut().set_string(&top_level, 0, &boffs.item.name); 
         exchangeoffers_liststore.borrow_mut().set_string(&top_level, 1, &boffs.price.to_string()); 
         exchangeoffers_liststore.borrow_mut().set_string(&top_level, 2, &boffs.quantity.to_string()); 

         boff.push(boffs);
      }
      let mut soff: Vec<SellOffer> = Vec::new();
      for soffs in an_alias.selloffers {
         let mut top_level = gtk::TreeIter::new();
         /*let mut val1 = glib::Value::new();
         val1.init(glib::Type::F64);
         val1.set(soffs.price);
         let mut val2 = glib::Value::new();
         val2.init(glib::Type::I64);
         val2.set_long(soffs.price as i64);*/
         exchangeoffers_liststore.borrow_mut().append(&mut top_level);
         exchangeoffers_liststore.borrow_mut().set_string(&top_level, 0, &soffs.item.name); 
         exchangeoffers_liststore.borrow_mut().set_string(&top_level, 1, &soffs.price.to_string()); 
         exchangeoffers_liststore.borrow_mut().set_string(&top_level, 2, &soffs.quantity.to_string()); 
         soff.push(soffs);
      }
      let mut itemtxns: Vec<ItemTransaction> = Vec::new();
      for txn in an_alias.item_trans {
         itemtxns.push(txn);
      }
      let mut cointxns: Vec<CoinTransaction> = Vec::new();
      for txn in an_alias.coin_trans {
         cointxns.push(txn);
      }
      let mut friendlist: Vec<Friend> = Vec::new();
      for frnd in an_alias.friends {
         friendlist.push(frnd);
      }
      exchangeoffers_tree_view.set_model(&exchangeoffers_liststore.borrow().get_model().unwrap());
      trade_scrollwindow.add(&exchangeoffers_tree_view);
      exchange_alias_cell.borrow_mut().name = an_alias.name.to_string();
      exchange_alias_cell.borrow_mut().bitcoin_public_keys = bpuk;
      exchange_alias_cell.borrow_mut().bitcoin_private_keys = bprk;
      exchange_alias_cell.borrow_mut().safecoin_public_keys = spuk;
      exchange_alias_cell.borrow_mut().safecoin_private_keys = sprk;
      exchange_alias_cell.borrow_mut().items = item;
      exchange_alias_cell.borrow_mut().buyoffers = boff;
      exchange_alias_cell.borrow_mut().selloffers = soff;
      exchange_alias_cell.borrow_mut().public_key = "".to_string();
      exchange_alias_cell.borrow_mut().private_key = "".to_string();
      exchange_alias_cell.borrow_mut().item_trans = itemtxns;
      exchange_alias_cell.borrow_mut().coin_trans = cointxns;
      exchange_alias_cell.borrow_mut().friends = friendlist;
      exchange_alias_cell.borrow_mut().mock_balance = an_alias.mock_balance;

      let name_bytes = an_alias.name.as_bytes();

      let sha512::Digest(name) =  crypto::hash::sha512::hash(&name_bytes);
      let hex = name.to_vec();
      let s1 = to_hex_string(hex);

      let mut s = Secp256k1::new();
      s.randomize(&mut thread_rng());


      let (sk, pk) = s.generate_keypair(&mut thread_rng()).unwrap();

      let mut msg = [0u8; 32];
      thread_rng().fill_bytes(&mut msg);
      let msg = Message::from_slice(&msg).unwrap();

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
      //println!("{:?}", s1.from_hex().unwrap().to_base64(base64::STANDARD));



      //exchange_alias_cell.borrow_mut().buyoffers = buyoffer;
      //exchange_alias_cell.borrow_mut().selloffers = selloffer;



     // *exchange_alias_cell.borrow_mut() = &profile_clone.alias[alias_index];
      window7_open_clone.show_all();

   });
   fn to_hex_string(bytes: Vec<u8>) -> String {
      let strs: Vec<String> = bytes.iter()
                               .map(|b| format!("{:02X}", b))
                               .collect();
      strs.join(" ")
   }
   /* window3 new alias button */
   newalias_button.connect_clicked(move |_| {
      window8_open_clone.show_all();
   });

   /* window3 wallet management button */
   wallet_button.connect_clicked(move |_| {

   });

   /* window3 messaging button */
   messaging_button.connect_clicked(move |_| {

   });

   /* window3 friends button - managing contacts and relationships */
   friends_button.connect_clicked(move |_| {

   });

   /* window7 buy item button - search and buy items */
   newbuy_button.connect_clicked(move |_| {
      window9_open_clone.show_all();
   });

   /* window10 youritems sell module */
   //let sellitems_vec = Rc::new(RefCell::new(Vec::new()));

   let sellexchangeitems_tree_view = gtk::TreeView::new().unwrap();
   let sellexchangeitems_treeview_clone = sellexchangeitems_tree_view.clone();

   let sellexchangeitems_liststore = Rc::new(RefCell::new(gtk::ListStore::new(&[glib::Type::String, glib::Type::USize]).unwrap()));

   let sellexchangeitem_model = sellexchangeitems_liststore.borrow().get_model().unwrap();
   let sellexchangeitem_model_clone = Rc::new(RefCell::new(sellexchangeitem_model));

   let mut vec_of_columns_sell_items: Vec<gtk::TreeViewColumn> = Vec::new();

   append_column("name", &mut vec_of_columns_sell_items);
   append_column("quantity", &mut vec_of_columns_sell_items);


   for i in vec_of_columns_sell_items {
      sellexchangeitems_tree_view.append_column(&i);
   }

   /* window7 sell item button - issue an item you own for sale */
   newsell_button.connect_clicked(move |_| {
      sellexchangeitems_liststore.borrow_mut().clear();

      //let the_selected_index: usize = *selected_item_cell_window7clone1.borrow() as usize;
      //extracting here the item and populating it into the exchange item cell so be consumed in the sell item if there was a selection.
     
      //println!("{:?}", exchange_alias_cellclone1.borrow().items[the_selected_index].name);
      
      let sellitem_vec = &exchange_alias_cellclone1.borrow().items;
      for item in sellitem_vec {
         println!("{:?}", &item);    
         let mut top_level = gtk::TreeIter::new();
         let mut val1 = glib::Value::new();
         val1.init(glib::Type::ISize);      
         val1.set_long(item.quantity as i64);
         sellexchangeitems_liststore.borrow_mut().append(&mut top_level);
         sellexchangeitems_liststore.borrow_mut().set_string(&top_level, 0, &item.name);         
         sellexchangeitems_liststore.borrow_mut().set_value(&top_level, 1, &val1); 

      }

      sellexchangeitems_tree_view.set_model(&sellexchangeitems_liststore.borrow().get_model().unwrap());
      sellyouritems_scroll.add(&sellexchangeitems_tree_view);

       /*
      let exchange_alias = &exchange_alias_cellclone1.borrow().items[paths_int];
      let exchange_alias_clone = exchange_alias;
      let item_name = &exchange_alias_clone.name;
        
      exchange_item_cell.borrow_mut().name = item_name;
      exchange_item_cell.borrow_mut().description = exchange_alias_cellclone1.borrow().items[paths_int].description;
      exchange_item_cell.borrow_mut().quantity = exchange_alias_cellclone1.borrow().items[paths_int].quantity;
      exchange_item_cell.borrow_mut().recommended_price = exchange_alias_cellclone1.borrow().items[paths_int].recommended_price;
      exchange_item_cell.borrow_mut().contact = exchange_alias_cellclone1.borrow().items[paths_int].contact;
      exchange_item_cell.borrow_mut().brand_name = exchange_alias_cellclone1.borrow().items[paths_int].brand_name;
      exchange_item_cell.borrow_mut().keywords = exchange_alias_cellclone1.borrow().items[paths_int].keywords;
      exchange_item_cell.borrow_mut().images = exchange_alias_cellclone1.borrow().items[paths_int].images;
      exchange_item_cell.borrow_mut().alias_owner = exchange_alias_cellclone1.borrow().items[paths_int].alias_owner;
*/
      window10_open_clone.show_all();
   });

   /* window7 new item button - new item creation */
   newitem_button.connect_clicked(move |_| {
      window11_open_clone.show_all();
   });
   /* window10 Sell Confirm Button */
   sellconfirm_button.connect_clicked(move |_| {
      //get the eslected item:Button
      //check quantity that it matches with the selected item;
      //deduct quantity fromthe item held by the alias;
      //let item_index = &selected_item_cell_window10clone2.borrow();'


      //get the quantity
      let chosen_quantity: i64 = sellquantity_entry_clone8.get_text().unwrap().parse().ok().expect("invalid entry");
      
      //get the price 
      let chosen_price: f64 = sellprice_entry_clone8.get_text().unwrap().parse().ok().expect("invalid entry");
      //trying to reduce the amount and then have to write this to the file
      let the_items = &exchange_alias_cellclone3.borrow_mut().items;

      println!("paniced earlier");

      println!("paniced early");
      let item_index = (*selected_item_cell_window10clone2.borrow()) as usize;

      let the_item = &the_items[item_index];

      let item_clone = the_item.clone();
      let item_clone2 = the_item.clone();
      let mut keywords = Vec::new();
      for words in item_clone.keywords {
         keywords.push(words);
      }

      let mut images = Vec::new();
      for image in item_clone.images {
         images.push(image);
      }
            
      let new_quantity = item_clone.quantity - chosen_quantity;
      println!("{:?}", new_quantity);
      let item_owner = item_clone2.alias_owner;
      let mut the_particular_item = Item {
         name: item_clone.name.to_string(),
         description: item_clone.description.to_string(),
         quantity: new_quantity,
         recommended_price: item_clone.recommended_price,
         contact: item_clone.contact.to_string(),
         brand_name: item_clone.brand_name.to_string(),
         keywords: keywords,
         images: images,
         alias_owner: item_clone.alias_owner,
      };
      let the_item_clone = the_particular_item.clone();
      let hash_string = "a".to_string() + &the_particular_item.name + &item_owner + &chosen_quantity.to_string();
      let name = hash_string.as_bytes();
      let sha512::Digest(name) =  crypto::hash::sha512::hash(&name);
      let hex = name.to_vec();
      let s1 = to_hex_string(hex);
      let sell_offer = SellOffer {
         item: the_particular_item,
         alias_owner: item_owner,
         quantity: chosen_quantity,
         price: chosen_price,
         condition: Condition::Enabled,
         hash_id: s1,
      };

      let alias_index = (*selected_alias_cell_clone3.borrow()) as usize;


      let mut the_actual_profile = read_account(&profilecell_clone9.borrow().username);

      the_actual_profile.alias[alias_index].items[item_index].quantity = new_quantity;
      the_actual_profile.alias[alias_index].selloffers.push(sell_offer);
      println!("{:?}", &the_actual_profile);
      write_account(&the_actual_profile);

      //need to modify the alias's selloffers and items


      //get into the alias into the items and go through the items and change them.

      //create the sell offer

      //write the sell offer, and adjust the quantity to the item in possession

      //get the profile, find the alias index, find the item index, change the quantity insert it to the profile;
      //write the profile

   });


   /* window11 new item remove selected image */
   newitem_imageremove_button.connect_clicked(move |_| {

   });

   /* window11 new item confirm add new item to inventory */
   newitem_add_button.connect_clicked(move |_| {
      let newitem_name = newitem_name_entry.get_text().unwrap();
      let newitem_quantity: String = newitem_quantity_entry.get_text().unwrap();
      let newitem_recprice: String = newitem_recprice_entry.get_text().unwrap();
      let newitem_keywords = newitem_keyword_entry.get_text().unwrap();

      let mut newitem_keywords_vec: Vec<String> = Vec::new();
      newitem_keywords_vec.push(newitem_keywords);

      let newitem_brandname = newitem_brandname_entry.get_text().unwrap();
      let newitem_contact = newitem_contact_entry.get_text().unwrap();
      let newitem_description = newitem_description_entry.get_text().unwrap();
      let newitem_quantity: i64 = newitem_quantity.parse().ok().expect("not parsed");
      let newitem_recprice: f64 = newitem_recprice.parse().ok().expect("not parsed price");
      let alias_index = (*selected_alias_cell.borrow()) as usize;
      let active_alias = &profilecell_clone2.borrow().alias[alias_index as usize].name;
      let alias_index = (*selected_alias_cell.borrow()) as usize;

      let mut newitem_images: Vec<String> = Vec::new();
      for image in imagelist_cell_clone1.borrow().iter() {
         newitem_images.push(image.to_string());
      }

      println!("{:?}", &active_alias);

      let item = Item { name: newitem_name, description: newitem_description, quantity: newitem_quantity, recommended_price: newitem_recprice, 
          contact: newitem_contact, brand_name: newitem_brandname, keywords: newitem_keywords_vec, images: newitem_images, alias_owner: active_alias.to_string() };

      let mut the_actual_profile = read_account(&profilecell_clone5.borrow().username);
     
      the_actual_profile.alias[alias_index as usize].items.push(item);
      write_account(&the_actual_profile);
      //add item to the items vector of the alias that is active.
      //we need to add an active alias label and also keep track of the active alias
   });

   /* window11 new item add image to item list */
   newitem_addimage_button.connect_clicked(move |_| {
      //modify the image vector that gets passed to creating an item
      imagelist_cell.borrow_mut().push("hello".to_string());


   });

   /* window8 new alias confirm button - appends a new alias to a profile and writes it to the profile */
   confirmnewalias_button.connect_clicked(move |_| {
      let mut bitcoin_public_keys: Vec<String> = Vec::new();
      let mut bitcoin_private_keys: Vec<String> = Vec::new();
      let safecoin_public_keys: Vec<String> = Vec::new();
      let safecoin_private_keys: Vec<String> = Vec::new();
      let items: Vec<Item> = Vec::new();
      let buy_vec: Vec<BuyOffer> = Vec::new();
      let sell_vec: Vec<SellOffer> = Vec::new();
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
      bitcoin_private_keys.push(sk_base);
      bitcoin_public_keys.push(pk_base);
      let the_alias = Alias {
         name: newalias_name_entry.get_text().unwrap(),
         bitcoin_public_keys: bitcoin_public_keys,
         bitcoin_private_keys: bitcoin_private_keys,
         safecoin_public_keys: safecoin_public_keys,
         safecoin_private_keys: safecoin_private_keys,
         items: items,
         buyoffers: buy_vec,
         selloffers: sell_vec,
         public_key: "".to_string(),
         private_key: "".to_string(),
         item_trans: item_trans_vec,
         coin_trans: coin_trans_vec,
         friends: friends_vec,
         mock_balance: 1000,
      };
      let mut the_actual_profile = read_account(&profilecell_clone3.borrow().username);
      let the_name: String = newalias_name_entry.get_text().unwrap();
     //add bitcoin keys and safecoins keys 
      let mut bin_int = 0;
      for aliases in &the_actual_profile.alias {
         if aliases.name == the_name {
            newalias_status_label.set_text("name already exists, choose a new name");
            bin_int += 1;
         }
      }
      if bin_int == 0 {
         the_actual_profile.alias.push(the_alias);
         write_account(&the_actual_profile);
      //need to update the tree model for the list that there is a new alias in town ;)
      /* updating the global profile */
         profilecell_clone3.borrow_mut().username = the_actual_profile.username;
         profilecell_clone3.borrow_mut().password = the_actual_profile.password;
         profilecell_clone3.borrow_mut().alias = the_actual_profile.alias;
         profilecell_clone3.borrow_mut().public_key = the_actual_profile.public_key;
         profilecell_clone3.borrow_mut().private_key = the_actual_profile.private_key;
      }  
      //trtying to get this profile thing into the to write a new profile content in the profile file.
   });
   /* window2 button register a new account */
  	create_button.connect_clicked(move |_| {
  		let username_text = create_username_entry.get_text().unwrap();
      let password1_text = create_password1_entry.get_text().unwrap();
      let password2_text = create_password2_entry.get_text().unwrap();
      let alt_text = create_alt_entry.get_text().unwrap();
      let mut the_home_dir = String::new();

      match env::home_dir() {
         Some(ref p) => the_home_dir = p.display().to_string(),
         None => println!("Impossible to get your home dir!")
      }
      let path_string = String::from("/.test_root/");
      let path_string2 = path_string + &username_text;
      let path_string3 = the_home_dir + &path_string2;
      let path = Path::new(&path_string3);         
      println!("{:?}", &path);
      match fs::metadata(&path) {
         Ok(_) => { 
            create_status_label.set_text("username already exists");
            return;
         },
         Err(..) => {
            create_status_label.set_text("username is all good");//make the directory
         },    
      }
      if password1_text != password2_text {
         create_status_label.set_text("passwords do not match");
         return;
      } else {
         make_new_account(&path_string3, &username_text, &password1_text, &alt_text);
      }

      create_status_label.set_text("profile created");

 		println!("{:?}", &username_text);
      window2_register_clone.hide();
  	});

      /* window3 treeview, treemodel, liststore */
   let aliases_vec = Rc::new(RefCell::new(Vec::new()));

   let alias_tree_view = gtk::TreeView::new().unwrap();
   let the_new = alias_tree_view.clone();
   let alias_list_store = Rc::new(RefCell::new(gtk::ListStore::new(&[glib::Type::String]).unwrap()));

   let thi_thn = alias_list_store.borrow().get_model().unwrap();
   let mobile_model = Rc::new(RefCell::new(thi_thn));
   let mobile_model_clone = mobile_model.clone();

   alias_tree_view.set_model(&mobile_model.borrow());

   alias_tree_view.set_headers_visible(false);
   /* window3 second treeview, treemodel, liststore on window3  */
   let aliases_vec2 = Rc::new(RefCell::new(Vec::new()));

   let alias_tree_view2 = gtk::TreeView::new().unwrap();
   let the_new2 = alias_tree_view2.clone();
   let alias_list_store2 = Rc::new(RefCell::new(gtk::ListStore::new(&[glib::Type::String]).unwrap()));

   let thi_thn2 = alias_list_store2.borrow().get_model().unwrap();
   let mobile_model2 = Rc::new(RefCell::new(thi_thn2));
   let mobile_model_clone2 = mobile_model2.clone();
   alias_tree_view2.set_model(&mobile_model2.borrow());
   alias_tree_view2.set_headers_visible(false);
   /* window1 button */
   login_button.connect_clicked(move |_| {
      let username_text = username_entry.get_text().unwrap();
      let password_text = password_entry.get_text().unwrap();
      
      let the_profile = read_account(&username_text);
      let the_profile_clone = the_profile.clone();
      profile_cell.borrow_mut().username = the_profile_clone.username;
      profile_cell.borrow_mut().password = the_profile_clone.password;
      profile_cell.borrow_mut().alias = the_profile_clone.alias;
      profile_cell.borrow_mut().public_key = the_profile_clone.public_key;
      profile_cell.borrow_mut().private_key = the_profile_clone.private_key;
      let string = the_profile.password;
      if password_text == string {
                  //(*v2.borrow_mut()) = the_profile.alias;
         username_label.set_text(&the_profile.username);

         let l = aliases_vec.borrow().len();
         let position = aliases_vec.borrow().len();
         let renderer = gtk::CellRendererText::new().unwrap();

         aliases_vec.borrow_mut().push(gtk::TreeViewColumn::new().unwrap());
         let tmp = aliases_vec.borrow_mut();
         tmp[l].set_title("aliases");
         tmp[l].set_resizable(true);
         tmp[l].pack_start(&renderer, true);
         tmp[l].add_attribute(&renderer, "text", position as i32);
             //append_column("alias", &mut aliases_vec);
                  
         alias_tree_view.append_column(&tmp[l]);
                  
         for aliases in the_profile.alias {
            println!("{:?}", &aliases);    
            let mut top_level = gtk::TreeIter::new();

            alias_list_store.borrow_mut().append(&mut top_level);
            alias_list_store.borrow_mut().set_string(&top_level, 0, &aliases.name); 
         }


         alias_tree_view2.set_model(&mobile_model2.borrow());
         alias_tree_view2.set_headers_visible(false);

         let l2 = aliases_vec2.borrow().len();
         let position2 = aliases_vec2.borrow().len();
         let renderer2 = gtk::CellRendererText::new().unwrap();

         aliases_vec2.borrow_mut().push(gtk::TreeViewColumn::new().unwrap());
         let tmp2 = aliases_vec2.borrow_mut();
         tmp2[l2].set_title("aliases");
         tmp2[l2].set_resizable(true);
         tmp2[l2].pack_start(&renderer2, true);
         tmp2[l2].add_attribute(&renderer2, "text", position2 as i32);
               //append_column("alias", &mut aliases_vec);
                  
         alias_tree_view2.append_column(&tmp2[l2]);
                  
         let the_profile2 = read_account(&username_text);

         for aliases in the_profile2.alias {
            println!("{:?}", &aliases);
            for address in aliases.bitcoin_public_keys {
               let mut top_level = gtk::TreeIter::new();

               alias_list_store2.borrow_mut().append(&mut top_level);
               alias_list_store2.borrow_mut().set_string(&top_level, 0, &address); 
            }
         }
         
         own_addres_scroll_box.add(&alias_tree_view2);
         alias_scroll_box.add(&alias_tree_view);
         window3.show_all();

         window1_login_successful_clone.hide();
      }

   });

   /* window9 buy item selection */
   let searchtobuy_selection = searchtobuy_treeview_clone.get_selection().unwrap();
   searchtobuy_selection.connect_changed(move |tree_selection| {
      let mut iter = gtk::TreeIter::new();
      tree_selection.get_selected(&searchtobuy_model_clone.borrow(), &mut iter);
      if let Some(path) = searchtobuy_model_clone.borrow().get_path(&iter) {
         
      }
   });

   /* window10 selection item selection of selected alias on window10 SellOffer */
   let sellexchangeitem_selection = sellexchangeitems_treeview_clone.get_selection().unwrap();
   sellexchangeitem_selection.connect_changed(move |tree_selection| {
      let mut iter = gtk::TreeIter::new();
      tree_selection.get_selected(&sellexchangeitem_model_clone.borrow(), &mut iter);
      if let Some(path) = sellexchangeitem_model_clone.borrow().get_path(&iter) {
         let the_paths = path.to_string().unwrap();
         let paths_int: usize = the_paths.parse().unwrap();
         println!("{:?}", &paths_int);
         //println!("{:?}", &exchange_alias_cellclone1);
         let ints: usize = *selected_item_window10cell.borrow() as usize;
         (*selected_item_window10cell.borrow_mut()) = paths_int;
         window10exchange_item_cell.borrow_mut().name = exchange_alias_cellclone2.borrow().items[paths_int].name.to_string();
         sellname_label.set_text(&exchange_alias_cellclone2.borrow().items[paths_int].name);
         selldescription_text.get_buffer().unwrap().set_text(&exchange_alias_cellclone2.borrow().items[paths_int].description);
         (*selecteditem_window10_quantity_cell_clone1.borrow_mut()) = exchange_alias_cellclone2.borrow().items[paths_int].quantity;
         (*window10_recprice_cell_clone1.borrow_mut()) = exchange_alias_cellclone2.borrow().items[paths_int].recommended_price;
         sellprice_label.set_text(&exchange_alias_cellclone2.borrow().items[paths_int].recommended_price.to_string());
         let the_zero = 1;
         sellquantity_entry_clone7.set_text(&the_zero.to_string());
         //double click enabled'

         //let item_name = &(*exchange_alias_cellclone1.borrow()).items[0].name;
         
         //get a refcell here that stores the selected item, and loads it up and passes that items attributes to the buttons above.
      }
   });

   /* window7 selection item selection of selected alias*/
   let exchangeitem_selection = exchangeitems_treeview_clone.get_selection().unwrap();
   exchangeitem_selection.connect_changed(move |tree_selection| {
      let mut iter = gtk::TreeIter::new();
      tree_selection.get_selected(&exchangeitem_model_clone.borrow(), &mut iter);
      if let Some(path) = exchangeitem_model_clone.borrow().get_path(&iter) {
         let the_paths = path.to_string().unwrap();
         let paths_int: usize = the_paths.parse().unwrap();
         println!("{:?}", &paths_int);
         //println!("{:?}", &exchange_alias_cellclone1);
         (*selected_item_window7cell.borrow_mut()) = paths_int;
         //let item_name = &(*exchange_alias_cellclone1.borrow()).items[0].name;
         
         //get a refcell here that stores the selected item, and loads it up and passes that items attributes to the buttons above.
      }
   });


   /* window3 list selection of cryptocurrency addresses */
   let right_selection = the_new2.get_selection().unwrap();
   right_selection.connect_changed(move |tree_selection| {
      let mut iter = gtk::TreeIter::new();
      tree_selection.get_selected(&mobile_model_clone2.borrow(), &mut iter);
      if let Some(path) = mobile_model_clone2.borrow().get_path(&iter) {
         let the_paths = path.to_string().unwrap();
         let the_paths_int: usize = the_paths.parse().unwrap();
         println!("{:?}", the_paths_int);
         //(*selected_alias_cell_clone1.borrow_mut()) = the_paths_int;
      }
   });

   //just need to extract the model and pass it to get the path out.
   /* window3 button */
   let left_selection = the_new.get_selection().unwrap();
   //let left_model1 = left_model.clone();
   left_selection.connect_changed(move |tree_selection| {
      let mut iter = gtk::TreeIter::new();
      tree_selection.get_selected(&mobile_model_clone.borrow(), &mut iter);
      if let Some(path) = mobile_model_clone.borrow().get_path(&iter) {
         let the_paths = path.to_string().unwrap();
         let the_paths_int: usize = the_paths.parse().unwrap();
         //println!("{:?}", the_paths_int);
         (*selected_alias_cell_clone1.borrow_mut()) = the_paths_int;
         //let s = (*z2.borrow()).get_slice();
         //let a_string = "hello"; //&alias_ve[the_paths_int].name;
         //(*z.borrow_mut()).push(&a_string);
         //println!("selected row {:?}", (*z2.borrow_mut()).as_slice().split_at_mut(0));
      }
   });
      
   

   window1.show_all();

   window1.connect_delete_event(|_, _| {
		gtk::main_quit();
		Inhibit(true)
	});

   window2_close_clone.connect_delete_event(move |_, _| {
      window2_close_clone2.hide();
      Inhibit(true)
   });

   window3_panel_clone.connect_delete_event(|_, _| {
      gtk::main_quit();
      Inhibit(true)
   });

   window7.connect_delete_event(move |_, _| {
      window7_close_clone.hide();
      Inhibit(true)
   });

   window9.connect_delete_event(move |_, _| {
      window9_close_clone.hide();
      Inhibit(true)
   });

   window10.connect_delete_event(move |_, _| {
      window10_close_clone.hide();
      Inhibit(true)
   });

   window11.connect_delete_event(move |_, _| {
      window11_close_clone.hide();
      Inhibit(true)
   });

   window8.connect_delete_event(move |_, _| {
      window8_close_clone.hide();
      Inhibit(true)

   });

   gtk::main();

}
