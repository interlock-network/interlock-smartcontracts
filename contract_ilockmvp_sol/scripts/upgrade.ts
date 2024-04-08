import { ethers, upgrades, defender } from 'hardhat'
import { readFileSync, writeFileSync } from 'fs'

import * as dotenv from 'dotenv'
dotenv.config({ path: './.env.dev' })

const PROXY_ADDRESS = process.env.PROXY_ADDRESS
const CONTRACT_UPGRADE = process.env.CONTRACT_UPGRADE
const ADMIN_LOG_PATH = process.env.ADMIN_LOG_PATH

async function main() {
  const ILOCKV2 = await ethers.getContractFactory(CONTRACT_UPGRADE)

  console.log('Upgrading ILOCK token contract...')
  //const response = await defender.proposeUpgradeWithApproval(PROXY_ADDRESS, ILOCKV2);
  const response = await upgrades.upgradeProxy(PROXY_ADDRESS, ILOCKV2)
  console.log('ILOCK token contract upgrade proposed.')

  let upgradeReceipt = {
    contractVersion: CONTRACT_UPGRADE,
    hash: response.deployTransaction.hash,
    blockHash: response.deployTransaction.blockHash,
    dateAndTime: new Date().toUTCString()
  }
  upgradeReceipt = {
    upgradeReceipt: upgradeReceipt
  }

  console.log(upgradeReceipt)
  const buffer = JSON.parse(readFileSync(ADMIN_LOG_PATH, 'utf8'))
  buffer.push(upgradeReceipt)
  writeFileSync(ADMIN_LOG_PATH, JSON.stringify(buffer, null, 2), 'utf-8')
}

main().catch((error) => {
  console.error(error)
  process.exitCode = 1
})
