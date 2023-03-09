#!/bin/bash

# debug output
set -x

# deploy and get token contract address
ADDRESS=$(

	# answer dry-run prompt
	yes |

	# instantiate token contract with salt
	cargo contract instantiate --constructor new_token --manifest-path=../../ilockmvp/Cargo.toml --suri //Alice --salt $(date +%s) |

	# find output line with token contract address
	grep "Contract " |

	# get address string
	awk -F " " '{
		print $2;
	}';

# assign address string to $ADDRESS
)





