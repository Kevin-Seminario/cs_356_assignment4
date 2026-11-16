# World 3 peer renewals

This folder puts the three prepared bilateral agreements in one place. For each partner, upload its `.wasm` file as a **separate** agreement on the game's World 3 Agreements page and attach the matching `.rs` source file. The site requires each partner to sign its own agreement; selecting several files together does not create several offers.

- `lucky-comets.wasm` and `lucky-comets.rs` — Lucky Comets.
- `spatulameow.wasm` and `spatulameow.rs` — spatulameow.
- `networkarena.wasm` and `networkarena.rs` — networkarena.

Each agreement opens one peer link with jknetwork, has no payment, and lasts one round before renewing unless either party declines. Claude 1 has already proposed a separate peer agreement; review that offer instead of using a fourth file here. Claude 2's paid regional-transit offer is separate and has already been sent.

The source crates remain under `examples/agreements/`. If their source changes, rebuild the wasm and refresh this folder before uploading anything.
