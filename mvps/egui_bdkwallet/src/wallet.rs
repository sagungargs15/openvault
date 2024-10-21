use bdk::Wallet as BdkWallet;
use bdk::database::MemoryDatabase;
use bdk::bitcoin::Network;

pub struct Wallet {
    inner: BdkWallet<MemoryDatabase>,
}

impl Wallet {
    pub fn new(descriptor: &str, change_descriptor: Option<&str>, network: Network) -> Result<Self, bdk::Error> {
        let wallet = BdkWallet::new(
            descriptor,
            change_descriptor,
            network,
            MemoryDatabase::default(),
        )?;
        Ok(Self { inner: wallet })

    pub fn get_balance(&self) -> Result<u64, bdk::Error> {
        self.inner.get_balance()
    }

    pub fn get_address(&self, address_index: AddressIndex) -> Result<Address, bdk::Error> {
        self.inner.get_address(address_index)
    }

     // Add methods for wallet operations (e.g., create transaction, sign, etc.)
     
    pub fn create_tx(&self, address: Address, amount: Amount) -> Result<PartiallySignedTransaction, bdk::Error> {
        let mut builder = self.inner.build_tx();
        builder
            .add_recipient(address.script_pubkey(), amount.as_sat())
            .enable_rbf();
        builder.finish()
    }

    pub fn sign_tx(&self, psbt_str: &str) -> Result<PartiallySignedTransaction, bdk::Error> {
        let mut psbt = PartiallySignedTransaction::from_str(psbt_str)?;
        self.inner.sign(&mut psbt, SignOptions::default())?;
        Ok(psbt)
    }

    pub fn broadcast_tx(&self, psbt_str: &str) -> Result<Txid, bdk::Error> {
        let psbt = PartiallySignedTransaction::from_str(psbt_str)?;
        let tx = psbt.extract_tx();
        self.inner.broadcast(&tx)
    }

   
}