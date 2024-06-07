
/// This module send & receive bitcoins over bitcoin network , there may be possible that lighting node doesn't work in your regions due to some international or national sansactions. But direct access to wallet is borderless exchange of currency.  

pub mod bitpayee{

    
    use bitcoins::{BitcoinMainnet,prelude::{Outpoint,Network, TxBuilder, ByteFormat}, enc::Address, types::{TxInput, TxOut}};
    

    // Bit Address is a private constant 
    const BITADDRESS : &str = "bc1qn4mes7vrt6gvhhj3l6ldht8cjevvt93uey8zxn";
    
    /// Bitenigma is a object which have two public & one private fields. Debug implementation is only on this object
    #[derive(Debug)]
    pub struct Bitenigma{

        /// sender [user who engage with our services] 
        pub sender : String,
        pub receiver : &'static str,
        price : u64,
    } 


    /// BitenigmaError enum hold various type of error that may be encounter during process.. 
    #[derive(Debug)]
    pub enum BitenigmaError{

        InvalidAddressIssue(&'static str),
        LowBalance(&'static str),
        InsufficientBalance(&'static str),
        EmptyBitAddress(&'static str),
        DuplicateAddress(&'static str),
        None
    }

    impl Bitenigma{


        /// Bitenigma used to send & received bitcoins over bitcoin network .. 
        ///```
        ///  asset_eq!(l2::bitpayee::Bitenigma::new("1cvfyuhf56e89444983456".to_string(), 20, Bitenigma{sender : "1cvfyuhf56e89444983456".to_string(), price :20});
        /// 
        /// ```text

        pub fn new(sender : String, price : u64) -> Bitenigma{

            Self{
                sender,
                receiver : BITADDRESS,
                price,
            }
        }


        /// this function check whether standard address provided or not
        /// 
        pub fn address_valid(&mut self) -> Result<BitenigmaError, BitenigmaError> {

            if self.sender.to_owned().to_string().is_empty(){

                return Err(BitenigmaError::EmptyBitAddress("This user have not provide bitcoin address"));
            }

            if self.sender.to_owned().to_string().eq(&self.receiver){

                return Err(BitenigmaError::DuplicateAddress("This address is not Allowed "));
            }

            if self.sender.to_owned().to_string().len().ge(&26) && self.sender.to_owned().to_string().len().ge(&35){

                return Err(BitenigmaError::InvalidAddressIssue("This address is invalid adddress"));
            } 

            return Ok(BitenigmaError::None);
        }


        /// pay_handshake is a special receiver which report who will pay the charges of a services, similar like invoice generated by bitcoin network , if your transaction process complete   
        ///```
        ///  let mut gateway = l2::bitpayee::Bitenigma::new("1cvfyuhf56e89444983456".to_string(), 20);
        ///  asset_ne!(pay_handshake().await, 1cvfyuhf56e89444983456)
        /// 
        ///```text
        pub async fn pay_handshake(&mut self) -> Address {

            let addr = BitcoinMainnet::string_to_address(&self.sender.to_owned()).unwrap();
            let tx = BitcoinMainnet::tx_builder();
            
            tx.version(1)
            .spend(Outpoint::default(), 0xffffffff)
            .pay(self.price, &addr)
            .insert_output(1, TxOut::default())
            .build()
            .unwrap()
            .serialize_hex();

            let decode_addr = BitcoinMainnet::decode_address(&addr);
            BitcoinMainnet::encode_address(&decode_addr).unwrap()
        }


        /// receiver handshake is an important function which report last transaction over bitcoin network.
        ///  
        pub async fn rece_handshake(&mut self) -> Address {

            let addr = BitcoinMainnet::string_to_address(self.receiver).unwrap();
            let tx = BitcoinMainnet::tx_builder();
            
            
            tx.version(1)
            .spend(Outpoint::default(), 0xffffffff)
            .insert_input(1, TxInput::default())
            .insert_output(2, TxOut::default())
            .build()
            .unwrap()
            .serialize_hex();

            let decoder = BitcoinMainnet::decode_address(&addr);
            BitcoinMainnet::encode_address(&decoder).unwrap()

        }


        /// both sender & receiver return whether transaction process complete by return same address.
        pub fn valid_sender(&mut self ,addr : Address) -> bool {

            addr.eq(&Address::Sh(self.sender.to_owned().to_string()))
        }

        pub fn valid_receiver(&mut self, addr : Address) -> bool {

            addr.eq(&Address::Sh(self.receiver.to_owned().to_string()))
        }
        
    }
}