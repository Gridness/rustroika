#!/bin/bash

echo Initializing container...

mkdir ~/.config/rustroika
cat >> ~/.config/rustroika/config.yaml << EOF
defaults:
    trips-per-week: null
    monthly-cost: null
    ticket-price: null
EOF

rustroika

exec "$@"
