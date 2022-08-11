        set -eu
        fm_cfg=./var/lib/minimint
        fm_bin=./result/bin
        cargo run --bin configgen $fm_cfg 1 4000 5000 1 10 100 1000 10000 100000 1000000

        btc_rpc_address="127.0.0.1:8333"
        btc_rpc_user="bitcoin"
        btc_rpc_pass="bitcoin"
        fm_tmp_config="$(mktemp -d)/config.json"

        echo "Writing tmp config to $fm_tmp_config"
        cat $fm_cfg/client.json | jq ".wallet.btc_rpc_address=\"$btc_rpc_address\"" \
        | jq ".wallet.btc_rpc_user=\"$btc_rpc_user\"" \
        | jq ".wallet.btc_rpc_pass=\"$btc_rpc_pass\"" > $fm_tmp_config

        $fm_bin/ln_gateway $fm_cfg/server-0.json &
