# pooltowin
# Built with Seahorse v0.1.1

from curses import use_default_colors
from tokenize import Token

from seahorse.prelude import *

declare_id('FPpwkb2FsmhYnUXfbrxkE5HpwKZsbCcwExDUudrVdGdY')


class Pool(Account):
    token_mint: Pubkey
    token_account: Pubkey
    owner: Pubkey

class UserInfo(Account):
    amount: u64

@instruction
def init_token_mint(owner: Signer, token_mint: Empty[TokenMint]):
    token_mint.init(
        payer=owner,
        seeds=["token-mint", owner],
        decimals=6,
        authority=owner
    )

@instruction
def init_user_info(owner: Signer, user_info: Empty[UserInfo]):
    user_info.init(
        payer=owner,
        seeds=["user-info", owner],
    )
    user_info.amount = 0

@instruction
def init_pool(owner: Signer, pool: Empty[Pool], token_account: Empty[TokenAccount], mint: TokenMint):
    pool.init(
        payer=owner,
        seeds=["pool-account", owner],
    )
    pool.owner = owner.key()
    pool.token_mint = mint.key()
    pool.token_account = token_account.init(
        payer=owner,
        seeds=["token-account", owner],
        mint=mint,
        authority=owner
    ).key()


@instruction
def mint_token(mint: TokenMint, recipient: TokenAccount, signer: Signer):
    mint.mint(
        authority=signer,
        to=recipient,
        amount=1000
    )


@instruction
def deposit_pool(signer: Signer, user_token_account: TokenAccount, pool_token_account: TokenAccount, user_info: UserInfo, amount: u64):
    assert amount > 0, "Deposit amount should be more than 0"

    user_token_account.transfer(
        authority=signer,
        to=pool_token_account,
        amount=amount
    )

    user_info.amount += amount


@instruction
def withdraw_pool(signer: Signer, user_token_account: TokenAccount, pool_token_account: TokenAccount, user_info: UserInfo, amount: u64):
    assert amount > 0, "Withdraw amount should be more than 0"

    pool_token_account.transfer(
        authority=signer,
        to=user_token_account,
        amount=amount
    )

    user_info.amount -= amount
