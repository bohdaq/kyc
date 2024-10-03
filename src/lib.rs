use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
    // program_pack::Pack,
};
use solana_program::account_info::next_account_info;

#[derive(Debug)]
pub struct UserIdentity {
    pub user_pubkey: Pubkey,
    pub name: String,
    pub age: u32,
    pub document_hash: String, // hash of the document (e.g., ID)
}

impl UserIdentity {
    const LEN: usize = 32 + 4 + 100 + 64; // Adjust sizes as necessary

    fn pack(&self, dst: &mut [u8]) {
        let user_pubkey_bytes = self.user_pubkey.to_bytes();
        dst[..32].copy_from_slice(&user_pubkey_bytes);
        dst[32..36].copy_from_slice(&self.age.to_le_bytes());
        dst[36..136].copy_from_slice(self.name.as_bytes());
        dst[136..200].copy_from_slice(self.document_hash.as_bytes());
    }

    fn unpack(src: &[u8]) -> Self {
        let user_pubkey = Pubkey::new_from_array(src[..32].try_into().unwrap());
        let age = u32::from_le_bytes(src[32..36].try_into().unwrap());
        let name = String::from_utf8(src[36..136].to_vec()).unwrap();
        let document_hash = String::from_utf8(src[136..200].to_vec()).unwrap();
        
        UserIdentity {
            user_pubkey,
            name,
            age,
            document_hash,
        }
    }
}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let user_account = next_account_info(accounts_iter)?;

    if !user_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    let user_identity = UserIdentity::unpack(instruction_data);

    // Store user identity in the account data
    let mut data = user_account.try_borrow_mut_data()?;
    let len = UserIdentity::LEN;
    
    if data.len() < len {
        return Err(ProgramError::AccountDataTooSmall);
    }

    user_identity.pack(&mut data);
    msg!("User identity verified and stored: {:?}", user_identity);

    Ok(())
}