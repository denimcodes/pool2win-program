import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { createAssociatedTokenAccount, getAccount } from "@solana/spl-token";
import { expect } from "chai";
import { Pooltowin } from "../target/types/pooltowin";

describe("pooltowin", () => {
	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);

	const program = anchor.workspace.Pooltowin as Program<Pooltowin>;

	const owner = provider.wallet.publicKey;

	const user = anchor.web3.Keypair.generate();

	const getPDAFromSeed = (seed: string) => {
		return anchor.web3.PublicKey.findProgramAddressSync(
			[Buffer.from(seed), owner.toBuffer()],
			program.programId
		)[0];
	};

	const poolPDA = getPDAFromSeed("pool-account");
	const tokenAccountPDA = getPDAFromSeed("token-account");
	const tokenMintPDA = getPDAFromSeed("token-mint");
	const userInfoPDA = getPDAFromSeed("user-info");

	let userTokenAccount: anchor.web3.PublicKey;

	it("Setup", async () => {
		await provider.connection.requestAirdrop(
			user.publicKey,
			anchor.web3.LAMPORTS_PER_SOL
		);
	});

	it("Creates new token", async () => {
		await program.methods
			.initTokenMint()
			.accounts({ owner, tokenMint: tokenMintPDA })
			.rpc();
		console.log(`New token created`);
	});

	it("Creates pool", async () => {
		await program.methods
			.initPool()
			.accounts({
				owner,
				pool: poolPDA,
				tokenAccount: tokenAccountPDA,
				mint: tokenMintPDA,
			})
			.rpc();
		console.log(`New pool created`);
	});

	it("Creates user token", async () => {
		userTokenAccount = await createAssociatedTokenAccount(
			provider.connection,
			user,
			tokenMintPDA,
			user.publicKey
		);
		console.log(`User token account: ${userTokenAccount}`);
	});

	it("Mints token", async () => {
		await program.methods
			.mintToken()
			.accounts({
				mint: tokenMintPDA,
				recipient: userTokenAccount,
				signer: owner,
			})
			.rpc();
		const userAccount = await getAccount(provider.connection, userTokenAccount);
		console.log(`User token amount: ${userAccount.amount}`);

		await program.methods
			.mintToken()
			.accounts({
				mint: tokenMintPDA,
				recipient: tokenAccountPDA,
				signer: owner,
			})
			.rpc();
		const poolAccount = await getAccount(provider.connection, tokenAccountPDA);
		console.log(`Pool token amount: ${poolAccount.amount}`);
	});
	
	it("Creates user info", async () => {
		await program.methods.initUserInfo().accounts({
			owner,
			userInfo: userInfoPDA,
		}).rpc();

		const userInfo = await program.account.userInfo.fetch(userInfoPDA);
		expect(userInfo.amount.toString()).to.equal("0");
	})

	it("Deposits token in pool", async () => {
		const depositAmount = new anchor.BN(100);
		await program.methods.depositPool(depositAmount).accounts({
			signer: user.publicKey,
			userTokenAccount,
			poolTokenAccount: tokenAccountPDA,
			userInfo: userInfoPDA
		}).signers([user]).rpc();

		const userAccount = await getAccount(provider.connection, userTokenAccount);
		expect(userAccount.amount.toString()).to.equal("900");
		const poolAccount = await getAccount(provider.connection, tokenAccountPDA);
		expect(poolAccount.amount.toString()).to.equal("1100");
		const userInfoAccount = await program.account.userInfo.fetch(userInfoPDA);
		expect(userInfoAccount.amount.toString()).to.equal("100");
	});

	it("Withdraws token from pool", async () => {
		const withdrawAmount = new anchor.BN(100);
		await program.methods.withdrawPool(withdrawAmount).accounts({
			signer: owner,
			userTokenAccount,
			poolTokenAccount: tokenAccountPDA,
			userInfo: userInfoPDA
		}).rpc();

		const userAccount = await getAccount(provider.connection, userTokenAccount);
		expect(userAccount.amount.toString()).to.equal("1000");
		const poolAccount = await getAccount(provider.connection, tokenAccountPDA);
		expect(poolAccount.amount.toString()).to.equal("1000");
		const userInfoAccount = await program.account.userInfo.fetch(userInfoPDA);
		expect(userInfoAccount.amount.toString()).to.equal("0");
	});
});
