import os
import time
import requests
import json
from web3 import Web3

# Environment variables
PRIVATE_KEY = os.getenv("MANTLE_PRIVATE_KEY", "0x1111111111111111111111111111111111111111111111111111111111111111")
MANTLE_RPC_URL = os.getenv("MANTLE_RPC_URL", "https://rpc.mantle.xyz")
AUDITOR_NODE_URL = "http://127.0.0.1:4002/verify_trade"

def fetch_market_data():
    # Mocking Bybit data
    return {"price": 1.05, "rsi": 25.0}

def main():
    print("[Execution Agent] Initializing...")
    w3 = Web3(Web3.HTTPProvider(MANTLE_RPC_URL))
    account = w3.eth.account.from_key(PRIVATE_KEY)
    print(f"[Execution Agent] Wallet: {account.address}")

    while True:
        data = fetch_market_data()
        payload = {"price": data["price"], "rsi": data["rsi"], "payment_tx_hash": None}
        
        print(f"\n[Execution Agent] Requesting Validation from Auditor Node...")
        response = requests.post(AUDITOR_NODE_URL, json=payload)
        
        if response.status_code == 402:
            res_json = response.json()
            fee = res_json.get("fee_usdc")
            payment_addr = res_json.get("payment_address")
            print(f"[Execution Agent] Received HTTP 402 Payment Required! Fee: {fee} USDC -> {payment_addr}")
            
            # Simulate Mantle L2 Transaction Settlement
            print("[Execution Agent] Settling invoice on Mantle L2...")
            time.sleep(2) # Network latency
            mock_tx_hash = "0xdeadbeef1234567890abcdef1234567890abcdef1234567890abcdef12345678"
            print(f"[Execution Agent] Payment Confirmed. TxHash: {mock_tx_hash}")
            
            # Retry with payment proof
            payload["payment_tx_hash"] = mock_tx_hash
            print("[Execution Agent] Resubmitting to Auditor with proof of payment...")
            response = requests.post(AUDITOR_NODE_URL, json=payload)

        if response.status_code == 200:
            res_json = response.json()
            score = res_json.get("score")
            decision = res_json.get("decision")
            print(f"[Execution Agent] Validation Successful! Score: {score}/100")
            print(f"[Execution Agent] Auditor WASM Result: Action={decision.get('action')}, Reason={decision.get('reason')}")
            
            if decision.get("action") in ["BUY", "SELL"]:
                print("[Execution Agent] Executing DEX Swap on Agni Finance...")
                # Here we would use the web3.py integration with ISwapRouter
                print("[Execution Agent] Swap Complete.\n")
        else:
            print(f"[Execution Agent] Validation Failed: {response.text}")
            
        time.sleep(15)

if __name__ == "__main__":
    main()
