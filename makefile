add:
	cargo run -- add "0xAC234d27D17D02C37B17E7B918f66b6c8Cb50a5D" "0xCfC5431d4d9844E34390d53f82964C0Ee9327b6B" "0x1925c0cee880b44Ee14E425D14a0dE122966A3dC"

show:
	cargo run -- show

remove:
	cargo run -- remove

balance:
	cargo run -- balance

set_bet_amount:
	cargo run -- set_bet_amount 1

provide_outcome:
	cargo run  -- provide_outcome 0x524f8b01f86125dbc5fa43c1dba2495410c63906bcda01430883ccb28e748a2b
