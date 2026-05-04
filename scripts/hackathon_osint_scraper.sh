#!/bin/bash
echo "[*] Searching GitHub for ERC-8004 implementations..."
gh search repos "ERC-8004" --limit 5 --json name,owner,url,description | jq .

echo "[*] Searching GitHub for RealClaw Mantle..."
gh search repos "RealClaw" "Mantle" --limit 5 --json name,owner,url,description | jq .

echo "[*] Searching GitHub for Byreal Agent Skills..."
gh search repos "Byreal Agent Skills" --limit 5 --json name,owner,url,description | jq .
