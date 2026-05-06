#!/bin/sh
echo "Running task at $(date)" >> /var/log/luci-app-giwifi
/usr/bin/giwifi login >> /var/log/luci-app-giwifi 2>&1
