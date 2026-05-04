// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "openzeppelin-contracts/contracts/token/ERC721/ERC721.sol";
import "openzeppelin-contracts/contracts/access/Ownable.sol";

/**
 * @title Sovereign ERC-8004 Agent Identity Registry
 * @dev Implementation of the ERC-8004 standard for Autonomous AI Agents.
 * Maps an agent's on-chain wallet to a persistent verifiable identity NFT.
 */
contract ERC8004Registry is ERC721, Ownable {
    uint256 private _nextTokenId;

    // Mapping from agent token ID to the controlling EOA/Contract
    mapping(uint256 => address) public agentControllers;

    // Mapping from agent token ID to reputation score (based on successful on-chain inferences)
    mapping(uint256 => uint256) public agentReputation;

    event AgentRegistered(uint256 indexed agentId, address indexed controller);
    event ReputationUpdated(uint256 indexed agentId, uint256 newReputation);

    constructor() ERC721("Trustless Agent Identity", "ERC8004") Ownable(msg.sender) {}

    /**
     * @notice Register a new AI Agent identity.
     * @param controller The address that will sign transactions for this agent.
     * @return agentId The newly minted ERC-8004 token ID.
     */
    function registerAgent(address controller) external returns (uint256 agentId) {
        agentId = ++_nextTokenId;
        _mint(controller, agentId);
        agentControllers[agentId] = controller;
        
        emit AgentRegistered(agentId, controller);
    }

    /**
     * @notice Update agent's reputation based on execution success.
     * @param agentId The ID of the agent.
     * @param scoreDelta The amount of reputation to add.
     */
    function addReputation(uint256 agentId, uint256 scoreDelta) external {
        // In a production environment, this should be restricted to authorized protocols.
        // For Hackathon MVP, we allow open reputation tracking to demonstrate on-chain metrics.
        require(_ownerOf(agentId) != address(0), "Agent does not exist");
        
        agentReputation[agentId] += scoreDelta;
        emit ReputationUpdated(agentId, agentReputation[agentId]);
    }
}
