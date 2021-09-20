
pub mod Payer {
     pub mod Bob{
         #[derive(Debug)]
         pub struct record{
             pub Balance :i64,
             pub Name : String,            
         }
         pub fn tnx(balance :i64 , name: String, pay : i64 ) -> record{
            let peer = record{Balance : balance -pay, Name : name};
            peer
         }
     }
     pub mod Alice{
        use crate::Payer::Bob; 
        pub fn tnx(balance :i64 , name: String, pay: i64 ) -> Bob::record{
             let alice = Bob::record{Balance : balance+pay, Name : name};
             alice
         }
     }
}

use crate::Payer::Bob;
use crate::Payer::Alice;
fn main(){
    let name : String = String::from("Bob");
    let balance : i64 = 100;
    let bob_tnx = Bob::tnx(balance,name, 20);
    println!(" Bob Transaction {:?}, {:?}", bob_tnx, balance);
    let payee : String = String::from("Alice");
    let balance_tns : i64 = 100;
    let alice = Alice::tnx(balance_tns,payee, balance_tns - bob_tnx.Balance);
    println!(" Alice balance {:?}, {:?}", alice, balance_tns);
}