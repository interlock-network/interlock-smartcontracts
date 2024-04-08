import { ethers, upgrades } from 'hardhat'
import { writeFileSync } from 'fs'

import * as dotenv from 'dotenv'
dotenv.config({ path: './.env.dev' })

const CONTRACT = process.env.CONTRACT
const ADMIN_LOG_PATH = process.env.ADMIN_LOG_PATH
const OWNER_ADDRESS = process.env.OWNER_ADDRESS

async function main() {
  const ilockmvp = await ethers.getContractFactory(CONTRACT)

  console.log('Deploying ilockmvp token contract...')
  const ilock = await upgrades.deployProxy(ilockmvp, [OWNER_ADDRESS], { initializer: 'initialize' })
  const response = await ilock.waitForDeployment()
  console.log('ilockmvp token contract deployed.')

  let deploymentReceipt = {
    contractNetwork: 'Arbitrum Mainnet',
    contractProxyAddress: response.target,
    dateAndTime: new Date().toUTCString()
  }
  deploymentReceipt = {
    deploymentReceipt: deploymentReceipt
  }

  console.log(deploymentReceipt)
  writeFileSync(ADMIN_LOG_PATH, JSON.stringify([deploymentReceipt], null, 2), 'utf-8')
}

main().catch((error) => {
  console.error(error)
  process.exitCode = 1
})
