# Reference: https://betterprogramming.pub/building-an-api-gateway-in-rust-with-hyper-c84aaf549443
# Badge: https://docs.github.com/en/actions/monitoring-and-troubleshooting-workflows/adding-a-workflow-status-badge

get_users:
	curl --location 'http://localhost:8080/users' \
    --header 'Authorization: Bearer JWT'

get_orders:
	curl --location 'http://localhost:8080/orders' \
    --header 'Authorization: Bearer JWT'