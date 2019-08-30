print("test blockchain");
list_blocks();
add_block("abcdefghijklmnopqrstuvwxyz78787777777777777777777777777777777777777");
list_blocks();
add_peer("0.0.0.0");
list_peers();



create_and_broadcast_transaction("AAA","BBB");
create_and_broadcast_transaction("ccc","ddd");
list_transaction_local();
add_block_from_local_transactions();
list_blocks();
