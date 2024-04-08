import { upgrades } from 'hardhat'
import { readFileSync, writeFileSync } from 'fs'

import * as dotenv from 'dotenv'
dotenv.config({ path: './.env.dev' })

const SAFE_ADDRESS = process.env.SAFE_ADDRESS
const OWNER_ADDRESS = process.env.OWNER_ADDRESS
const ADMIN_LOG_PATH = process.env.ADMIN_LOG_PATH

async function main() {
  console.log('Transferring admin contract ownership to Safe multisig account...')
  await upgrades.admin.transferProxyAdminOwnership(SAFE_ADDRESS)
  console.log('Admin contract ownership transferred to Safe multisig account.')

  let multisigEnabledReceipt = {
    oldAddress: OWNER_ADDRESS,
    safeAddress: SAFE_ADDRESS,
    dateAndTime: new Date().toUTCString()
  }
  multisigEnabledReceipt = {
    multisigEnabledReceipt: multisigEnabledReceipt
  }

  console.log(multisigEnabledReceipt)
  const buffer = JSON.parse(readFileSync(ADMIN_LOG_PATH, 'utf8'))
  buffer.push(multisigEnabledReceipt)
  writeFileSync(ADMIN_LOG_PATH, JSON.stringify(buffer, null, 2), 'utf-8')
}

main().catch((error) => {
  console.error(error)
  process.exitCode = 1
})
