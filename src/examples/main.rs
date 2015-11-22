extern crate gtk;
extern crate glib;
extern crate rustc_serialize;
extern crate exchangelib;

use exchangelib::*;

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

//   let buy_description_text_view: gtk::TextView = builder.get_object("buyDescriptionTextView").unwrap();
   
   /* window10 part of window7 + UI components for Sell */
   let window10: Window = builder.get_object("window10").unwrap();
   let window10_open_clone = window10.clone();
   let window10_close_clone = window10.clone();

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
   let mut imagecreation_list: Vec<String> = Vec::new();
   let imagelist_cell = Rc::new(RefCell::new(imagecreation_list));
   let imagelist_cell_clone1 = imagelist_cell.clone();

   let mut profile_global = Profile::new();
   let profile_cell = Rc::new(RefCell::new(profile_global));
   let profilecell_clone2 = profile_cell.clone();
   let profilecell_clone3 = profile_cell.clone();
   let profilecell_clone4 = profile_cell.clone();
   let profilecell_clone5 = profile_cell.clone();

   let mut selected_alias = 0;
   let selected_alias_cell = Rc::new(RefCell::new(selected_alias));
   let selected_alias_cell_clone1 = selected_alias_cell.clone();
   let selected_alias_cell_clone2 = selected_alias_cell.clone();

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

      /* window9 BuyTHings Buttons */
   confirm_buy_button.connect_clicked(move |_| {

   });

      /* window9 BuyTHings Buttons */
   buy_search_button.connect_clicked(move |_| {

   });

   /* window1 button */
  	new_button.connect_clicked(move |_| {
  		window2.show_all();
  	});

   /* window7 treeview, treemodel, liststore for your item list */
   //let items_vec = Rc::new(RefCell::new(Vec::new()));

   let exchangeitems_tree_view = gtk::TreeView::new().unwrap();
   let exchangeitems_tree_view_clone = exchangeitems_tree_view.clone();
   let exchangeitems_liststore = Rc::new(RefCell::new(gtk::ListStore::new(&[glib::Type::String]).unwrap()));

   let exchangeitems_model = exchangeitems_liststore.borrow().get_model().unwrap();
   let exchangeitems_model_cell = Rc::new(RefCell::new(exchangeitems_model));
   let exchangeitems_model_cell_clone = exchangeitems_model_cell.clone();
   /* window3 exchange open button */
   exchange_button.connect_clicked(move |_| {
      let alias_index = (*selected_alias_cell_clone2.borrow()) as usize;

      //get selected alias and then choose selected alias from profile list of aliases
      //then go through the items vector and put them into the model
      //add the model to exchangeyouritemsScrollWindow
      //fill window 7 scroll boxes with the corresponding item ist.
      window7_open_clone.show_all();
   });

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

   /* window7 sell item button - issue an item you own for sale */
   newsell_button.connect_clicked(move |_| {
      window10_open_clone.show_all();
   });

   /* window7 new item button - new item creation */
   newitem_button.connect_clicked(move |_| {
      window11_open_clone.show_all();
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
      let bitcoin_public_keys: Vec<String> = Vec::new();
      //generate bitcoin key pairs and deposit them into the vector. Add just one to start.
      let bitcoin_private_keys: Vec<String> = Vec::new();
      let safecoin_public_keys: Vec<String> = Vec::new();
      let safecoin_private_keys: Vec<String> = Vec::new();
      let items: Vec<Item> = Vec::new();
      let the_alias = Alias {
         name: newalias_name_entry.get_text().unwrap(),
         bitcoin_public_keys: bitcoin_public_keys,
         bitcoin_private_keys: bitcoin_private_keys,
         safecoin_public_keys: safecoin_public_keys,
         safecoin_private_keys: safecoin_private_keys,
         items: items,
      };
      let mut the_actual_profile = read_account(&profilecell_clone4.borrow().username);
     //add bitcoin keys and safecoins keys 
      the_actual_profile.alias.push(the_alias);
      write_account(&the_actual_profile);
      //need to update the tree model for the list that there is a new alias in town ;)
      /* updating the global profile */
      profilecell_clone3.borrow_mut().username = the_actual_profile.username;
      profilecell_clone3.borrow_mut().password = the_actual_profile.password;
      profilecell_clone3.borrow_mut().alias = the_actual_profile.alias;
      profilecell_clone3.borrow_mut().public_key = the_actual_profile.public_key;
      profilecell_clone3.borrow_mut().private_key = the_actual_profile.private_key;

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

   /* window3 second treeview, treemodel, liststore on window3  */
   //let aliases_vec2 = Rc::new(RefCell::new(Vec::new()));

   let alias_tree_view2 = gtk::TreeView::new().unwrap();
   let the_new2 = alias_tree_view2.clone();
   let alias_list_store2 = Rc::new(RefCell::new(gtk::ListStore::new(&[glib::Type::String]).unwrap()));

   let thi_thn2 = alias_list_store2.borrow().get_model().unwrap();
   let mobile_model2 = Rc::new(RefCell::new(thi_thn2));
   let mobile_model_clone2 = mobile_model2.clone();

   /* window1 button */
   login_button.connect_clicked(move |_| {
      let username_text = username_entry.get_text().unwrap();
      let password_text = password_entry.get_text().unwrap();
      /*
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
         Ok(_) => { */
            //create_status_label.set_text("trying the password");
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

               //let left_model = alias_list_store.borrow().get_model().unwrap();
               //let left_model_clone = left_model.clone();
               //mobile_model.borrow_mut().set_model(left_model).unwrap();

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


                 // create_and_fill_model(alias_list_store.borrow_mut(), &aliases.name);
        //reser.lock().unwrap().push(aliases);
         //(*v2.borrow_mut()).push(aliases);
         }
         let the_model_to_use = alias_list_store.borrow().get_model().unwrap();

         alias_tree_view.set_model(&the_model_to_use);
         alias_tree_view.set_headers_visible(false);
/*
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
                 // create_and_fill_model(alias_list_store.borrow_mut(), &aliases.name);
        //reser.lock().unwrap().push(aliases);
         //(*v2.borrow_mut()).push(aliases);
         }*/
                  //println!("{:?}", &the_profile);
         //own_addres_scroll_box.add(&alias_tree_view2);
         alias_scroll_box.add(&alias_tree_view);
         window3.show_all();

         window1_login_successful_clone.hide();
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