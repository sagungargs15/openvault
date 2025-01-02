
## ASCII ART - Categorization

                                                    [Functional] Python Tests for Bitcoin
                                                                    |
                                                                    |
                        +-------------------------------------------+----------------------------------------
                        |                                           |                                    |   
                (feature, interface)                          (mempool, mining)                         (misc)
                        |                                              |                                 |
            +-------+--------------+                       +------+-----------+                      example
            |                      |                       |                   |
    Network & Protocol  Segwit & Taporoot           Transactions        Compatibility
        |                      |                          |                    |
    +---+---+              +---------+                  +---+---+         +-----------+
    |       |              |         |                  |        |        |            |
N&P1.py   N&P2.py        S&T1.py   S&T2.py             T1.py             C1.py


# Categories and Subcategories
* - example
    *   - General
* - feature
    *   - Network & Protocol
    *   - Segwit and Taproot
    *   - Binding & Network Configuration
    *   - Unsupported or Skipped
* - interface
    *   - CLI & HTTP
    *   - RPC
    *   - usdt
    *   - ZMQ
* - mempool
    *   - Transactions
    *   - Compatibility
* - mining
    *   - Basic Mining
* - p2p
    *   - Network & Peer Management
    *   - V2 Transport
* - rpc
    *   - Blockchain & Network
    *   - Transactions
    *   - Binding & Network Configuration
    *   - PSBT
    *   - Bans
    *   - Signer
* - wallet
    *   - Descriptors
    *   - Legacy Wallet
    *   - General
    *   - Transaction Cloning
    *   - Migration
* - tool
    *   - Signet Miner
    *   - Wallet Tools

This structure provides a more detailed organization, grouping files by both main categories and sub-categories where applicable.
Here's an organized list of the file names under both categories and sub-categories:

### example
**General**
    example_test.py

### feature
**Network & Protocol**
    feature_abortnode.py
    feature_addrman.py
    feature_anchors.py
    feature_asmap.py
    feature_assumevalid.py
    feature_block.py
    feature_blocksdir.py
    feature_blocksxor.py
    feature_cltv.py
    feature_coinstatsindex.py
    feature_config_args.py
    feature_csv_activation.py
    feature_dersig.py
    feature_dirsymlinks.py
    feature_discover.py
    feature_filelock.py
    feature_framework_miniwallet.py
    feature_framework_unit_tests.py
    feature_help.py
    feature_includeconf.py
    feature_init.py
    feature_loadblock.py
    feature_logging.py
    feature_maxtipage.py
    feature_maxuploadtarget.py
    feature_minchainwork.py
    feature_notifications.py
    feature_nulldummy.py
    feature_posix_fs_permissions.py
    feature_presegwit_node_upgrade.py
    feature_proxy.py
    feature_rbf.py
    feature_reindex.py
    feature_reindex_readonly.py
    feature_remove_pruned_files_on_startup.py
    feature_signet.py
    feature_startupnotify.py
    feature_uacomment.py
    feature_utxo_set_hash.py
    feature_versionbits_warning.py
**Segwit and Taproot**
    feature_assumeutxo.py
    feature_segwit.py --descriptors --v1transport
    feature_segwit.py --descriptors --v2transport
    feature_segwit.py --legacy-wallet
    feature_taproot.py
**Binding & Network Configuration**
    feature_bind_extra.py
    feature_bind_port_discover.py
    feature_bind_port_externalip.py
**Unsupported or Skipped**
    feature_unsupported_utxo_db.py

### interface
**CLI & HTTP**
    interface_bitcoin_cli.py --descriptors
    interface_bitcoin_cli.py --legacy-wallet
    interface_http.py
    interface_rest.py
**RPC**
    interface_rpc.py
    usdt
    interface_usdt_coinselection.py
    interface_usdt_mempool.py
    interface_usdt_net.py
    interface_usdt_utxocache.py
    interface_usdt_validation.py
**ZMQ**
    interface_zmq.py

### mempool
**Transactions**
    mempool_accept.py
    mempool_accept_wtxid.py
    mempool_datacarrier.py
    mempool_dust.py
    mempool_expiry.py
    mempool_limit.py
    mempool_package_limits.py
    mempool_package_onemore.py
    mempool_package_rbf.py
    mempool_packages.py
    mempool_persist.py --descriptors
    mempool_reorg.py
    mempool_resurrect.py
    mempool_sigoplimit.py
    mempool_spend_coinbase.py
    mempool_truc.py
    mempool_unbroadcast.py
    mempool_updatefromblock.py
    Compatibility
    mempool_compatibility.py

### mining
**Basic Mining**
    mining_basic.py
    mining_getblocktemplate_longpoll.py
    mining_prioritisetransaction.py

### p2p
**Network & Peer Management**
    p2p_1p1c_network.py
    p2p_add_connections.py
    p2p_addr_relay.py
    p2p_addrfetch.py
    p2p_addrv2_relay.py
    p2p_block_sync.py --v1transport
    p2p_block_sync.py --v2transport
    p2p_blockfilters.py
    p2p_blocksonly.py
    p2p_compactblocks.py
    p2p_compactblocks_blocksonly.py
    p2p_compactblocks_hb.py --v1transport
    p2p_compactblocks_hb.py --v2transport
    p2p_disconnect_ban.py --v1transport
    p2p_disconnect_ban.py --v2transport
    p2p_dns_seeds.py
    p2p_dos_header_tree.py
    p2p_eviction.py
    p2p_feefilter.py
    p2p_filter.py
    p2p_fingerprint.py
    p2p_getaddr_caching.py
    p2p_getdata.py
    p2p_handshake.py
    p2p_handshake.py --v2transport
    p2p_headers_sync_with_minchainwork.py
    p2p_i2p_ports.py
    p2p_i2p_sessions.py
    p2p_ibd_stalling.py --v1transport
    p2p_ibd_stalling.py --v2transport
    p2p_ibd_txrelay.py
    p2p_initial_headers_sync.py
    p2p_invalid_block.py --v1transport
    p2p_invalid_block.py --v2transport
    p2p_invalid_locator.py
    p2p_invalid_messages.py
    p2p_invalid_tx.py --v1transport
    p2p_invalid_tx.py --v2transport
    p2p_leak.py
    p2p_leak_tx.py --v1transport
    p2p_leak_tx.py --v2transport
    p2p_message_capture.py
    p2p_mutated_blocks.py
    p2p_net_deadlock.py --v1transport
    p2p_net_deadlock.py --v2transport
    p2p_nobloomfilter_messages.py
    p2p_node_network_limited.py --v1transport
    p2p_node_network_limited.py --v2transport
    p2p_opportunistic_1p1c.py
    p2p_orphan_handling.py
    p2p_outbound_eviction.py
    p2p_permissions.py
    p2p_ping.py
    p2p_seednode.py
    p2p_segwit.py
    p2p_sendheaders.py
    p2p_sendtxrcncl.py
    p2p_timeouts.py --v1transport
    p2p_timeouts.py --v2transport
    p2p_tx_download.py
    p2p_tx_privacy.py
    p2p_unrequested_blocks.py
**V2 Transport**
    p2p_v2_encrypted.py
    p2p_v2_misbehaving.py
    p2p_v2_transport.py

### rpc
**Blockchain & Network**
    rpc_blockchain.py --v1transport
    rpc_blockchain.py --v2transport
    rpc_net.py --v1transport
    rpc_net.py --v2transport
**Transactions**
    rpc_createmultisig.py
    rpc_decodescript.py
    rpc_deriveaddresses.py
    rpc_deriveaddresses.py --usecli
    rpc_dumptxoutset.py
    rpc_estimatefee.py
    rpc_generate.py
    rpc_getblockfilter.py
    rpc_getblockfrompeer.py
    rpc_getblockstats.py
    rpc_getchaintips.py
    rpc_getdescriptorinfo.py
    rpc_help.py
    rpc_invalid_address_message.py
    rpc_invalidateblock.py
    rpc_mempool_info.py
    rpc_misc.py
    rpc_named_arguments.py
    rpc_orphans.py
    rpc_packages.py
    rpc_preciousblock.py
    rpc_psbt.py --descriptors
    rpc_rawtransaction.py --legacy-wallet
    rpc_scanblocks.py
    rpc_scantxoutset.py
    rpc_signmessagewithprivkey.py
    rpc_signrawtransactionwithkey.py
    rpc_txoutproof.py
    rpc_uptime.py
    rpc_users.py
    rpc_validateaddress.py
    rpc_whitelist.py
**Binding & Network Configuration**
    rpc_bind.py --ipv4
    rpc_bind.py --ipv6
    rpc_bind.py --nonloopback
**PSBT**
    rpc_psbt.py --legacy-wallet
**Bans**
    rpc_setban.py --v1transport
    rpc_setban.py --v2transport
**Signer**
    rpc_signer.py

### wallet
**Descriptors**
    wallet_abandonconflict.py --descriptors
    wallet_address_types.py --descriptors
    wallet_assumeutxo.py --descriptors
    wallet_avoid_mixing_output_types.py --descriptors
    wallet_avoidreuse.py --descriptors
    wallet_backup.py --descriptors
    wallet_balance.py --descriptors
    wallet_basic.py --descriptors
    wallet_blank.py --descriptors
    wallet_bumpfee.py --descriptors
    wallet_change_address.py --descriptors
    wallet_coinbase_category.py --descriptors
    wallet_conflicts.py --descriptors
    wallet_create_tx.py --descriptors
    wallet_createwallet.py --descriptors
    wallet_createwallet.py --usecli
    wallet_createwalletdescriptor.py --descriptors
    wallet_descriptor.py --descriptors
    wallet_encryption.py --descriptors
    wallet_fallbackfee.py --descriptors
    wallet_fast_rescan.py --descriptors
    wallet_fundrawtransaction.py --descriptors
    wallet_gethdkeys.py --descriptors
    wallet_groups.py --descriptors
    wallet_hd.py --descriptors
    wallet_importdescriptors.py --descriptors
    wallet_importprunedfunds.py --descriptors
    wallet_keypool.py --descriptors
    wallet_keypool_topup.py --descriptors
    wallet_labels.py --descriptors
    wallet_listdescriptors.py --descriptors
    wallet_listreceivedby.py --descriptors
    wallet_listsinceblock.py --descriptors
    wallet_listtransactions.py --descriptors
    wallet_miniscript.py --descriptors
    wallet_multisig_descriptor_psbt.py --descriptors
    wallet_multiwallet.py --descriptors
    wallet_multiwallet.py --usecli
    wallet_reindex.py --descriptors
    wallet_rescan_unconfirmed.py --descriptors
    wallet_resendwallettransactions.py --descriptors
    wallet_send.py --descriptors
    wallet_sendall.py --descriptors
    wallet_sendmany.py --descriptors
    wallet_signer.py --descriptors
    wallet_signrawtransactionwithwallet.py --descriptors
    wallet_simulaterawtx.py --descriptors
    wallet_taproot.py --descriptors
    wallet_transactiontime_rescan.py --descriptors
    wallet_txn_doublespend.py --descriptors
    wallet_backwards_compatibility.py --descriptors
**Legacy Wallet**
    wallet_abandonconflict.py --legacy-wallet
    wallet_address_types.py --legacy-wallet
    wallet_avoidreuse.py --legacy-wallet
    wallet_backup.py --legacy-wallet
    wallet_balance.py --legacy-wallet
    wallet_basic.py --legacy-wallet
    wallet_blank.py --legacy-wallet
    wallet_bumpfee.py --legacy-wallet
    wallet_change_address.py --legacy-wallet
    wallet_coinbase_category.py --legacy-wallet
    wallet_conflicts.py --legacy-wallet
    wallet_create_tx.py --legacy-wallet
    wallet_createwallet.py --legacy-wallet
    wallet_dump.py --legacy-wallet
    wallet_encryption.py --legacy-wallet
    wallet_fallbackfee.py --legacy-wallet
    wallet_fundrawtransaction.py --legacy-wallet
    wallet_groups.py --legacy-wallet
    wallet_hd.py --legacy-wallet
    wallet_implicitsegwit.py --legacy-wallet
    wallet_import_rescan.py --legacy-wallet
    wallet_import_with_label.py --legacy-wallet
    wallet_importmulti.py --legacy-wallet
    wallet_importprunedfunds.py --legacy-wallet
    wallet_inactive_hdchains.py --legacy-wallet
    wallet_keypool.py --legacy-wallet
    wallet_keypool_topup.py --legacy-wallet
    wallet_labels.py --legacy-wallet
    wallet_listreceivedby.py --legacy-wallet
    wallet_listsinceblock.py --legacy-wallet
    wallet_listtransactions.py --legacy-wallet
    wallet_multiwallet.py --legacy-wallet
    wallet_reindex.py --legacy-wallet
    wallet_resendwallettransactions.py --legacy-wallet
    wallet_send.py --legacy-wallet
    wallet_sendall.py --legacy-wallet
    wallet_sendmany.py --legacy-wallet
    wallet_signrawtransactionwithwallet.py --legacy-wallet
    wallet_simulaterawtx.py --legacy-wallet
    wallet_transactiontime_rescan.py --legacy-wallet
    wallet_txn_doublespend.py --legacy-wallet
    wallet_upgradewallet.py --legacy-wallet
    wallet_watchonly.py --legacy-wallet
    wallet_watchonly.py --usecli --legacy-wallet
    wallet_backwards_compatibility.py --legacy-wallet
**General**
    wallet_crosschain.py
    wallet_disable.py
    wallet_orphanedreward.py
    wallet_reorgsrestore.py
    wallet_signmessagewithaddress.py
    wallet_spend_unconfirmed.py
    wallet_startup.py
    wallet_timelock.py
**Transaction Cloning**
    wallet_txn_clone.py
    wallet_txn_clone.py --mineblock
    wallet_txn_clone.py --segwit
**Migration**
    wallet_migration.py

### tool
**Signet Miner**
    tool_signet_miner.py --descriptors
    tool_signet_miner.py --legacy-wallet
**Wallet Tools**
    tool_wallet.py --descriptors
    tool_wallet.py --legacy-wallet
    tool_wallet.py --legacy-wallet --bdbro
    tool_wallet.py --legacy-wallet --bdbro --swap-bdb-endian

