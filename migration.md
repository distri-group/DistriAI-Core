# Program data migration
When changing the account struct, if want to retain the existing data of the program, need to perform data migration after upgrading the program.

## Overview
1. Migrate the accounts in the original struct to the accounts in the temporary struct
2. Migrate the accounts in the temporary struct to the accounts in the original struct. 

The public key of the accounts will not change, and program users will not be affected before and after data migration.

## Example
1. Original account struct `Example`.
```
#[account]
#[derive(InitSpace)]
pub struct Example {
    pub owner: Pubkey,
}
```

2. New account struct `ExampleNew`, add a data field.
```
#[account]
#[derive(InitSpace)]
pub struct ExampleNew {
    pub owner: Pubkey,
    pub data: u32,
}
```

3. Instruction `migrate_example_new` to migrate the `Example` accounts to `ExampleNew` accounts.
```
pub fn migrate_example_new(ctx: Context<MigrationExampleNew>) -> Result<()> {
    let example_before = &mut ctx.accounts.example_before;
    let example_after = &mut ctx.accounts.example_after;
    example_after.a = example_before.a;

    Ok(())
}

#[derive(Accounts)]
pub struct MigrationExampleNew<'info> {
    #[account(
        mut,
        close = signer
    )]
    pub example_before: Account<'info, Example>,

    #[account(
        init,
        seeds = [b"example-new", example_before.owner.as_ref()],
        bump,
        payer = signer,
        space = 8 + ExampleNew::INIT_SPACE
    )]
    pub example_after: Account<'info, ExampleNew>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
```

4. Query all `Example` accounts, execute `migrate_example_new` instruction.
```
const examples = await pg.program.account.example.all();
examples.forEach(async (example) => {
  const [exampleNewPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("example-new"),
      example.account.owner.toBuffer(),
    ],
    pg.PROGRAM_ID
  );
  const txHash = await pg.program.methods
    .migrateExampleNew()
    .accounts({
      exampleBefore: example.publicKey,
      exampleAfter: exampleNewPDA,
    })
    .rpc();
  await logTransaction(txHash);
});
```

5. Original account struct `Example` also add the data field.
```
#[account]
#[derive(InitSpace)]
pub struct Example {
    pub owner: Pubkey,
    pub data: u32,
}
```

6. Instruction `migrate_example_rename` to migrate the `ExampleNew` accounts to `Example` accounts.
```
pub fn migrate_example_rename(ctx: Context<MigrationExampleRename>) -> Result<()> {
    let example_before = &mut ctx.accounts.example_before;
    let example_after = &mut ctx.accounts.example_after;
    example_after.a = example_before.a;
    example_after.b = example_before.b;
    
    Ok(())
}

#[derive(Accounts)]
pub struct MigrationExampleRename<'info> {
    #[account(
        mut,
        close = signer
    )]
    pub example_before: Account<'info, ExampleNew>,

    #[account(
        init,
        seeds = [b"example", example_before.owner.as_ref()],
        bump,
        payer = signer,
        space = 8 + Example::INIT_SPACE
    )]
    pub example_after: Account<'info, Example>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
```

7. Query all `ExampleNew` accounts, execute `migrate_example_rename` instruction.
```
const exampleNews = await pg.program.account.exampleNew.all();
exampleNews.forEach(async (exampleNew) => {
  const [examplePDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("example"),
      exampleNew.account.owner.toBuffer(),
    ],
    pg.PROGRAM_ID
  );
  const txHash = await pg.program.methods
    .migrateExampleRename()
    .accounts({
      exampleBefore: exampleNew.publicKey,
      exampleAfter: examplePDA,
    })
    .rpc();
  await logTransaction(txHash);
});
```