# Reference: https://betterprogramming.pub/building-an-api-gateway-in-rust-with-hyper-c84aaf549443

get_users:
	curl --location 'http://localhost:8080/users' \
    --header 'Authorization: Bearer JWT'

get_orders:
	curl --location 'http://localhost:8080/orders' \
    --header 'Authorization: Bearer JWT'