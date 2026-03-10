#!/bin/bash
# auto_generate.sh - Continuous generation with safety checks

# Configuration
MIN_FREE_GB=1
TARGET_DIR="."
# Calculate threads: max CPU cores minus 1 (minimum 1 thread)
CPU_CORES=$(nproc)
THREADS=$((CPU_CORES > 1 ? CPU_CORES - 1 : 1))

echo "Starting autonomous generation..."
echo "System Cores: $CPU_CORES | Using Threads: $THREADS"
echo "Safety Limit: Stop if free space < ${MIN_FREE_GB}GB"
echo "------------------------------------------"

while true; do
    # Check free space in GB on the partition containing TARGET_DIR
    FREE_SPACE_KB=$(df -k "$TARGET_DIR" | awk 'NR==2 {print $4}')
    FREE_SPACE_GB=$((FREE_SPACE_KB / 1024 / 1024))

    if [ "$FREE_SPACE_GB" -lt "$MIN_FREE_GB" ]; then
        echo "CRITICAL: Low disk space ($FREE_SPACE_GB GB remaining). Stopping."
        break
    fi

    echo "[$(date '+%Y-%m-%d %H:%M:%S')] Generating batch of 5000 addresses..."
    
    # Run the generator
    # We use a batch size of 5000 to check disk space frequently
    cargo run --release -- --count 5000 --threads "$THREADS" --format segwit --network bitcoin
    
    if [ $? -ne 0 ]; then
        echo "Error detected during execution. Restarting in 5 seconds..."
        sleep 5
    fi
done
