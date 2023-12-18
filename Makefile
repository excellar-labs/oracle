deploy-oracle:
	soroban contract deploy \
		--wasm target/wasm32-unknown-unknown/release/excellar_oracle.wasm \
        --source SCVGDYKZQFNDLJ2DI4HS7JH3Q65T2SCAR7TBZOKZXRHUEYRM5VW4GXWA \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022'
