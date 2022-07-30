# rust_blockchain_server
To execute : 

`bash start.sh`

or 

`cd console/`

`cargo build && cargo run`

-----------------------------------------------------

Commands :

`start_node` The console will start the blockchain and grant access to the node in the form of a Rocket.rs API in a new `gnome-terminal`.

`create_account [id] [initial_balance]` Creates an account with given id and initial balance.

`tranfer [id] [id] [funds]` Transfers funds from an account to another.

`balance [id]` Reads balance and displays it on the interactive console.

------------------------------------------------------

Note :
This project requires gnome-terminal to open a new console. 
It is installed by default on most linux distributions, it might not work on operating systems such as Windows or MacOs.
