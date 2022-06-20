(function() {var implementors = {};
implementors["ln_gateway"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"mint_client/clients/gateway/enum.GatewayClientError.html\" title=\"enum mint_client::clients::gateway::GatewayClientError\">GatewayClientError</a>&gt; for <a class=\"enum\" href=\"ln_gateway/enum.LnGatewayError.html\" title=\"enum ln_gateway::LnGatewayError\">LnGatewayError</a>","synthetic":false,"types":["ln_gateway::LnGatewayError"]}];
implementors["minimint"] = [{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"https://rust-random.github.io/rand/rand_core/trait.RngCore.html\" title=\"trait rand_core::RngCore\">RngCore</a> + <a class=\"trait\" href=\"https://rust-random.github.io/rand/rand_core/trait.CryptoRng.html\" title=\"trait rand_core::CryptoRng\">CryptoRng</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"struct\" href=\"minimint/consensus/struct.MinimintConsensus.html\" title=\"struct minimint::consensus::MinimintConsensus\">MinimintConsensus</a>&lt;R&gt;&gt; for &amp;'a <a class=\"struct\" href=\"minimint/modules/wallet/struct.Wallet.html\" title=\"struct minimint::modules::wallet::Wallet\">Wallet</a>","synthetic":false,"types":["minimint_wallet::Wallet"]},{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"https://rust-random.github.io/rand/rand_core/trait.RngCore.html\" title=\"trait rand_core::RngCore\">RngCore</a> + <a class=\"trait\" href=\"https://rust-random.github.io/rand/rand_core/trait.CryptoRng.html\" title=\"trait rand_core::CryptoRng\">CryptoRng</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"struct\" href=\"minimint/consensus/struct.MinimintConsensus.html\" title=\"struct minimint::consensus::MinimintConsensus\">MinimintConsensus</a>&lt;R&gt;&gt; for &amp;'a <a class=\"struct\" href=\"minimint/modules/mint/struct.Mint.html\" title=\"struct minimint::modules::mint::Mint\">Mint</a>","synthetic":false,"types":["minimint_mint::Mint"]},{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"https://rust-random.github.io/rand/rand_core/trait.RngCore.html\" title=\"trait rand_core::RngCore\">RngCore</a> + <a class=\"trait\" href=\"https://rust-random.github.io/rand/rand_core/trait.CryptoRng.html\" title=\"trait rand_core::CryptoRng\">CryptoRng</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"struct\" href=\"minimint/consensus/struct.MinimintConsensus.html\" title=\"struct minimint::consensus::MinimintConsensus\">MinimintConsensus</a>&lt;R&gt;&gt; for &amp;'a <a class=\"struct\" href=\"minimint/modules/ln/struct.LightningModule.html\" title=\"struct minimint::modules::ln::LightningModule\">LightningModule</a>","synthetic":false,"types":["minimint_ln::LightningModule"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"minimint/transaction/enum.TransactionError.html\" title=\"enum minimint::transaction::TransactionError\">TransactionError</a>&gt; for <a class=\"enum\" href=\"minimint/consensus/enum.TransactionSubmissionError.html\" title=\"enum minimint::consensus::TransactionSubmissionError\">TransactionSubmissionError</a>","synthetic":false,"types":["minimint::consensus::TransactionSubmissionError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;<a class=\"enum\" href=\"https://docs.rs/bincode/1.3.3/bincode/error/enum.ErrorKind.html\" title=\"enum bincode::error::ErrorKind\">ErrorKind</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"enum\" href=\"minimint/net/framed/enum.FrameError.html\" title=\"enum minimint::net::framed::FrameError\">FrameError</a>","synthetic":false,"types":["minimint::net::framed::FrameError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"minimint/net/framed/enum.FrameError.html\" title=\"enum minimint::net::framed::FrameError\">FrameError</a>","synthetic":false,"types":["minimint::net::framed::FrameError"]}];
implementors["minimint_api"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"minimint_api/db/batch/struct.Accumulator.html\" title=\"struct minimint_api::db::batch::Accumulator\">Accumulator</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;T&gt;","synthetic":false,"types":["alloc::vec::Vec"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"minimint_api/db/enum.DecodingError.html\" title=\"enum minimint_api::db::DecodingError\">DecodingError</a>&gt; for ConflictableTransactionError&lt;<a class=\"enum\" href=\"minimint_api/db/enum.DecodingError.html\" title=\"enum minimint_api::db::DecodingError\">DecodingError</a>&gt;","synthetic":false,"types":["sled::transaction::ConflictableTransactionError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt; for <a class=\"enum\" href=\"minimint_api/db/enum.DatabaseError.html\" title=\"enum minimint_api::db::DatabaseError\">DatabaseError</a>","synthetic":false,"types":["minimint_api::db::DatabaseError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"minimint_api/db/enum.DecodingError.html\" title=\"enum minimint_api::db::DecodingError\">DecodingError</a>&gt; for <a class=\"enum\" href=\"minimint_api/db/enum.DatabaseError.html\" title=\"enum minimint_api::db::DatabaseError\">DatabaseError</a>","synthetic":false,"types":["minimint_api::db::DatabaseError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Hash&gt; for <a class=\"struct\" href=\"minimint_api/struct.TransactionId.html\" title=\"struct minimint_api::TransactionId\">TransactionId</a>","synthetic":false,"types":["minimint_api::TransactionId"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"minimint_api/struct.TransactionId.html\" title=\"struct minimint_api::TransactionId\">TransactionId</a>&gt; for Sha256","synthetic":false,"types":["bitcoin_hashes::sha256::Hash"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>&gt; for <a class=\"struct\" href=\"minimint_api/struct.PeerId.html\" title=\"struct minimint_api::PeerId\">PeerId</a>","synthetic":false,"types":["minimint_api::PeerId"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"minimint_api/struct.PeerId.html\" title=\"struct minimint_api::PeerId\">PeerId</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Amount&gt; for <a class=\"struct\" href=\"minimint_api/struct.Amount.html\" title=\"struct minimint_api::Amount\">Amount</a>","synthetic":false,"types":["minimint_api::Amount"]}];
implementors["minimint_ln"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Hash&gt; for <a class=\"struct\" href=\"minimint_ln/contracts/incoming/struct.OfferId.html\" title=\"struct minimint_ln::contracts::incoming::OfferId\">OfferId</a>","synthetic":false,"types":["minimint_ln::contracts::incoming::OfferId"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"minimint_ln/contracts/incoming/struct.OfferId.html\" title=\"struct minimint_ln::contracts::incoming::OfferId\">OfferId</a>&gt; for Sha256","synthetic":false,"types":["bitcoin_hashes::sha256::Hash"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Hash&gt; for <a class=\"struct\" href=\"minimint_ln/contracts/struct.ContractId.html\" title=\"struct minimint_ln::contracts::ContractId\">ContractId</a>","synthetic":false,"types":["minimint_ln::contracts::ContractId"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"minimint_ln/contracts/struct.ContractId.html\" title=\"struct minimint_ln::contracts::ContractId\">ContractId</a>&gt; for Sha256","synthetic":false,"types":["bitcoin_hashes::sha256::Hash"]}];
implementors["minimint_mint"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"minimint_mint/struct.SignRequest.html\" title=\"struct minimint_mint::SignRequest\">SignRequest</a>&gt; for <a class=\"struct\" href=\"minimint_mint/tiered/coins/struct.Coins.html\" title=\"struct minimint_mint::tiered::coins::Coins\">Coins</a>&lt;<a class=\"struct\" href=\"minimint_mint/struct.BlindToken.html\" title=\"struct minimint_mint::BlindToken\">BlindToken</a>&gt;","synthetic":false,"types":["minimint_mint::tiered::coins::Coins"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"minimint_mint/struct.InvalidAmountTierError.html\" title=\"struct minimint_mint::InvalidAmountTierError\">InvalidAmountTierError</a>&gt; for <a class=\"enum\" href=\"minimint_mint/enum.MintError.html\" title=\"enum minimint_mint::MintError\">MintError</a>","synthetic":false,"types":["minimint_mint::MintError"]}];
implementors["minimint_wallet"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"minimint_wallet/bitcoind/struct.BitcoinRpcError.html\" title=\"struct minimint_wallet::bitcoind::BitcoinRpcError\">BitcoinRpcError</a>","synthetic":false,"types":["minimint_wallet::bitcoind::BitcoinRpcError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"minimint_wallet/keys/struct.CompressedPublicKey.html\" title=\"struct minimint_wallet::keys::CompressedPublicKey\">CompressedPublicKey</a>&gt; for PublicKey","synthetic":false,"types":["bitcoin::util::ecdsa::PublicKey"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"minimint_wallet/bitcoind/struct.BitcoinRpcError.html\" title=\"struct minimint_wallet::bitcoind::BitcoinRpcError\">BitcoinRpcError</a>&gt; for <a class=\"enum\" href=\"minimint_wallet/enum.WalletError.html\" title=\"enum minimint_wallet::WalletError\">WalletError</a>","synthetic":false,"types":["minimint_wallet::WalletError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"minimint_wallet/txoproof/enum.PegInProofError.html\" title=\"enum minimint_wallet::txoproof::PegInProofError\">PegInProofError</a>&gt; for <a class=\"enum\" href=\"minimint_wallet/enum.WalletError.html\" title=\"enum minimint_wallet::WalletError\">WalletError</a>","synthetic":false,"types":["minimint_wallet::WalletError"]}];
implementors["mint_client"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/reqwest/0.11.10/reqwest/error/struct.Error.html\" title=\"struct reqwest::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"mint_client/api/enum.ApiError.html\" title=\"enum mint_client::api::ApiError\">ApiError</a>","synthetic":false,"types":["mint_client::api::ApiError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"mint_client/ln/enum.LnClientError.html\" title=\"enum mint_client::ln::LnClientError\">LnClientError</a>&gt; for <a class=\"enum\" href=\"mint_client/clients/gateway/enum.GatewayClientError.html\" title=\"enum mint_client::clients::gateway::GatewayClientError\">GatewayClientError</a>","synthetic":false,"types":["mint_client::clients::gateway::GatewayClientError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"mint_client/api/enum.ApiError.html\" title=\"enum mint_client::api::ApiError\">ApiError</a>&gt; for <a class=\"enum\" href=\"mint_client/clients/gateway/enum.GatewayClientError.html\" title=\"enum mint_client::clients::gateway::GatewayClientError\">GatewayClientError</a>","synthetic":false,"types":["mint_client::clients::gateway::GatewayClientError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"mint_client/mint/enum.MintClientError.html\" title=\"enum mint_client::mint::MintClientError\">MintClientError</a>&gt; for <a class=\"enum\" href=\"mint_client/clients/gateway/enum.GatewayClientError.html\" title=\"enum mint_client::clients::gateway::GatewayClientError\">GatewayClientError</a>","synthetic":false,"types":["mint_client::clients::gateway::GatewayClientError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"mint_client/api/enum.ApiError.html\" title=\"enum mint_client::api::ApiError\">ApiError</a>&gt; for <a class=\"enum\" href=\"mint_client/clients/user/enum.ClientError.html\" title=\"enum mint_client::clients::user::ClientError\">ClientError</a>","synthetic":false,"types":["mint_client::clients::user::ClientError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"mint_client/wallet/enum.WalletClientError.html\" title=\"enum mint_client::wallet::WalletClientError\">WalletClientError</a>&gt; for <a class=\"enum\" href=\"mint_client/clients/user/enum.ClientError.html\" title=\"enum mint_client::clients::user::ClientError\">ClientError</a>","synthetic":false,"types":["mint_client::clients::user::ClientError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"mint_client/mint/enum.MintClientError.html\" title=\"enum mint_client::mint::MintClientError\">MintClientError</a>&gt; for <a class=\"enum\" href=\"mint_client/clients/user/enum.ClientError.html\" title=\"enum mint_client::clients::user::ClientError\">ClientError</a>","synthetic":false,"types":["mint_client::clients::user::ClientError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"mint_client/ln/enum.LnClientError.html\" title=\"enum mint_client::ln::LnClientError\">LnClientError</a>&gt; for <a class=\"enum\" href=\"mint_client/clients/user/enum.ClientError.html\" title=\"enum mint_client::clients::user::ClientError\">ClientError</a>","synthetic":false,"types":["mint_client::clients::user::ClientError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;CreationError&gt; for <a class=\"enum\" href=\"mint_client/clients/user/enum.ClientError.html\" title=\"enum mint_client::clients::user::ClientError\">ClientError</a>","synthetic":false,"types":["mint_client::clients::user::ClientError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"mint_client/api/enum.ApiError.html\" title=\"enum mint_client::api::ApiError\">ApiError</a>&gt; for <a class=\"enum\" href=\"mint_client/mint/enum.MintClientError.html\" title=\"enum mint_client::mint::MintClientError\">MintClientError</a>","synthetic":false,"types":["mint_client::mint::MintClientError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"mint_client/mint/enum.CoinFinalizationError.html\" title=\"enum mint_client::mint::CoinFinalizationError\">CoinFinalizationError</a>&gt; for <a class=\"enum\" href=\"mint_client/mint/enum.MintClientError.html\" title=\"enum mint_client::mint::MintClientError\">MintClientError</a>","synthetic":false,"types":["mint_client::mint::MintClientError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"minimint_mint/struct.InvalidAmountTierError.html\" title=\"struct minimint_mint::InvalidAmountTierError\">InvalidAmountTierError</a>&gt; for <a class=\"enum\" href=\"mint_client/mint/enum.CoinFinalizationError.html\" title=\"enum mint_client::mint::CoinFinalizationError\">CoinFinalizationError</a>","synthetic":false,"types":["mint_client::mint::CoinFinalizationError"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()